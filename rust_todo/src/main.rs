use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Todo {
    title: String,
    content: String,
}

fn main() {
    let mut todos: HashMap<u32, Todo> = HashMap::new();
    let mut id_counter: u32 = 1;

    loop {
        println!("Choose an option:");
        println!("1 - Add a new todo");
        println!("2 - Remove a todo");
        println!("3 - List todos");
        println!("4 - Quit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                let mut title = String::new();
                let mut content = String::new();

                println!("Enter the title of the todo:");
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read input");

                println!("Enter the content of the todo:");
                io::stdin()
                    .read_line(&mut content)
                    .expect("Failed to read input");

                todos.insert(
                    id_counter,
                    Todo {
                        title: title.trim().to_string(),
                        content: content.trim().to_string(),
                    },
                );
                println!("Todo added with ID: {}", id_counter);

                id_counter += 1;
            }
            2 => {
                println!("Enter the ID of the todo to delete:");
                let mut id_input = String::new();
                io::stdin()
                    .read_line(&mut id_input)
                    .expect("Failed to read input");

                let id_input: u32 = match id_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID");
                        continue;
                    }
                };

                if todos.remove(&id_input).is_some() {
                    println!("Todo with ID {} deleted", id_input);
                } else {
                    println!("No todo found with ID {}", id_input);
                }
            }
            3 => {
                println!("--- Todos ---");
                for (id, todo) in &todos {
                    println!("ID: {}", id);
                    println!("Title: {}", todo.title);
                    println!("Content: {}\n", todo.content);
                }
            }
            4 => break,
            _ => {
                println!("Invalid option");
            }
        }
    }
}
