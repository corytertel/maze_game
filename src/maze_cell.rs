use std::rc::Rc;
use std::cell::{RefCell, Ref};

use super::maze_wall::MazeWall;

pub struct MazeCell {
    top_wall: Option<Rc<RefCell<MazeWall>>>,
    bottom_wall: Option<Rc<RefCell<MazeWall>>>,
    left_wall: Option<Rc<RefCell<MazeWall>>>,
    right_wall: Option<Rc<RefCell<MazeWall>>>,
}

impl MazeCell {
    pub fn new() -> Self {
        Self {
            top_wall: None,
            bottom_wall: None,
            left_wall: None,
            right_wall: None,
        }
    }

    //Getters
    pub fn top_wall(&self) -> Ref<'_, MazeWall> { 
        match &self.top_wall {
            Some(value) => value.borrow(),
            None => panic!("MazeCell was not properly constructed."),
        }
    }

    pub fn bottom_wall(&self) -> Ref<'_, MazeWall> { 
        match &self.bottom_wall {
            Some(value) => value.borrow(),
            None => panic!("MazeCell was not properly constructed."),
        }
    }

    pub fn left_wall(&self) -> Ref<'_, MazeWall> { 
        match &self.left_wall {
            Some(value) => value.borrow(),
            None => panic!("MazeCell was not properly constructed."),
        }
    }

    pub fn right_wall(&self) -> Ref<'_, MazeWall> { 
        match &self.right_wall {
            Some(value) => value.borrow(),
            None => panic!("MazeCell was not properly constructed."),
        }
    }

    //Mutable Access
    pub fn top_wall_mut(&self) -> &Rc<RefCell<MazeWall>> { 
        match &self.top_wall {
            Some(value) => value,
            None => panic!("MazeCell was not properly constructed."),
        }
    }

    pub fn bottom_wall_mut(&self) -> &Rc<RefCell<MazeWall>> { 
        match &self.bottom_wall {
            Some(value) => value,
            None => panic!("MazeCell was not properly constructed."),
        }
    }

    pub fn left_wall_mut(&self) -> &Rc<RefCell<MazeWall>> { 
        match &self.left_wall {
            Some(value) => value,
            None => panic!("MazeCell was not properly constructed."),
        }
    }

    pub fn right_wall_mut(&self) -> &Rc<RefCell<MazeWall>> { 
        match &self.right_wall {
            Some(value) => value,
            None => panic!("MazeCell was not properly constructed."),
        }
    }

    //Setters
    pub fn set_top_wall(&mut self, wall: Rc<RefCell<MazeWall>>) { self.top_wall = Some(wall); }
    pub fn set_bottom_wall(&mut self, wall: Rc<RefCell<MazeWall>>) { self.bottom_wall = Some(wall); }
    pub fn set_left_wall(&mut self, wall: Rc<RefCell<MazeWall>>) { self.left_wall = Some(wall); }
    pub fn set_right_wall(&mut self, wall: Rc<RefCell<MazeWall>>) { self.right_wall = Some(wall); }
}

impl Clone for MazeCell {
    fn clone(&self) -> MazeCell {
        MazeCell {
            top_wall: self.top_wall.clone(),
            bottom_wall: self.bottom_wall.clone(),
            left_wall: self.left_wall.clone(),
            right_wall: self.right_wall.clone(),
        }
    }
}