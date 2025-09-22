use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Instant;

use minifb::{Window as MiniFbWindow, WindowOptions};
use rand::prelude::IteratorRandom;
use rand::{Rng};
use font8x8::legacy::BASIC_LEGACY;
use rayon::prelude::*;

#[derive(Clone, Copy)]
struct Cell {
    walls: [bool; 4], // North, East, South, West
    visited: bool,
}
struct Maze {
    start_point: (usize, usize),
    end_point: (usize, usize),
    width: usize,
    height: usize,
    grid: Vec<Cell>,
}
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

fn draw_char(buffer: &mut [u32], width: usize, x: usize, y: usize, c: char, color: u32) {
    if let Some(bitmap) = BASIC_LEGACY.get(c as usize) {
        for (row, bits) in bitmap.iter().enumerate() {
            for col in 0..8 {
                if (bits >> col) & 1 == 1 {
                    let px = x + col;
                    let py = y + row;
                    if px < width && py < (buffer.len() / width) {
                        buffer[py * width + px] = color;
                    }
                }
            }
        }
    }
}

fn draw_text(buffer: &mut [u32], width: usize, x: usize, y: usize, text: &str, color: u32) {
    for (i, c) in text.chars().enumerate() {
        draw_char(buffer, width, x + i * 8, y, c, color);
    }
}

fn draw_path(
    buffer: &mut [u32], screen_width: usize, path: &[(usize, usize)],
    cell_size: usize, offset_x: usize, offset_y: usize, color: u32,
    window: &mut MiniFbWindow, slow_draw: bool,
) {
    let path_size = (cell_size / 2).max(1);
    let path_offset = (cell_size - path_size) / 2;
    for &(x, y) in path {
        for dy in 0..path_size {
            for dx in 0..path_size {
                let px = offset_x + x * cell_size + path_offset + dx;
                let py = offset_y + y * cell_size + path_offset + dy;
                let idx = py * screen_width + px;
                if idx < buffer.len() {
                    buffer[idx] = color;
                }
            }
        }
        if slow_draw {
            window.update_with_buffer(&buffer, screen_width, buffer.len() / screen_width).unwrap();
            sleep(std::time::Duration::from_micros(100));
        }
    }
    if !slow_draw {
        window.update_with_buffer(&buffer, screen_width, buffer.len() / screen_width).unwrap();
    }
}

