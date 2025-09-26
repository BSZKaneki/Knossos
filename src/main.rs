use std::collections::VecDeque;
use std::thread::sleep;
use std::time::{Duration, Instant};

use minifb::{Key, Window as MiniFbWindow, WindowOptions};
use rand::prelude::IteratorRandom;
use rand::Rng;
use font8x8::legacy::BASIC_LEGACY;
use rayon::prelude::*;

// --- NEW --- An enum to identify the algorithms.
// To add a new one, just add a variant here (e.g., AStar).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Algorithm {
    Bfs,
    Dfs,
}

// --- NEW --- A struct to hold all information related to a specific algorithm.
// This makes the simulation loop completely dynamic.
struct AlgorithmInfo {
    name: &'static str,
    function: fn(&Maze) -> (usize, u128, Vec<(usize, usize)>, Vec<(usize, usize)>),
    search_color: u32,
    path_color: u32,
}

// --- NEW --- A centralized place to define the properties of each algorithm.
// To add a new algorithm, just add a match arm here.
fn get_algorithm_info(algo: Algorithm) -> AlgorithmInfo {
    match algo {
        Algorithm::Bfs => AlgorithmInfo {
            name: "BFS",
            function: Maze::path_finding_bfs,
            search_color: 0xAA0000FF, // Blueish search
            path_color: 0xAAFFFF00,   // Yellow path
        },
        Algorithm::Dfs => AlgorithmInfo {
            name: "DFS",
            function: Maze::path_finding_dfs,
            search_color: 0xAA00FFFF, // Cyan search
            path_color: 0xAAFF00FF,   // Magenta path
        },
    }
}

// --- NEW --- A struct to hold the results for cleaner data management.
struct PathfindingResult {
    name: &'static str,
    color: u32,
    steps: usize,
    duration: u128,
    path_len: usize,
}

// --- CONFIGURATION ---
struct Config {
    screen_width: usize,
    screen_height: usize,
    use_perfect_maze: bool,
    skip_visualization: bool,
    maze_width: usize,
    maze_height: usize,
    batch_size: usize,
    target_fps: u64,
    // --- CHANGED --- We now use a Vec to hold the sequence of algorithms to run.
    algorithms_to_run: Vec<Algorithm>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            screen_width: 1920,
            screen_height: 1080,
            use_perfect_maze: false,
            skip_visualization: false,
            maze_width: 240,
            maze_height: 140,
            batch_size: 40,
            target_fps: 60,
            // --- CHANGED --- Default is now a vector.
            algorithms_to_run: vec![Algorithm::Bfs, Algorithm::Dfs],
        }
    }
}


#[derive(Clone, Copy)]
struct Cell {
    walls: [bool; 4],
    visited: bool,
}

struct Maze {
    start_point: (usize, usize),
    end_point: (usize, usize),
    width: usize,
    height: usize,
    grid: Vec<Cell>,
}

// ... The Maze implementation remains exactly the same
impl Maze {
    fn new(width: usize, height: usize) -> Self {
        let grid = vec![
            Cell {
                walls: [true; 4],
                visited: false
            };
            width * height
        ];
        Maze {
            start_point: (0, 0),
            end_point: (width - 1, height - 1),
            width,
            height,
            grid,
        }
    }

