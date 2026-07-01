use crate::read_input;
use crate::todo_struct::Todo;

pub fn select_todo_index (todos: &Vec<Todo>) -> Option<usize> {
    if todos.is_empty() {
        println!("There are no todos");
        return None;
    }


    for (index, todo) in todos.iter().enumerate() {
        println!(
            "{}) {} - {} - Done: {}",
            index + 1,
            todo.title,
            todo.task,
            todo.status
        );
    }

    let choice = read_input("Choose Todo NUmber");

    let choice: usize = match choice.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("There was a problem opening the input");
            return None;
        }
    };

    if choice == 0 || choice > todos.len() {
        println!("There was a problem opening the input");
        return None;
    }

    Some(choice -1)

}