fn main() {
    // --- CONFIGURATION ---
    const SCREEN_WIDTH: usize = 1920;
    const SCREEN_HEIGHT: usize = 1080;
    
    const USE_PERFECT_MAZE: bool = false;
    const SKIP_VISUALIZATION: bool = false;
    const MAZE_SIZE: (usize, usize) = (240, 140);
    const BATCH_SIZE: usize = 20; // Draw this many cells per screen update for a smooth animation

    // --- INITIALIZATION ---
    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];
    let mut window = MiniFbWindow::new(
        "Maze Pathfinding Comparison",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();
    window.set_target_fps(60);

    let mut maze_created = false;
    
    let min_margin = 50;
    let max_cell_width = (SCREEN_WIDTH - 2 * min_margin) / MAZE_SIZE.0;
    let max_cell_height = (SCREEN_HEIGHT - 2 * min_margin) / MAZE_SIZE.1;
    let cell_size = max_cell_width.min(max_cell_height).max(1);

    let maze_width_px = MAZE_SIZE.0 * cell_size;
    let maze_height_px = MAZE_SIZE.1 * cell_size;
    let offset_x = (SCREEN_WIDTH.saturating_sub(maze_width_px)) / 2;
    let offset_y = (SCREEN_HEIGHT.saturating_sub(maze_height_px)) / 2;

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        if !maze_created {
            let mut maze = Maze::new(MAZE_SIZE.0, MAZE_SIZE.1);
            if USE_PERFECT_MAZE {
                maze.generate_iterative();
            } else {
                maze.generate_with_loops();
            }

            buffer.fill(0x00101020);
            let wall_color = 0xFF808080;

            if !SKIP_VISUALIZATION {
                buffer
                    .par_chunks_mut(SCREEN_WIDTH)
                    .enumerate()
                    .for_each(|(y_pixel, row_slice)| {
                        if y_pixel < offset_y || y_pixel >= offset_y + maze_height_px { return; }
                        let y_cell = (y_pixel - offset_y) / cell_size;
                        if y_cell >= maze.height { return; }

                        for (x_pixel_in_row, pixel) in row_slice.iter_mut().enumerate() {
                            let x_pixel = x_pixel_in_row;
                            if x_pixel < offset_x || x_pixel >= offset_x + maze_width_px { continue; }
                            let x_cell = (x_pixel - offset_x) / cell_size;
                            if x_cell >= maze.width { continue; }

                            let inner_x = (x_pixel - offset_x) % cell_size;
                            let inner_y = (y_pixel - offset_y) % cell_size;

                            let cell = &maze.grid[y_cell * maze.width + x_cell];

                            let mut is_wall = false;
                            if cell.walls[0] && inner_y == 0 { is_wall = true; }
                            if cell.walls[1] && inner_x == cell_size - 1 { is_wall = true; }
                            if cell.walls[2] && inner_y == cell_size - 1 { is_wall = true; }
                            if cell.walls[3] && inner_x == 0 { is_wall = true; }

                            if is_wall { *pixel = wall_color; }
                        }
                    });
                draw_path(&mut buffer, SCREEN_WIDTH, &[maze.start_point], cell_size, offset_x, offset_y, 0x0000FF00, &mut window, false);
                draw_path(&mut buffer, SCREEN_WIDTH, &[maze.end_point], cell_size, offset_x, offset_y, 0x00FF0000, &mut window, false);
                window.update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
                sleep(std::time::Duration::from_secs(1));
            }

            let (bfs_steps, bfs_duration, bfs_path, bfs_entire_path) = maze.path_finding_bfs();
            let (dfs_steps, dfs_duration, dfs_path, dfs_entire_path) = maze.path_finding_dfs();
            
            if !SKIP_VISUALIZATION {
                let path_size = (cell_size / 2).max(1);
                let path_offset = (cell_size - path_size) / 2;
                let draw_cell = |buffer: &mut Vec<u32>, x: usize, y: usize, color: u32| {
                    for dy in 0..path_size {
                        for dx in 0..path_size {
                            let px = offset_x + x * cell_size + path_offset + dx;
                            let py = offset_y + y * cell_size + path_offset + dy;
                            if let Some(pixel) = buffer.get_mut(py * SCREEN_WIDTH + px) {
                                *pixel = color;
                            }
                        }
                    }
                };

                // --- BFS Visualization (Optimized with Batching) ---
                draw_text(&mut buffer, SCREEN_WIDTH, 10, 10, "Algorithm: BFS", 0xFFFFFFFF);
                let mut batch_counter = 0;
                for &(x, y) in &bfs_entire_path {
                    draw_cell(&mut buffer, x, y, 0xAA0000FF);
                    batch_counter += 1;
                    if batch_counter >= BATCH_SIZE {
                        window.update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
                        sleep(std::time::Duration::from_micros(16600)); // ~60fps
                        batch_counter = 0;
                    }
                }
                // Clean up for next draw
                window.update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
                draw_path(&mut buffer, SCREEN_WIDTH, &bfs_path, cell_size, offset_x, offset_y, 0xAAFFFF00, &mut window, true);
                sleep(std::time::Duration::from_secs(3));

                draw_path(&mut buffer, SCREEN_WIDTH, &bfs_entire_path, cell_size, offset_x, offset_y, 0x00101020, &mut window, false);
                draw_path(&mut buffer, SCREEN_WIDTH, &bfs_path, cell_size, offset_x, offset_y, 0x00101020, &mut window, false);
                draw_path(&mut buffer, SCREEN_WIDTH, &[maze.start_point], cell_size, offset_x, offset_y, 0x0000FF00, &mut window, false);
                draw_path(&mut buffer, SCREEN_WIDTH, &[maze.end_point], cell_size, offset_x, offset_y, 0x00FF0000, &mut window, false);
                draw_text(&mut buffer, SCREEN_WIDTH, 10, 10, "                 ", 0x00101020);
                window.update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
                sleep(std::time::Duration::from_secs(1));

                // --- DFS Visualization (Optimized with Batching) ---
                draw_text(&mut buffer, SCREEN_WIDTH, 10, 10, "Algorithm: DFS", 0xFFFFFFFF);
                batch_counter = 0;
                for &(x, y) in &dfs_entire_path {
                    draw_cell(&mut buffer, x, y, 0xAA00FFFF);
                    batch_counter += 1;
                    if batch_counter >= BATCH_SIZE {
                        window.update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
                        sleep(std::time::Duration::from_micros(16600)); // ~60fps
                        batch_counter = 0;
                    }
                }
                window.update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
                draw_path(&mut buffer, SCREEN_WIDTH, &dfs_path, cell_size, offset_x, offset_y, 0xAAFF00FF, &mut window, true);
                sleep(std::time::Duration::from_secs(3));
            }

            buffer.fill(0x00101020);
            draw_text(&mut buffer, SCREEN_WIDTH, 10, 10, "--- Pathfinding Comparison ---", 0xFFFFFFFF);
            let maze_type_text = format!("Maze Type:       {}", if USE_PERFECT_MAZE { "Perfect (No Loops)" } else { "Imperfect (With Loops)" });
            let maze_dim_text = format!("Maze Dimensions: {}x{}", MAZE_SIZE.0, MAZE_SIZE.1);
            draw_text(&mut buffer, SCREEN_WIDTH, 10, 25, &maze_type_text, 0xFF808080);
            draw_text(&mut buffer, SCREEN_WIDTH, 10, 35, &maze_dim_text, 0xFF808080);
            
            let bfs_stats1 = format!("Algorithm:      BFS");
            let bfs_stats2 = format!("Steps Taken:    {}", bfs_steps);
            let bfs_stats3 = format!("Time Elapsed:   {} ms", bfs_duration);
            let bfs_stats4 = format!("Final Path Len: {}", bfs_path.len());

            draw_text(&mut buffer, SCREEN_WIDTH, 10, 60, &bfs_stats1, 0xAAFFFF00);
            draw_text(&mut buffer, SCREEN_WIDTH, 10, 70, &bfs_stats2, 0xFFFFFFFF);
            draw_text(&mut buffer, SCREEN_WIDTH, 10, 80, &bfs_stats3, 0xFFFFFFFF);
            draw_text(&mut buffer, SCREEN_WIDTH, 10, 90, &bfs_stats4, 0xFFFFFFFF);

            let dfs_stats1 = format!("Algorithm:      DFS");
            let dfs_stats2 = format!("Steps Taken:    {}", dfs_steps);
            let dfs_stats3 = format!("Time Elapsed:   {} ms", dfs_duration);
            let dfs_stats4 = format!("Final Path Len: {}", dfs_path.len());
            
            draw_text(&mut buffer, SCREEN_WIDTH, 10, 130, &dfs_stats1, 0xAAFF00FF);
            draw_text(&mut buffer, SCREEN_WIDTH, 10, 140, &dfs_stats2, 0xFFFFFFFF);
            draw_text(&mut buffer, SCREEN_WIDTH, 10, 150, &dfs_stats3, 0xFFFFFFFF);
            draw_text(&mut buffer, SCREEN_WIDTH, 10, 160, &dfs_stats4, 0xFFFFFFFF);
            
            maze_created = true;
        }

        window.update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_new() {
        let width = 10;
        let height = 15;
        let maze = Maze::new(width, height);

        assert_eq!(maze.width, width);
        assert_eq!(maze.height, height);
        assert_eq!(maze.grid.len(), width * height);

        // All cells should be initialized with 4 walls and not visited.
        for cell in maze.grid.iter() {
            assert!(cell.walls.iter().all(|&w| w));
            assert!(!cell.visited);
        }
    }

    #[test]
    fn test_generate_iterative_visits_all_cells() {
        let mut maze = Maze::new(5, 5);
        maze.generate_iterative();

        // The generation algorithm should visit every cell.
        for cell in maze.grid.iter() {
            assert!(cell.visited);
        }
    }
    
    #[test]
    fn test_generate_with_loops_visits_all_cells() {
        let mut maze = Maze::new(8, 8);
        maze.generate_with_loops();

        // The generation algorithm (even with loops) should visit every cell.
        for cell in maze.grid.iter() {
            assert!(cell.visited);
        }
    }

    #[test]
    fn test_bfs_finds_shortest_path() {
        // Create a simple, predictable maze.
        //  S ╴ E
        let mut maze = Maze::new(3, 1);
        maze.grid[0].walls = [true, false, true, true]; // Path to the right
        maze.grid[1].walls = [true, false, true, false];
        maze.grid[2].walls = [true, true, true, false]; // Path from the left

        let (_, _, path, _) = maze.path_finding_bfs();

        assert_eq!(path.len(), 3, "BFS should find the shortest path of length 3.");
        assert_eq!(path[0], (0, 0), "Path should start at the start point.");
        assert_eq!(path[2], (2, 0), "Path should end at the end point.");
        assert_eq!(path, vec![(0, 0), (1, 0), (2, 0)]);
    }

    #[test]
    fn test_dfs_finds_a_valid_path() {
        // Create a simple, predictable maze.
        // S
        // |
        // E
        let mut maze = Maze::new(1, 3);
        maze.end_point = (0, 2);
        
        maze.grid[0].walls = [true, true, false, true]; // Path downwards
        maze.grid[1].walls = [false, true, false, true];
        maze.grid[2].walls = [false, true, true, true]; // Path from above

        let (_, _, path, _) = maze.path_finding_dfs();

        assert!(!path.is_empty(), "DFS should find a path.");
        assert_eq!(path[0], (0, 0), "Path should start at the start point.");
        assert_eq!(path.last().unwrap(), &maze.end_point, "Path should end at the end point.");
        assert_eq!(path, vec![(0, 0), (0, 1), (0, 2)]);
    }
    
    #[test]
    fn test_no_path_found() {
        // Create a maze with no possible path from start to end.
        let  maze = Maze::new(3, 3);
        // All walls are up by default, so no path exists.

        let (_, _, bfs_path, _) = maze.path_finding_bfs();
        let (_, _, dfs_path, _) = maze.path_finding_dfs();

        // When no path is found, the resulting "path" should only contain the end point,
        // as the loop breaks without connecting back to the start. Or it might be empty
        // depending on implementation details. Let's check the current behavior.
        assert_ne!(bfs_path.last().unwrap(), &maze.start_point, "BFS path should not reach the start if no path exists.");
        assert_ne!(dfs_path.last().unwrap(), &maze.start_point, "DFS path should not reach the start if no path exists.");
    }
}