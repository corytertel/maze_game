extern crate rand;

use std::rc::Rc;
use std::cell::RefCell;

use super::maze_cell::MazeCell;
use super::maze_wall::MazeWall;

pub trait MazeAlgorithm {
    fn generate(&self, cells: &mut Vec<Vec<MazeCell>>, walls: &mut Vec<Rc<RefCell<MazeWall>>>);
}

pub struct DepthFirstSearch;

impl MazeAlgorithm for DepthFirstSearch {
    fn generate(&self, cells: &mut Vec<Vec<MazeCell>>, _walls: &mut Vec<Rc<RefCell<MazeWall>>>) {
        #[derive(Clone, Copy, Debug)]
        enum NextPosition {
            Up,
            Down,
            Left,
            Right,
            Backtrack,
        }

        #[derive(Clone, Copy, Debug)]
        struct Point {
            pub x: usize,
            pub y: usize,
        }
        
        //Depth first search algorithm
        //1. Randomly choose an initial cell, mark it as visited, add it to stack
        //2. Randomly choose the next cell from the cell's unvisited neighbors
        //3. Mark the chosen cell as visited, remove the walls between the two cells,
        // and set the chosen cell as the current cell. Add it to the stack.
        //4. Repeat this process until a cell has no unvisted neighbors.
        //5. Backtrack on the path that is in the stack until you reach a cell with an unvisted neighbor.
        //6. Continue the process from there.
        //7. When the algorithm backtracks back to the inital cell, the maze is complete.
    
        //Picks a random cell on the map
        let width = cells.len();
        let height = cells[0].len();

        let initial = Point { 
            x: rand::random::<usize>() % width,
            y: rand::random::<usize>() % height,
        };

        let mut current = initial;

        let mut visited = vec![vec![false; height]; width];
        let mut backtracked = vec![vec![false; height]; width];

        let mut stack: Vec<Point> = Vec::new();

        visited[initial.x][initial.y] = true;

        //Determines where the next cell is in the depth-first search algorithm
        let next_cell_position = |current: &Point, visited: &Vec<Vec<bool>>| -> NextPosition {
            let mut random_cell_list: Vec<NextPosition> = Vec::new();
    
            if (current.x > 0) && (!visited[current.x - 1][current.y])                     { random_cell_list.push(NextPosition::Left); }  //If it can move to the Above Cell
            if (current.x < cells.len() - 1) && (!visited[current.x + 1][current.y])       { random_cell_list.push(NextPosition::Right); } //If it can move to the Below Cell
            if (current.y > 0) && (!visited[current.x][current.y - 1])                     { random_cell_list.push(NextPosition::Up); }    //If it can move to the Left Cell
            if (current.y < cells[0].len() - 1) && (!visited[current.x][current.y + 1])    { random_cell_list.push(NextPosition::Down); }  //If it can move to the Right Cell
    
            //Randomly selects a direction to go with the algorithm
            //If it cannot move anywhere, then it is a deadend and needs to backtrack
            if random_cell_list.len() > 0 {
                random_cell_list[rand::random::<usize>() % random_cell_list.len()]
            } 
            else {
                NextPosition::Backtrack
            }
        };

        loop {
            match next_cell_position(&current, &visited) {
                NextPosition::Up => {
                    cells[current.x][current.y].top_wall_mut().borrow_mut().active = false;

                    stack.push(current.clone());
                    current.y -= 1;
                    visited[current.x][current.y] = true;
                }
    
                NextPosition::Down => {
                    cells[current.x][current.y].bottom_wall_mut().borrow_mut().active = false;

                    stack.push(current);
                    current.y += 1;
                    visited[current.x][current.y] = true;
                }
    
                NextPosition::Left => {
                    cells[current.x][current.y].left_wall_mut().borrow_mut().active = false;

                    stack.push(current);
                    current.x -= 1;
                    visited[current.x][current.y] = true;
                }
    
                NextPosition::Right => {
                    cells[current.x][current.y].right_wall_mut().borrow_mut().active = false;

                    stack.push(current);
                    current.x += 1;
                    visited[current.x][current.y] = true;
                }
    
                NextPosition::Backtrack => {
                    backtracked[current.x][current.y] = true;
                    if let Some(value) = stack.pop() {
                        current = value;
                    }
                }
            }
    
            
            //If the algorithm backtracked all the way to the initial cell, then it's done.
            if backtracked[initial.x][initial.y] { break; }
        }

        //Set up maze exits
        cells[0][0].left_wall_mut().borrow_mut().active = false;
        cells[width - 1][height - 1].right_wall_mut().borrow_mut().active = false;
    }
}