    fn generate_iterative(&mut self) {
        for cell in self.grid.iter_mut() {
            cell.visited = false;
        }

        let mut rng = rand::rng();
        let mut stack = vec![(0, 0)];
        self.grid[0].visited = true;
        while let Some((x, y)) = stack.pop() {
            let mut neighbors = vec![];
            let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
            for (i, &(dx, dy)) in directions.iter().enumerate() {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    let ni = (ny as usize) * self.width + (nx as usize);
                    if !self.grid[ni].visited {
                        neighbors.push((ni, i));
                    }
                }
            }
            if !neighbors.is_empty() {
                stack.push((x, y));
                let &(ni, dir) = neighbors.iter().choose(&mut rng).unwrap();
                self.grid[ni].visited = true;
                self.grid[y * self.width + x].walls[dir] = false;
                self.grid[ni].walls[(dir + 2) % 4] = false;
                let nx = ni % self.width;
                let ny = ni / self.width;
                stack.push((nx, ny));
            }
        }
    }

    fn generate_with_loops(&mut self) {
        self.generate_iterative();

        let mut rng = rand::rng();
        let loop_percentage = 0.08;
        let walls_to_remove = ((self.width * self.height) as f32 * loop_percentage) as usize;

        for _ in 0..walls_to_remove {
            let x = rng.random_range(0..self.width.saturating_sub(1));
            let y = rng.random_range(0..self.height.saturating_sub(1));
            let current_idx = y * self.width + x;

            if rng.random_bool(0.5) {
                let neighbor_idx = y * self.width + (x + 1);
                if self.grid[current_idx].walls[1] {
                    self.grid[current_idx].walls[1] = false;
                    self.grid[neighbor_idx].walls[3] = false;
                }
            } else {
                let neighbor_idx = (y + 1) * self.width + x;
                if self.grid[current_idx].walls[2] {
                    self.grid[current_idx].walls[2] = false;
                    self.grid[neighbor_idx].walls[0] = false;
                }
            }
        }
    }

    fn path_finding_dfs(&self) -> (usize, u128, Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let start_time = Instant::now();
        let mut came_from: Vec<Option<(usize, usize)>> = vec![None; self.width * self.height];
        let mut stack = vec![self.start_point];
        let mut visited_for_dfs: Vec<bool> = vec![false; self.width * self.height];
        let mut entire_path = Vec::new();

        let start_idx = self.start_point.1 * self.width + self.start_point.0;
        visited_for_dfs[start_idx] = true;
        entire_path.push(self.start_point);
        came_from[start_idx] = Some(self.start_point);

        let mut steps = 0;

        while let Some((x, y)) = stack.pop() {
            steps += 1;
            if (x, y) == self.end_point {
                break;
            }

            let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
            for (i, &(dx, dy)) in directions.iter().enumerate() {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    let current_idx = y * self.width + x;
                    let neighbor_idx = ny * self.width + nx;

                    if !self.grid[current_idx].walls[i] && !visited_for_dfs[neighbor_idx] {
                        visited_for_dfs[neighbor_idx] = true;
                        came_from[neighbor_idx] = Some((x, y));
                        stack.push((nx, ny));
                        entire_path.push((nx, ny));
                    }
                }
            }
        }

        let mut path = Vec::new();
        let mut current = self.end_point;
        while current != self.start_point {
            path.push(current);
            if let Some(prev) = came_from[current.1 * self.width + current.0] {
                if prev == current { break; }
                current = prev;
            } else {
                break;
            }
        }
        path.push(self.start_point);
        path.reverse();

        let duration = start_time.elapsed().as_millis();
        (steps, duration, path, entire_path)
    }

    fn path_finding_bfs(&self) -> (usize, u128, Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let start_time = Instant::now();
        let mut came_from: Vec<Option<(usize, usize)>> = vec![None; self.width * self.height];
        let mut queue = VecDeque::new();
        let mut entire_path = Vec::new();

        queue.push_back(self.start_point);
        let start_idx = self.start_point.1 * self.width + self.start_point.0;
        came_from[start_idx] = Some(self.start_point);
        entire_path.push(self.start_point);

        let mut steps = 0;

        while let Some((x, y)) = queue.pop_front() {
            steps += 1;
            if (x, y) == self.end_point {
                break;
            }
            let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
            for (i, &(dx, dy)) in directions.iter().enumerate() {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if !self.grid[y * self.width + x].walls[i] {
                        if came_from[ny * self.width + nx].is_none() {
                            came_from[ny * self.width + nx] = Some((x, y));
                            queue.push_back((nx, ny));
                            entire_path.push((nx, ny));
                        }
                    }
                }
            }
        }

        let mut path = Vec::new();
        let mut current = self.end_point;
        while current != self.start_point {
            path.push(current);
             if let Some(prev) = came_from[current.1 * self.width + current.0] {
                if prev == current { break; }
                current = prev;
            } else {
                break;
            }
        }
        path.push(self.start_point);
        path.reverse();

        let duration = start_time.elapsed().as_millis();
        (steps, duration, path, entire_path)
    }
}


struct Visualization<'a> {
    window: MiniFbWindow,
    buffer: Vec<u32>,
    config: &'a Config,
    cell_size: usize,
    offset_x: usize,
    offset_y: usize,
}

// ... Visualization struct has minor changes, mostly simplification
impl<'a> Visualization<'a> {
    fn new(config: &'a Config) -> Self {
        let mut window = MiniFbWindow::new(
            "Maze Pathfinding",
            config.screen_width,
            config.screen_height,
            WindowOptions::default(),
        )
        .unwrap();
        window.set_target_fps(config.target_fps as usize);

        let buffer = vec![0; config.screen_width * config.screen_height];

        let min_margin = 50;
        let max_cell_width = (config.screen_width - 2 * min_margin) / config.maze_width;
        let max_cell_height = (config.screen_height - 2 * min_margin) / config.maze_height;
        let cell_size = max_cell_width.min(max_cell_height).max(1);

        let maze_width_px = config.maze_width * cell_size;
        let maze_height_px = config.maze_height * cell_size;
        let offset_x = (config.screen_width.saturating_sub(maze_width_px)) / 2;
        let offset_y = (config.screen_height.saturating_sub(maze_height_px)) / 2;

        Self {
            window,
            buffer,
            config,
            cell_size,
            offset_x,
            offset_y,
        }
    }

