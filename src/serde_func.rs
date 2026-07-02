use crate::todo_struct::Todo;
use std::fs;
use std::io::ErrorKind;

const FILE_PATH: &str = "todos.json";
pub fn save_todos(todos: &Vec<Todo>) {
    let json = serde_json::to_string_pretty(todos)
        .expect("Error serializing json");

    fs::write(FILE_PATH, json)
        .expect("Error writing file");
}

pub fn load_todos() -> Vec<Todo> { // since i expect return type and i do not return anything it was angry for me now i return something and it wont
    let data = match fs::read_to_string(FILE_PATH) {
        Ok(data) => data,

        // we catch error here but specificly if the file is NotFound
        Err(error) if error.kind() == ErrorKind::NotFound => {
            fs::write(FILE_PATH, "[]")
                .expect("Error while creating file");

            return Vec::new();
        }

        // so because of we are catching specific error at top rust ask what if some other thing happens?
        Err(why) => panic!("Error while opening file: {}", why),
    };

    if data.trim().is_empty() {
        return Vec::new();
    }


    serde_json::from_str(&data).expect("Error parsing json")
}