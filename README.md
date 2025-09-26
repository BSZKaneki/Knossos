
````markdown
# 🌀 Maze Pathfinding Visualization in Rust

A dynamic and extensible platform for **generating random mazes** and **visualizing pathfinding algorithms** in real time.  
Built with [minifb](https://github.com/emoon/rust-minifb) for smooth, step-by-step animations.

---

## ✨ Features

### 🏗 Maze Generation
- **Perfect mazes** (no loops) via iterative Depth-First Search.
- **Imperfect mazes** (with loops) by randomly removing walls post-generation.

### 🔍 Pathfinding System
- Run and visualize **any sequence of algorithms** (e.g., BFS only, or DFS → BFS).
- Includes **BFS** (shortest path guaranteed) and **DFS** out of the box.
- **Highly extensible**: Add new algorithms like A* or Dijkstra’s in just **3 steps**—no core changes required.

### 🎨 Real-Time Visualization
- Smooth, step-by-step animation of the search process.
- Start (🟩) and end (🟥) points clearly marked.
- Distinct, customizable colors for each algorithm’s exploration and path.
- Final **summary screen** compares:
  - Steps taken
  - Execution time
  - Path length

### ✅ Testing
- Unit tests ensure maze generation and pathfinding correctness.

---

## 📸 Screenshots

| BFS Exploration | DFS Exploration | Stats Screen |
|-----------------|-----------------|--------------|
| ![bfs animation](docs/bfs.gif) | ![dfs animation](docs/dfs.gif) | ![final stats screen](docs/stats.png) |

---

## 🚀 Getting Started

### 1️⃣ Clone the repo
```bash
git clone https://github.com/BSZKaneki/Knossos
cd Knossos
````

### 2️⃣ Run the project

Run the simulation with the default config (**BFS → DFS**):

```bash
cargo run --release
```

> 💡 Use the `--release` flag for smooth performance.

### 3️⃣ Run tests

```bash
cargo test
```

---

## ⚙️ Configuration & Customization

Control which algorithms run and in what order via `main.rs`.

Example configurations:

```rust
// src/main.rs

fn main() {
    let config = Config {
        // --- EDIT THIS LIST ---

        // Example 1: Run only BFS
        // algorithms_to_run: vec![Algorithm::Bfs],

        // Example 2: Run only DFS
        // algorithms_to_run: vec![Algorithm::Dfs],

        // Example 3: Compare both (default)
        algorithms_to_run: vec![Algorithm::Bfs, Algorithm::Dfs],

        ..Default::default()
    };

    let mut simulation = Simulation::new(&config);
    simulation.run();
}
```

---

## 🧩 Adding a New Algorithm

Adding a new pathfinding algorithm (e.g., **A*** ) is straightforward:

1. **Add to the enum** in `main.rs`:

   ```rust
   pub enum Algorithm {
       Bfs,
       Dfs,
       AStar,
   }
   ```

2. **Register its info** in `get_algorithm_info`:

   ```rust
   match algo {
       // ...
       Algorithm::AStar => AlgorithmInfo {
           name: "A*",
           function: Maze::path_finding_a_star, // Your new function
           search_color: 0xAAFF8C00, // Orange
           path_color: 0xAA00FA9A,   // Sea green
       },
   }
   ```

3. **Implement the algorithm** inside `impl Maze`:

   ```rust
   impl Maze {
       fn path_finding_a_star(
           &self
       ) -> (usize, u128, Vec<(usize, usize)>, Vec<(usize, usize)>) {
           // Your A* implementation...
       }
   }
   ```

---