    fn draw_char(&mut self, x: usize, y: usize, c: char, color: u32) {
        if let Some(bitmap) = BASIC_LEGACY.get(c as usize) {
            for (row, bits) in bitmap.iter().enumerate() {
                for col in 0..8 {
                    if (bits >> col) & 1 == 1 {
                        let px = x + col;
                        let py = y + row;
                        if px < self.config.screen_width && py < (self.buffer.len() / self.config.screen_width) {
                            self.buffer[py * self.config.screen_width + px] = color;
                        }
                    }
                }
            }
        }
    }

    fn draw_text(&mut self, x: usize, y: usize, text: &str, color: u32) {
        for (i, c) in text.chars().enumerate() {
            self.draw_char(x + i * 8, y, c, color);
        }
    }
    
    fn draw_maze(&mut self, maze: &Maze) {
        self.buffer.fill(0x00101020);
        let wall_color = 0xFF808080;
        let maze_width_px = self.config.maze_width * self.cell_size;
        let maze_height_px = self.config.maze_height * self.cell_size;

        self.buffer
            .par_chunks_mut(self.config.screen_width)
            .enumerate()
            .for_each(|(y_pixel, row_slice)| {
                if y_pixel < self.offset_y || y_pixel >= self.offset_y + maze_height_px { return; }
                let y_cell = (y_pixel - self.offset_y) / self.cell_size;
                if y_cell >= maze.height { return; }

                for (x_pixel_in_row, pixel) in row_slice.iter_mut().enumerate() {
                    let x_pixel = x_pixel_in_row;
                    if x_pixel < self.offset_x || x_pixel >= self.offset_x + maze_width_px { continue; }
                    let x_cell = (x_pixel - self.offset_x) / self.cell_size;
                    if x_cell >= maze.width { continue; }

                    let inner_x = (x_pixel - self.offset_x) % self.cell_size;
                    let inner_y = (y_pixel - self.offset_y) % self.cell_size;

                    let cell = &maze.grid[y_cell * maze.width + x_cell];

                    let mut is_wall = false;
                    if cell.walls[0] && inner_y == 0 { is_wall = true; }
                    if cell.walls[1] && inner_x == self.cell_size - 1 { is_wall = true; }
                    if cell.walls[2] && inner_y == self.cell_size - 1 { is_wall = true; }
                    if cell.walls[3] && inner_x == 0 { is_wall = true; }

                    if is_wall { *pixel = wall_color; }
                }
            });

        self.draw_path(&[maze.start_point], 0x0000FF00, false);
        self.draw_path(&[maze.end_point], 0x00FF0000, false);
        self.update_screen();
    }


    fn draw_path(&mut self, path: &[(usize, usize)], color: u32, slow_draw: bool) {
        let path_size = (self.cell_size / 2).max(1);
        let path_offset = (self.cell_size - path_size) / 2;
        for &(x, y) in path {
            for dy in 0..path_size {
                for dx in 0..path_size {
                    let px = self.offset_x + x * self.cell_size + path_offset + dx;
                    let py = self.offset_y + y * self.cell_size + path_offset + dy;
                    let idx = py * self.config.screen_width + px;
                    if idx < self.buffer.len() {
                        self.buffer[idx] = color;
                    }
                }
            }
            if slow_draw {
                self.update_screen();
                sleep(Duration::from_micros(100));
            }
        }
        if !slow_draw {
            self.update_screen();
        }
    }

    fn draw_search_animation(&mut self, entire_path: &[(usize, usize)], color: u32, title: &str) {
        self.draw_text(10, 10, title, 0xFFFFFFFF);
        let path_size = (self.cell_size / 2).max(1);
        let path_offset = (self.cell_size - path_size) / 2;
        let mut batch_counter = 0;

        for &(x, y) in entire_path {
            for dy in 0..path_size {
                for dx in 0..path_size {
                    let px = self.offset_x + x * self.cell_size + path_offset + dx;
                    let py = self.offset_y + y * self.cell_size + path_offset + dy;
                    if let Some(pixel) = self.buffer.get_mut(py * self.config.screen_width + px) {
                        *pixel = color;
                    }
                }
            }
            batch_counter += 1;
            if batch_counter >= self.config.batch_size {
                self.update_screen();
                sleep(Duration::from_micros(16600)); // ~60fps
                batch_counter = 0;
            }
        }
        self.update_screen();
    }

