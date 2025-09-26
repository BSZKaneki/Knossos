
---

```markdown
# Maze Pathfinding Visualization in Rust

This project is a dynamic and extensible platform for generating random mazes and visualizing various pathfinding algorithms. It's built to be easily configurable, allowing you to run and compare any number of algorithms in sequence.

The real-time visualization is powered by [minifb](https://github.com/emoon/rust-minifb), so you can watch each algorithm explore the maze and find a path step by step.

## ✨ Features

-   **Maze Generation**
    -   **Perfect mazes** (no loops) using an iterative implementation of Depth-First Search.
    -   **Imperfect mazes** (with loops) by randomly removing walls after generation.

-   **Dynamic Pathfinding System**
    -   Run and visualize a custom sequence of algorithms (e.g., just BFS, or DFS then BFS).
    -   Comes with **BFS** (always finds the shortest path) and **DFS** included.
    -   **Highly Extensible**: Add new algorithms like A* or Dijkstra's in just 3 simple steps without changing the main simulation logic.

-   **Real-Time Visualization**
    -   Smooth step-by-step animation of the search process.
    -   Start (green) and end (red) points are clearly marked.
    -   Each algorithm's search pattern and final path are drawn in distinct, configurable colors.
    -   A final summary screen displays comparative stats (steps, time, path length) for all executed algorithms.

-   **Unit Tests** to ensure the correctness of maze generation and pathfinding logic.

## 📸 Screenshots

| BFS Exploration                                | DFS Exploration                                | Stats Screen                                   |
| ---------------------------------------------- | ---------------------------------------------- | ---------------------------------------------- |
| ![bfs animation](docs/bfs.gif) | ![dfs animation](docs/dfs.gif) | ![final stats screen](docs/stats.png) |

## 🚀 Getting Started

### 1. Clone the repo

```bash
git clone https://github.com/BSZKaneki/Knossos
cd Knossos
```

### 2. Run the project

The simulation will run with the default configuration (BFS followed by DFS).
```bash
cargo run --release
```
*(Using the `--release` flag is highly recommended for smooth performance.)*

### 3. Run tests

```bash
cargo test
```

## ⚙️ Configuration & Customization

You can easily control which algorithms run and in what order directly in `main.rs`. The primary configuration point is the `algorithms_to_run` vector.

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

        ..Default::default()    };

    let mut simulation = Simulation::new(&config);
    simulation.run();
}
```

## 🧩 Adding a New Algorithm

The project is designed for easy expansion. To add a new pathfinding algorithm (e.g., A*), follow these three steps:

1.  **Add an identifier** to the `Algorithm` enum in `main.rs`:
    ```rust
    pub enum Algorithm { Bfs, Dfs, AStar }
    ```

2.  **Register the algorithm's info** in the `get_algorithm_info` function. This tells the simulation its name, colors, and which function to call.
    ```rust
    // In get_algorithm_info()
    match algo {
        // ...
        Algorithm::AStar => AlgorithmInfo {
            name: "A*",
            function: Maze::path_finding_a_star, // The function you will create
            search_color: 0xAAFF8C00, // Orange
            path_color: 0xAA00FA9A,   // Sea green
        },
    }
    ```

3.  **Implement the algorithm's logic** as a new function within `impl Maze`.
    ```rust
    // In impl Maze
    fn path_finding_a_star(&self) -> (usize, u128, Vec<(usize, usize)>, Vec<(usize, usize)>) {
        // Your A* implementation here...
    }
    ```

That's it! The simulation and visualization loops will automatically handle the new algorithm.
```