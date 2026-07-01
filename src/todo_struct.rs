pub struct Todo {
    pub id: u32,
    pub title: String,
    pub task: String,
    pub status: bool,
}


impl Todo {
    pub fn new(id: u32, title: String, task: String) -> Todo {
        Todo {
            id,
            title,
            task,
            status: false,
        }
    }

    pub fn mark_done(&mut self) {
        self.status = true;
    }
}