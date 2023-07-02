use std::{io, process};
// ipad test
fn main() {
    loop{
        let mut input = String::new();

        println!("공백으로 숫자를 구분해서 입력해주세요.(ex: 1 2)");

        let (num1, num2) = loop {
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            // white space로 나눠서 리스트 생성
            let mut nums_iter = input.split_whitespace(); 

            // 각 변수에 입력 받은 숫자 공백으로 구분해서 선언
            let get_num1: Result<i32, _> = nums_iter.next().unwrap().parse();
            let get_num2: Result<i32, _> = nums_iter.next().unwrap().parse();

            // 변수에 숫자 말고 다른 타입이 입력됐는지 검증
            match (get_num1, get_num2) {
                (Ok(num1), Ok(num2)) => break (num1, num2),
                _ => {
                    println!("유효하지 않은 입력입니다. 다시 입력해주세요.");
                    continue;
                }
            }
        };

        print!("0. 종료\n1. 더하기\n2. 빼기\n3. 곱하기\n4. 나누기\n");

        let mut menu_num = String::new();

        // type check
        io::stdin().read_line(&mut menu_num).expect("입력받지 못했습니다.");
        let menu_num: u32 = match menu_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("숫자를 입력해주세요");
                return;
            }
        };

        // select menu
        match menu_num {
            0 => {
                println!("프로그램을 종료합니다.");
                process::exit(0);
            }
            1 => println!("결과: {}", num1 + num2),
            2 => println!("결과: {}", num1 - num2),
            3 => println!("결과: {}", num1 * num2),
            4 => {
                if num2 == 0 {
                    println!("0으로 나눌 수 없습니다");
                } else {
                    let result = num1 as f64 / num2 as f64;
                    println!("결과: {}", result);
                }
            }
            _ => {
                println!("올바르지 않은 번호입니다.");
            }
        }
    }
}