pub struct PrimsAlgorithm;

impl MazeAlgorithm for PrimsAlgorithm {
    fn generate(&self, cells: &mut Vec<Vec<MazeCell>>, _walls: &mut Vec<Rc<RefCell<MazeWall>>>) {
        #[derive(Clone, Copy, Debug)]
        struct Point {
            pub x: usize,
            pub y: usize,
        }
        
        /*
        1. Start with a grid full of walls.
        2. Pick a cell, mark it as part of the maze. Add the walls of the cell to the wall list.
        3. While there are walls in the list:
            1. Pick a random wall from the list. If only one of the two cells that the wall divides is visited, then:
                1. Make the wall a passage and mark the unvisited cell as part of the maze.
                2. Add the neighboring walls of the cell to the wall list.
            2. Remove the wall from the list.
        */

        let width = cells.len();
        let height = cells[0].len();

        let rand = Point {
            x: rand::random::<usize>() % width,
            y: rand::random::<usize>() % height,
        };

        let mut visited = vec![vec![false; height]; width];
        visited[rand.x][rand.y] = true;

        let mut wall_list: Vec<&Rc<RefCell<MazeWall>>> = Vec::new();

        //Adds indexes that aren't border walls
        if rand.x != 0 { 
            wall_list.push(cells[rand.x][rand.y].left_wall_mut());
        }
        if rand.x != width - 1 { 
            wall_list.push(cells[rand.x][rand.y].right_wall_mut());
        }
        if rand.y != 0 { 
            wall_list.push(cells[rand.x][rand.y].top_wall_mut());
        }
        if rand.y != height - 1 { 
            wall_list.push(cells[rand.x][rand.y].bottom_wall_mut());
        }

        while wall_list.len() > 0 {
            let random_index = rand::random::<usize>() % wall_list.len();

            let mut cell_one = Point { x: usize::MAX, y: usize::MAX, };
            let mut cell_two = Point { x: usize::MAX, y: usize::MAX, };

            for i in 0..width {
                for j in 0..height {
                    if (i == cell_one.x) && (j == cell_one.y) { continue; }
                    
                    if Rc::ptr_eq(wall_list[random_index], cells[i][j].bottom_wall_mut()) {
                        cell_one = Point { x: i, y: j, };
                        cell_two = Point { x: i, y: j + 1, };
                        break;
                    }
                    else if Rc::ptr_eq(wall_list[random_index], cells[i][j].right_wall_mut()) {
                        cell_one = Point { x: i, y: j, };
                        cell_two = Point { x: i + 1, y: j, };
                        break;
                    }
                    else if Rc::ptr_eq(wall_list[random_index], cells[i][j].left_wall_mut()) {
                        cell_one = Point { x: i, y: j, };
                        cell_two = Point { x: i - 1, y: j, };
                        break;
                    }
                    else if Rc::ptr_eq(wall_list[random_index], cells[i][j].top_wall_mut()) {
                        cell_one = Point { x: i, y: j, };
                        cell_two = Point { x: i, y: j - 1, };
                        break;
                    }
                }

                if cell_two.x != usize::MAX { break; }
            }

            let mut add_neighboring_walls = |cell: Point, visited: &mut Vec<Vec<bool>>| {
                wall_list[random_index].borrow_mut().active = false;
                visited[cell.x][cell.y] = true;

                let mut new_walls: Vec<&Rc<RefCell<MazeWall>>> = Vec::new();

                if cell.x != 0 && !Rc::ptr_eq(wall_list[random_index], cells[cell.x][cell.y].left_wall_mut()) {
                    new_walls.push(cells[cell.x][cell.y].left_wall_mut());
                }

                if cell.x != width - 1 && !Rc::ptr_eq(wall_list[random_index], cells[cell.x][cell.y].right_wall_mut()) {
                    new_walls.push(cells[cell.x][cell.y].right_wall_mut());
                }

                if cell.y != 0 && !Rc::ptr_eq(wall_list[random_index], cells[cell.x][cell.y].top_wall_mut()) {
                    new_walls.push(cells[cell.x][cell.y].top_wall_mut());
                }

                if cell.y != height - 1 && !Rc::ptr_eq(wall_list[random_index], cells[cell.x][cell.y].bottom_wall_mut()) {
                    new_walls.push(cells[cell.x][cell.y].bottom_wall_mut());
                }

                wall_list.append(&mut new_walls);
            };

            if !visited[cell_one.x][cell_one.y] {
                add_neighboring_walls(cell_one, &mut visited);
            }
            else if !visited[cell_two.x][cell_two.y] {
                add_neighboring_walls(cell_two, &mut visited);
            }

            wall_list.remove(random_index);
        }

        //Set up maze exits
        cells[0][0].left_wall_mut().borrow_mut().active = false;
        cells[width - 1][height - 1].right_wall_mut().borrow_mut().active = false;
    }
}

