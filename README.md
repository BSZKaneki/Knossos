# Maze Pathfinding Visualization in Rust

This project generates random mazes and visualizes pathfinding algorithms, currently supporting Breadth-First Search (BFS) and Depth-First Search (DFS) by default. It features a modular design that allows easy addition of new algorithms and configurable sequences for running them. Real-time visualization is powered by [minifb](https://github.com/emoon/rust-minifb), enabling you to watch algorithms explore the maze step by step, with a final stats screen comparing all results.

## Features
- Maze Generation  
  - Perfect mazes (no loops) using recursive backtracking.  
  - Imperfect mazes (with loops) by removing random walls.  

- Pathfinding Algorithms  
  - Modular support for multiple algorithms, run in a configurable sequence.  
  - BFS → always finds the shortest path.  
  - DFS → finds a valid path, not guaranteed to be the shortest.  
  - Easily extensible: Add new algorithms by updating the `Algorithm` enum and `get_algorithm_info` function.

- Visualization  
  - Smooth animation with configurable batch size.  
  - Start and end points clearly marked.  
  - Paths drawn in different colors for easy comparison.  
  - Sequential visualization: Each algorithm's exploration and pathfinding is shown one after another.  
  - Final stats screen displaying steps, time, and path length for all algorithms.  

- Unit Tests to ensure correctness of maze generation and pathfinding.  

## Screenshots

| BFS Exploration | DFS Exploration | Stats Screen |
|-----------------|-----------------|--------------|
| ![bfs](docs/bfs.gif) | ![dfs](docs/dfs.gif) | ![stats](docs/stats.png) |

## Getting Started

### 1. Clone the repo
```bash
git clone https://github.com/BSZKaneki/Knossos
cd Knossos
```

### 2. Run the repo 
```bash
cargo run
```

### 3. Run tests
```bash
cargo test
```

## Customization
The simulation is configurable via the `Config` struct in `main()`. Key options include:
- `use_perfect_maze`: Set to `true` for perfect mazes (no loops) or `false` for imperfect mazes (with loops).
- `skip_visualization`: Set to `true` to skip animations and only compute results.
- `maze_width` and `maze_height`: Dimensions of the maze.
- `batch_size`: Controls animation speed (number of cells processed per frame).
- `target_fps`: Target frames per second for the window.
- `algorithms_to_run`: A vector of algorithms to execute in sequence (e.g., `vec![Algorithm::Bfs, Algorithm::Dfs]`). Edit this to change the order or add new ones.

To add a new algorithm:
1. Add a variant to the `Algorithm` enum (e.g., `AStar`).
2. Implement the pathfinding function in the `Maze` struct (e.g., `path_finding_astar`).
3. Add a match arm in `get_algorithm_info` with the name, function pointer, and colors.
4. Include it in `algorithms_to_run` in `main()`.