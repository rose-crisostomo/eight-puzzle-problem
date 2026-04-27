use std::collections::{HashSet, VecDeque};
use std::error::Error;
extern crate serde_yaml;
use serde::Deserialize;
use clap::Parser;

mod tests;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    print_moves: bool,
    #[arg(short, long, default_value = "input.yml")]
    input: String,
}

#[derive(Clone, Copy)]
enum Actions {
    LEFT,
    RIGHT,
    DOWN,
    UP
}

type Board = [[u8; 3]; 3];
type Coordinates = (usize, usize);

#[derive(Clone, Copy, Default)]
struct State {
    board: Board,
    cursor: Coordinates,
    action: Option<Actions>,
    parent: usize, // index
    depth: u32,
}

#[derive(Debug, Deserialize)]
struct PuzzleConfig {
    initial_state: Board,
    goal_state: Board,
    dfs_limit: u32,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    cases: Vec<PuzzleConfig>,
}

#[derive(Default)]
struct SearchResult {
    history: Vec<State>,
    last_state: Option<State>,
    error_msg: Option<String>,
}

#[derive(Default)]
struct CaseResults {
    dfs: SearchResult,
    bfs: SearchResult,
    initial_state: Board,
    goal_state: Board,
    print_moves: bool,
    dfs_limit: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let path = args.input;
    let f = std::fs::File::open(&path).map_err(|e| {
        format!("Failed to open config at {}: {}", path, e)
    })?;
    let app_config: AppConfig = serde_yaml::from_reader(f)?;
    let mut final_results: Vec<CaseResults> = vec![];

    for puzzle in app_config.cases {
        println!("-------------------------------------");

        let board = puzzle.initial_state;
        let goal = puzzle.goal_state;
        let mut history: Vec<State> = vec![];

        let mut case_results: CaseResults = Default::default();
        case_results.initial_state = board;
        case_results.goal_state = goal;
        case_results.print_moves = args.print_moves;

        let cursor = find_cursor(&board);
        if let Ok(cursor) = cursor {
            let limit = puzzle.dfs_limit;
            case_results.dfs_limit = limit;
            let last_state = dfs_search(
                goal,
                State { board, cursor, action: None, parent: 0, depth: 0 },
                &mut history,
                limit
            );
            match last_state {
                Ok(last_state) => case_results.dfs = SearchResult { history, last_state: Some(last_state), error_msg: None },
                Err(e) => case_results.dfs = SearchResult { history, last_state: None, error_msg: Some(e.to_string()) },
            }

            history = vec![]; // reset
            let last_state = bfs_search(
                goal,
                State { board, cursor, action: None, parent: 0, depth: 0 },
                &mut history
            );
            match last_state {
                Ok(last_state) => case_results.bfs = SearchResult { history, last_state: Some(last_state), error_msg: None },
                Err(e) => case_results.bfs = SearchResult { history, last_state: None, error_msg: Some(e.to_string()) },
            }
        }

        final_results.push(case_results);
    }

    print_results(final_results);

    Ok(())
}

fn find_cursor(board: &Board) -> Result<Coordinates, Box<dyn Error>> {
    for (r, row) in board.iter().enumerate() {
        for (c, &val) in row.iter().enumerate() {
            if val == 0 {
                return Ok((r, c));
            }
        }
    }
    Err("No blank tile (0) found in board".into())
}

fn print_results(results: Vec<CaseResults>) {
    let line = "-".repeat(44);
    for (id, result) in results.iter().enumerate() {
        let dfs = &result.dfs;
        let bfs = &result.bfs;
        let initial_state = result.initial_state;
        let goal_state = result.goal_state;

        println!("\n[Case #{}] {}", id + 1, &line[..33]);
        println!("Initial:");
        print_board(&initial_state);
        println!("Goal:");
        print_board(&goal_state);
        println!();

        let bfs_moves = get_num_moves(&bfs.history, &bfs.last_state, result.print_moves);
        let dfs_moves = get_num_moves(&dfs.history, &dfs.last_state, result.print_moves);
        if bfs_moves > 0 {
            println!("  > BFS Result: {} moves (Optimal)", bfs_moves);
        }
        else if let Some(msg) = &bfs.error_msg {
            println!("  > BFS Result: {}", msg);
        }

        if dfs_moves > 0 {
            println!("  > DFS Result: {} moves (Limit: {})", dfs_moves, result.dfs_limit);
        }
        else if let Some(msg) = &dfs.error_msg {
            println!("  > DFS Result: {}", msg);
        }
        println!("{}", line);
    }
}

fn get_num_moves(history: &Vec<State>, last_state: &Option<State>, should_print_moves: bool) -> usize {
    if let Some(last_state) = last_state {
        let mut states: Vec<State> = vec![];
        let mut curr = last_state;
        while curr.parent != 0 {
            states.push(*curr);
            curr = &history[curr.parent];
        }
        states.push(*curr);

        if should_print_moves {
            print_moves(&mut states);
        }

        states.len()
    }
    else {
        0
    }

}

