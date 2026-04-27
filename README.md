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

## Running on Windows

There are two ways to run the solver on Windows: using the provided pre-built binary (no setup required), or building from source with Rust.

---

### Option A — Pre-built Binary (Recommended)

No installation needed. The `.exe` is already compiled and ready to run.

1. Download `eight_puzzle_problem.exe` and `input.yml` and place them in the **same folder**.
2. Open that folder in **Command Prompt** or **PowerShell**.
3. Run the solver:
```powershell
.\eight_puzzle_problem.exe
```

With options:
```powershell
# Print the full move-by-move solution
.\eight_puzzle_problem.exe --print-moves

# Use a different input file
.\eight_puzzle_problem.exe --input my-puzzles.yml
```

---

### Option B — Build from Source (Requires Rust)

Follow these steps if you want to modify and recompile the code.

#### 1. Install Rust

Go to **[https://rustup.rs](https://rustup.rs)** and download `rustup-init.exe`. Run it and follow the prompts — the default options are fine.

#### 2. Verify the installation

```powershell
rustc --version
cargo --version
```

Both commands should print a version number (e.g. `rustc 1.77.0`). If they return "not recognised", close and reopen your terminal.

#### 3. Clone or download the project

If you have Git:
```powershell
git clone https://github.com/rose-crisostomo/eight-puzzle-problem.git
cd eight-puzzle-problem
```

Or download the ZIP from GitHub → **Code → Download ZIP**, then extract it and `cd` into the folder.

#### 4. Build and run

```powershell
# Run directly via cargo
cargo run --release
```

---

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