    fn update_screen(&mut self) {
        self.window.update_with_buffer(&self.buffer, self.config.screen_width, self.config.screen_height).unwrap();
    }
}


struct Simulation<'a> {
    config: &'a Config,
    maze: Maze,
    viz: Visualization<'a>,
    maze_created: bool,
}

// --- CHANGED --- The entire Simulation logic is now a dynamic loop.
impl<'a> Simulation<'a> {
    fn new(config: &'a Config) -> Self {
        Self {
            config,
            maze: Maze::new(config.maze_width, config.maze_height),
            viz: Visualization::new(config),
            maze_created: false,
        }
    }

    fn run(&mut self) {
        while self.viz.window.is_open() && !self.viz.window.is_key_down(Key::Escape) {
            if !self.maze_created {
                self.run_full_simulation();
                self.maze_created = true;
            }
            self.viz.update_screen();
        }
    }

    fn run_full_simulation(&mut self) {
        // Step 1: Generate the maze
        if self.config.use_perfect_maze {
            self.maze.generate_iterative();
        } else {
            self.maze.generate_with_loops();
        }
        
        if !self.config.skip_visualization {
            self.viz.draw_maze(&self.maze);
            sleep(Duration::from_secs(1));
        }
        
        // Step 2: Run all chosen algorithms and collect results
        let mut results: Vec<PathfindingResult> = Vec::new();
        
        for (i, algo) in self.config.algorithms_to_run.iter().enumerate() {
            let info = get_algorithm_info(*algo);
            
            // Calculation is always performed
            let (steps, duration, path, entire_path) = (info.function)(&self.maze);
            
            results.push(PathfindingResult {
                name: info.name,
                color: info.path_color,
                steps,
                duration,
                path_len: path.len(),
            });

            // Visualization only runs if not skipped
            if !self.config.skip_visualization {
                let title = format!("Algorithm: {}", info.name);
                self.viz.draw_search_animation(&entire_path, info.search_color, &title);
                self.viz.draw_path(&path, info.path_color, true);
                sleep(Duration::from_secs(2));

                // If this is not the last algorithm to visualize, reset the view
                if i < self.config.algorithms_to_run.len() - 1 {
                    self.viz.draw_maze(&self.maze); // Redraw maze to clear paths
                    sleep(Duration::from_secs(1));
                }
            }
        }
        
        // Step 3: Display the final statistics screen
        self.display_final_stats(results);
    }
    
    // This is now the one and only stats screen function. It dynamically renders all results.
    fn display_final_stats(&mut self, results: Vec<PathfindingResult>) {
        self.viz.buffer.fill(0x00101020);
        
        let mut y_offset = 10;
        self.viz.draw_text(10, y_offset, "--- Pathfinding Results ---", 0xFFFFFFFF);
        y_offset += 15;

        let maze_type_text = format!("Maze Type:       {}", if self.config.use_perfect_maze { "Perfect (No Loops)" } else { "Imperfect (With Loops)" });
        let maze_dim_text = format!("Maze Dimensions: {}x{}", self.config.maze_width, self.config.maze_height);
        self.viz.draw_text(10, y_offset, &maze_type_text, 0xFF808080);
        y_offset += 10;
        self.viz.draw_text(10, y_offset, &maze_dim_text, 0xFF808080);
        y_offset += 25;

        for result in results {
            let stats1 = format!("Algorithm:      {}", result.name);
            let stats2 = format!("Steps Taken:    {}", result.steps);
            let stats3 = format!("Time Elapsed:   {} ms", result.duration);
            let stats4 = format!("Final Path Len: {}", result.path_len);
            
            self.viz.draw_text(10, y_offset, &stats1, result.color);
            y_offset += 10;
            self.viz.draw_text(10, y_offset, &stats2, 0xFFFFFFFF);
            y_offset += 10;
            self.viz.draw_text(10, y_offset, &stats3, 0xFFFFFFFF);
            y_offset += 10;
            self.viz.draw_text(10, y_offset, &stats4, 0xFFFFFFFF);
            y_offset += 40; // Add spacing for the next algorithm
        }
    }
}


fn main() {
    // --- CHANGED --- This is now the single point of control.
    // Simply edit the vector to change which algorithms are run and in what order.
    let config = Config {
        // examples:
        // algorithms_to_run: vec![Algorithm::Bfs],
        // algorithms_to_run: vec![Algorithm::Dfs, Algorithm::Bfs],
        algorithms_to_run: vec![Algorithm::Bfs, Algorithm::Dfs],
        ..Default::default()
    };

    let mut simulation = Simulation::new(&config);
    simulation.run();
}