fn print_moves(states: &mut Vec<State>) {
    states.reverse();
    for (i, st) in states.iter().enumerate() {
        println!("[Step {}] Action: {}", i + 1, get_action_str(&st.action));
        print_board(&st.board);
    }
}

fn bfs_search(goal: Board, init_state: State, history: &mut Vec<State>) -> Result<State, Box<dyn std::error::Error>>{
    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(init_state);
    let mut visited: HashSet<Board> = HashSet::new();

    while !queue.is_empty() {
        if let Some(s) = queue.pop_front() {
            let parent_index = history.len();
            history.push(s);

            let child_states = expand(s.board, s.cursor, parent_index, s.depth);
            for cs in child_states {
                if is_goal_state(goal, cs.board) {
                    return Ok(cs);
                }
                else if !visited.insert(cs.board) {
                    continue;
                }
                queue.push_back(cs);
            }
        }

    }
    Err("Failed to find solution".into())
}

fn dfs_search(goal: Board, init_state: State, history: &mut Vec<State>, limit: u32) -> Result<State, Box<dyn std::error::Error>>{
    let mut frontier: VecDeque<State> = VecDeque::new();
    frontier.push_back(init_state);
    let mut visited: HashSet<Board> = HashSet::new();
    while !frontier.is_empty() {
        if let Some(s) = frontier.pop_back() {
            let parent_index = history.len();
            history.push(s);

            if is_goal_state(goal, s.board) {
                return Ok(s);
            }
            else if s.depth == limit {
                return Err("Cutoff".into());
            }
            else if !visited.insert(s.board) { // check cycle
                continue;
            }

            let mut child_states = expand(s.board, s.cursor, parent_index, s.depth);
            child_states.reverse();
            for cs in child_states {
                frontier.push_back(cs);
            }
        }

    }
    Err("Failed to find solution".into())
}

fn print_board(board: &Board) {
    let horizontal_edge = "+---+---+---+";

    println!("{}", horizontal_edge);
    for row in board {
        print!("| ");
        for tile in row {
            let display = if *tile == 0 { " ".to_string() } else { tile.to_string() };
            print!("{}", display);

            print!(" | ");
        }
        println!("\n{}", horizontal_edge);
    }
}

fn get_action_str(action: &Option<Actions>) -> &str {
    if let Some(a) = action {
        match a {
            &Actions::LEFT => "←",
            &Actions::RIGHT => "→",
            &Actions::DOWN => "↓",
            &Actions::UP => "↑"
        }
    }
    else {
        ""
    }
}

fn expand(board: Board, cursor: Coordinates, parent_index: usize, depth: u32) -> Vec<State> {
    let mut child_states: Vec<State> = vec![];
    for action in [Actions::LEFT, Actions::RIGHT, Actions::DOWN, Actions::UP] {
        let child_state = move_cursor(board, cursor, action, parent_index, depth + 1);
        if let Some(child_state) = child_state  {
            child_states.push(child_state);
        }
    }
    child_states
}

fn move_cursor(board: Board, cursor: Coordinates, action: Actions, parent_index: usize, depth: u32) -> Option<State> {
    match action {
        Actions::LEFT => {
            if cursor.1 > 0 {
                let left: Coordinates = (cursor.0, cursor.1 - 1);
                let mut new_board = board;
                swap(&mut new_board, left, cursor);
                Option::from(State {
                    board: new_board,
                    cursor: left,
                    action: Option::from(action),
                    parent: parent_index,
                    depth: depth
                })
            }
            else {
                None
            }
        }
        Actions::RIGHT => {
            if cursor.1 < board[0].len() - 1 {
                let right: Coordinates = (cursor.0, cursor.1 + 1);
                let mut new_board = board;
                swap(&mut new_board, right, cursor);
                Option::from(State {
                    board: new_board,
                    cursor: right,
                    action: Option::from(action),
                    parent: parent_index,
                    depth: depth
                })
            }
            else {
                None
            }
        }
        Actions::DOWN => {
            if cursor.0 < board.len() - 1 {
                let down: Coordinates = (cursor.0 + 1, cursor.1);
                let mut new_board = board;
                swap(&mut new_board, down, cursor);
                Option::from(State {
                    board: new_board,
                    cursor: down,
                    action: Option::from(action),
                    parent: parent_index,
                    depth: depth
                })
            }
            else {
                None
            }
        }
        Actions::UP => {
            if cursor.0 > 0 {
                let up: Coordinates = (cursor.0 - 1, cursor.1);
                let mut new_board = board;
                swap(&mut new_board, up, cursor);
                Option::from(State {
                    board: new_board,
                    cursor: up,
                    action: Option::from(action),
                    parent: parent_index,
                    depth: depth
                })
            }
            else {
                None
            }
        }
    }
}

fn swap(board: &mut Board, a: Coordinates, b: Coordinates) {
    let a_value = board[a.0][a.1];
    board[a.0][a.1] = board[b.0][b.1];
    board[b.0][b.1] = a_value;
}

fn is_goal_state(goal: Board, board: Board) -> bool {
    goal == board
}