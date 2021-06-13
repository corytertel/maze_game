use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use super::maze_cell::MazeCell;
use super::maze_wall::MazeWall;
use super::maze_algorithm::MazeAlgorithm;

pub struct Maze {
    maze_algorithm: Box<dyn MazeAlgorithm>,
    cells: Vec<Vec<MazeCell>>,
    walls: Vec<Rc<RefCell<MazeWall>>>,
    width: usize,
    height: usize,
}

impl Maze {
    //Constructor
    pub fn new(width: usize, height: usize, maze_algorithm: Box<dyn MazeAlgorithm>) -> Self {
        let mut tmp = Self {
            maze_algorithm,
            cells: Vec::new(),
            walls: Vec::new(),
            width,
            height,
        };

        tmp.reconstruct();
        tmp.regenerate();

        tmp
    }

    //Resets all walls in maze to active. Primarily used to reset a maze for regeneration.
    pub fn reset(&mut self) {
        for wall in self.walls.iter_mut() {
            wall.borrow_mut().active = true;
        }
    }

    //Reconstructs maze with the dimensions width and height. Used to build a maze. No algorithm is applied on the Maze.
    pub fn reconstruct(&mut self) {
        let width = self.width;
        let height = self.height;
        self.cells = vec![vec![MazeCell::new(); height]; width];

        //Build Walls

        for x in 0..width {
            for y in 0..height {
                let top_wall = Rc::new(RefCell::new(MazeWall::new(true)));
                let left_wall = Rc::new(RefCell::new(MazeWall::new(true)));

                //Set current cell's top and left walls
                self.cells[x][y].set_top_wall(Rc::clone(&top_wall));
                self.cells[x][y].set_left_wall(Rc::clone(&left_wall));

                //Set neighboring cell's bottom and right walls
                if x != 0 {
                    self.cells[x - 1][y].set_right_wall(Rc::clone(&left_wall));
                }
                if y != 0 {
                    self.cells[x][y - 1].set_bottom_wall(Rc::clone(&top_wall));
                }

                //Add wall to the Maze walls vector
                self.walls.push(Rc::clone(&top_wall));
                self.walls.push(Rc::clone(&left_wall));
            }
        }

        //Add walls to the bottom row and right column
        for column in self.cells.iter_mut() {
            let bottom_wall = Rc::new(RefCell::new(MazeWall::new(true)));
            column[height - 1].set_bottom_wall(Rc::clone(&bottom_wall));
            self.walls.push(Rc::clone(&bottom_wall));
        }

        for row in self.cells[width - 1].iter_mut() {
            let right_wall = Rc::new(RefCell::new(MazeWall::new(true)));
            row.set_right_wall(Rc::clone(&right_wall));
            self.walls.push(Rc::clone(&right_wall));
        }
    }

    pub fn regenerate(&mut self) {
        self.reset();
        self.maze_algorithm.generate(&mut self.cells, &mut self.walls);
    }

    pub fn set_algorithm(&mut self, new_algorithm: Box<dyn MazeAlgorithm>) {
        self.maze_algorithm = new_algorithm;
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //Exception case
        if self.cells.len() == 0 { return write!(f, "Maze is empty."); }

        let mut maze_str = String::new();

        for y in 0..self.height {
            //Top Row
            for x in 0..self.width {
                maze_str += "██";

                if self.cells[x][y].top_wall().active { 
                    maze_str += "██"; 
                }
                else { 
                    maze_str += "  "; 
                }
            }

            //Rightest wall of top row
            maze_str += "██\n";
            
            //Middle Row
            for x in 0..self.width {
                if self.cells[x][y].left_wall().active { 
                    maze_str += "██"; 
                }
                else { 
                    maze_str += "  "; 
                }

                maze_str += "  ";
            }

            //Rightest wall of middle row
            if self.cells[self.width - 1][y].right_wall().active { 
                maze_str += "██\n"; 
            }
            else { 
                maze_str += "  \n"; 
            }
        }

        //Bottom Row
        for x in 0..self.width {
            maze_str += "██";

            if self.cells[x][self.height - 1].bottom_wall().active { 
                maze_str += "██"; 
            }
            else { 
                maze_str += "  "; 
            }
        }

        //Bottom right corner
        maze_str += "██\n";

        write!(f, "{}", maze_str)
    }
}