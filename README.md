# 8-Puzzle Solver

A Rust implementation of the classic 8-puzzle problem using **Breadth-First Search (BFS)** and **Depth-First Search (DFS)** with a configurable depth limit.

---

## Problem

The 8-puzzle is a 3×3 sliding tile board with tiles numbered 1 - 8 and one blank space (`0`). The goal is to find a sequence of moves that transforms an initial board configuration into a goal state by sliding adjacent tiles into the blank.

```
Initial:          Goal:
+---+---+---+    +---+---+---+
| 1 | 2 | 3 |    | 1 | 2 | 3 |
+---+---+---+    +---+---+---+
| 4 |   | 5 |    | 4 | 5 | 6 |
+---+---+---+    +---+---+---+
| 7 | 8 | 6 |    | 7 | 8 |   |
+---+---+---+    +---+---+---+
```

---

## Algorithms

### Breadth-First Search (BFS)
- Explores all states level by level
- **Guaranteed to find the optimal (shortest) solution**
- Uses a queue (FIFO) and a visited set to avoid cycles
- May use significant memory for deep solutions
- Returns `Failed to find solution` if the puzzle is provably unsolvable

### Depth-First Search (DFS) with Limit
- Explores deeply along a single path before backtracking
- Uses a stack (LIFO) and a configurable **depth limit**
- **Not guaranteed to find the optimal solution**: DFS may find a longer path depending on the order of expansion
- Returns `Cutoff` if no solution is found within the depth limit
- Returns `Failed to find solution` if the puzzle is provably unsolvable

---

## Project Structure

```
.
├── src/
│   ├── main.rs       # Entry point, search loop, result display
│   └── tests.rs      # Unit tests
├── input.yml         # Puzzle configurations
├── Cargo.toml
└── README.md
```

---

## Configuration (`input.yml`)

```yaml
cases:
  - dfs_limit: 30
    initial_state:
      - [1, 2, 3]
      - [4, 0, 5]
      - [7, 8, 6]
    goal_state:
      - [1, 2, 3]
      - [4, 5, 6]
      - [7, 8, 0]

  - dfs_limit: 5000
    initial_state:
      - [0, 1, 3]
      - [4, 2, 5]
      - [7, 8, 6]
    goal_state:
      - [1, 2, 3]
      - [4, 5, 6]
      - [7, 8, 0]

  - dfs_limit: 5000
    initial_state:
      - [ 1, 2, 3 ]
      - [ 4, 5, 6 ]
      - [ 8, 7, 0 ]
    goal_state:
      - [ 1, 2, 3 ]
      - [ 4, 5, 6 ]
      - [ 7, 8, 0 ]
```

Each case has its own `dfs_limit` to tune how deep DFS is allowed to search.

---

## Usage

### Build

```bash
cargo build --release
```

### Run

```bash
# Default input file (input.yml)
cargo run

# Custom input file
cargo run -- --input custom.yml

# Print full move sequence
cargo run -- --print-moves
```

### CLI Options

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--input` | `-i` | `input.yml` | Path to the YAML config file |
| `--print-moves` | `-p` | false | Print the full move-by-move solution |

---

## Sample Output

```
[Case #1] ---------------------------------
Initial:
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
| 4 |   | 5 |
+---+---+---+
| 7 | 8 | 6 |
+---+---+---+

Goal:
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 7 | 8 |   |
+---+---+---+

  > BFS Result: 2 moves (Optimal)
  > DFS Result: 28 moves (Limit: 30)
--------------------------------------------

[Case #3] ---------------------------------
  > BFS Result: Failed to find solution
  > DFS Result: Cutoff
--------------------------------------------
```

---

## Key Implementation Notes

- **Cursor tracking**: The blank tile's position (`0`) is tracked as a `cursor: Coordinates` on each `State` to avoid scanning the board on every move.
- **Depth field**: Each `State` carries a `depth: u32` field incremented at expansion time. This is O(1) and is used for the DFS limit check.
- **Cycle detection**: Both BFS and DFS use a `HashSet<Board>` of visited boards.
- **Parent tracing**: Each state stores the index of its parent in the `history` Vec, allowing the solution path to be reconstructed after the search.
- **Unsolvable detection**: BFS exhausts all reachable states and reports failure. DFS reports `Cutoff` when the depth limit is reached before a solution is found.

---

## Dependencies

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
clap = { version = "4", features = ["derive"] }
```
