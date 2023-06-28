use std::io;

fn main() {
    let mut get_num1 = String::new();
    let mut get_num2 = String::new();

    io::stdin().read_line(&mut get_num1).expect("입력받지 못했습니다.");
    let get_num1: i32 = match get_num1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("숫자를 입력해주세요");
            return;
        }
    };

    io::stdin().read_line(&mut get_num2).expect("입력받지 못했습니다.");
    let get_num2: i32 = match get_num2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("숫자를 입력해주세요");
            return;
        }
    };

    print!("1. 더하기\n2. 빼기\n3. 곱하기\n4. 나누기\n");

    let mut menu_num = String::new();

    io::stdin().read_line(&mut menu_num).expect("입력받지 못했습니다.");
    let menu_num: u32 = match menu_num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("숫자를 입력해주세요");
            return;
        }
    };

    match menu_num {
        1 => println!("결과: {}", get_num1 + get_num2),
        2 => println!("결과: {}", get_num1 - get_num2),
        3 => println!("결과: {}", get_num1 * get_num2),
        4 => {
            if get_num2 == 0 {
                println!("0으로 나눌 수 없습니다");
            } {
                let result = get_num1 as f64 / get_num2 as f64;
                println!("결과: {}", result);
            }
        }
        _ => {
            println!("올바르지 않은 번호입니다.");
        }
    };
}