pub struct KruskalsAlgorithm;

impl MazeAlgorithm for KruskalsAlgorithm {
    fn generate(&self, cells: &mut Vec<Vec<MazeCell>>, walls: &mut Vec<Rc<RefCell<MazeWall>>>) {
        //Step 1: Setup
        let width = cells.len();
        let height = cells[0].len();

        let n = walls.len();
        let mut rand_nums: Vec<usize> = Vec::with_capacity(n);
        let mut random: usize;

        //Step 1.1: create a vector of numbers in a random order        

        //Populate vector with numbers from 0 to n - 1
        for i in 0..n {
            rand_nums.push(i);
        }

        //Fisher-Yates shuffle for randomization
        for i in 0..(n - 1) {
            //j is random int i <= random < n
            random = (rand::random::<usize>() % (n - i)) + i;
            //exchange a[i] and a[j]
            //cannot use mem::swap because cannot borrow same vector twice
            let tmp = rand_nums[i];
            rand_nums[i] = rand_nums[random];
            rand_nums[random] = tmp;
        }

        //Step 1.2: create a vector for each cell, containing only that one cell
        let mut cell_sets: Vec<Vec<&MazeCell>> = Vec::with_capacity(width * height);

        for i in 0..width {
            for j in 0..height {
                cell_sets.push(Vec::new());
                cell_sets[(i * height) + j].push(&cells[i][j]);
            }
        }

        /*
        1. Create a list of all walls, and create a set for each cell, each containing just that one cell.
        2. For each wall, in some random order:
            1. If the cells divided by this wall belong to distinct sets:
                1. Remove the current wall.
                2. Join the sets of the formerly divided cells.
        */

        //Step 2
        //for each wall that exists
        let mut set_one_index: Option<usize>;
        let mut set_two_index: Option<usize>;

        for i in rand_nums.iter() {
            if cell_sets.len() == 1 { break; }

            set_one_index = None;
            set_two_index = None;

            //loops through every set to find the two sets that contain the two cells that the wall divides
            for set_index in 0..cell_sets.len() {
                for cell in &cell_sets[set_index] {
                    

                    if  (Rc::ptr_eq(&walls[*i], cell.top_wall_mut())) ||
                        (Rc::ptr_eq(&walls[*i], cell.bottom_wall_mut())) ||
                        (Rc::ptr_eq(&walls[*i], cell.left_wall_mut())) ||
                        (Rc::ptr_eq(&walls[*i], cell.right_wall_mut())) {

                        if set_one_index == None { //tests to see if the index is an actual index
                            set_one_index = Some(set_index);
                        }
                        else {
                            set_two_index = Some(set_index);
                        }
                        break;
                    }
                }
                if set_two_index != None { break; }
            }

            match set_two_index {
                //if the two cells sharing the wall did not pertain to two different sets 
                //then the wall will not be removed and the sets not joined
                None => continue,

                Some(sti) => {
                    //shared wall is broken
                    walls[*i].borrow_mut().active = false;

                    //sets combined
                    //cannot use vec::append because cannot borrow cell_sets as mut twice, have to do manually
                    let mut tmp = cell_sets.remove(sti);
                    if let Some(soi) = set_one_index {
                        cell_sets[soi].append(&mut tmp);
                    }   
                }
            }
        }

        //Set up maze exits
        cells[0][0].left_wall_mut().borrow_mut().active = false;
        cells[width - 1][height - 1].right_wall_mut().borrow_mut().active = false;
    }
}