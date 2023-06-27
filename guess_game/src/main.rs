use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("==== 숫자 맞추기 게임! ====");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count_number = 0;
    println!("숫자의 범위는 1 ~ 100입니다.");
    loop {
        println!("숫자를 입력해주세요.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("업!"),
            Ordering::Greater => println!("다운!"),
            Ordering::Equal => {
                println!("Yes");
                break;
            }
        }

        count_number += 1;
        println!("({}번 남았습니다.)\n", 7 - count_number);

        if count_number == 7 {
            println!("Failed to guess number");
            println!("{}입니다.", secret_number);
            break;
        }
    }
}