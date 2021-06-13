mod maze;
use maze::Maze;
mod maze_algorithm;
use maze_algorithm::{DepthFirstSearch, PrimsAlgorithm, KruskalsAlgorithm};
mod maze_cell;
mod maze_wall;

fn main() {
    let mut maze = Maze::new(15, 15, Box::new(DepthFirstSearch));

    println!("===Maze Generator Test===");
    println!("\nDepth First Search:");
    println!("{}", maze);

    maze.set_algorithm(Box::new(PrimsAlgorithm));
    maze.regenerate();
    println!("\nPrim's Algorithm");
    println!("{}", maze);

    maze.set_algorithm(Box::new(KruskalsAlgorithm));
    maze.regenerate();
    println!("\nKruskal's Algorithm");
    println!("{}", maze);
}
