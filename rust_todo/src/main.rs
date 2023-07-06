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
        println!("메뉴 번호를 선택해주세요:");
        println!("1 - 할 일 추가");
        println!("2 - 할 일 제거");
        println!("3 - 할 일 목록");
        println!("4 - 종료");

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

                println!("제목을 입력해주세요:");
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read input");

                println!("내용을 입력해주세요:");
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
                println!("할일이 추가 되었습니다. Todo ID : {}\n", id_counter);

                id_counter += 1;
            }
            2 => {
                println!("삭제할 Todo ID를 입력해주세요:");
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
                    println!("Todo ID {}번을 삭제했습니다.", id_input);
                } else {
                    println!("Todo ID {}번을 찾을 수 없습니다.", id_input);
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
