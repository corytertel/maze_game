pub struct MazeWall {
    pub active: bool,
}

impl MazeWall {
    pub fn new(active: bool) -> Self {
        Self {
            active,
        }
    }
}