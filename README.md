
Maze Generation and Pathfinding Visualizer

This project is a Rust application that visually demonstrates and compares maze generation algorithms and pathfinding algorithms (Breadth-First Search vs. Depth-First Search). It generates a maze, visualizes the process of each algorithm exploring the maze to find a solution, and then presents a statistical comparison of their performance.

(You would replace this with a GIF you record of your own application)

Features

Maze Generation: Creates complex mazes using a recursive backtracking algorithm.

Perfect & Imperfect Mazes: Option to generate a "perfect" maze (one unique path between any two points) or an "imperfect" maze by removing a percentage of walls to create loops.

Dual Algorithm Visualization: Animates both BFS and DFS pathfinding algorithms side-by-side on the same maze for a direct comparison.

Step-by-Step Animation: Clearly visualizes the exploration process (visited nodes) and the final path for each algorithm.

Performance Metrics: After the visualization, it displays key statistics:

Time elapsed for each algorithm to find the path.

Number of steps (nodes visited).

Length of the final path found.

Highly Configurable: Easily change maze dimensions, screen resolution, animation speed, and maze type directly in the source code.

High-Performance Rendering: Built with Rust and minifb for fast, low-level graphical output, with parallel processing using rayon to speed up maze drawing.

Algorithms Showcased
Maze Generation

Iterative Deepening (Recursive Backtracking): This algorithm carves out a maze by starting at a random point and performing a randomized depth-first search. The result is a "perfect" maze with no loops.

Loop Creation: An additional step can be enabled to randomly remove a percentage of walls after the initial generation, creating an "imperfect" maze with multiple possible paths and loops.

Pathfinding

Breadth-First Search (BFS): An algorithm that explores the maze layer by layer from the starting point. It guarantees finding the shortest possible path in an unweighted grid like this maze.

Depth-First Search (DFS): An algorithm that explores as far as possible down one branch before backtracking. It is often faster in terms of raw computation time but does not guarantee the shortest path.

Getting Started
Prerequisites

Rust: You must have the Rust programming language toolchain installed. You can install it from rustup.rs.

How to Run

Clone the repository:

code
Bash
download
content_copy
expand_less
git clone https://github.com/YOUR_USERNAME/YOUR_REPOSITORY_NAME.git
cd YOUR_REPOSITORY_NAME

Run the application:
The project can be run directly using Cargo. For the best performance, run it in release mode.

code
Bash
download
content_copy
expand_less
cargo run --release

To Quit:
Press the Escape key or close the window.

Configuration

You can customize the simulation by changing the const values at the top of the main() function in src/main.rs.

code
Rust
download
content_copy
expand_less
fn main() {
    // --- CONFIGURATION ---
    const SCREEN_WIDTH: usize = 1920;
    const SCREEN_HEIGHT: usize = 1080;
    
    // Set to 'true' for a perfect maze, 'false' to allow loops.
    const USE_PERFECT_MAZE: bool = false;
    
    // Set to 'true' to skip animations and just see the final stats.
    const SKIP_VISUALIZATION: bool = false;

    // Change the maze size (width, height).
    const MAZE_SIZE: (usize, usize) = (240, 140);
    
    // Controls animation speed: draws this many cells per screen update.
    const BATCH_SIZE: usize = 20;

    // ... rest of the code
}
Running Tests

Unit tests are included to verify the core logic of maze creation and pathfinding. To run the tests, use the following command:

code
Bash
download
content_copy
expand_less
cargo test
Dependencies

This project relies on several excellent crates from the Rust ecosystem:

minifb - For creating a simple, cross-platform window and drawing the framebuffer.

rand - Used for random number generation during maze creation.

rayon - For data parallelism, used here to significantly speed up the initial drawing of the maze.

font8x8 - A simple crate for rendering bitmap fonts to display statistics.

License

This project is licensed under the MIT License - see the LICENSE.md file for details.
(You would need to create a LICENSE.md file with the MIT license text if you choose this license)