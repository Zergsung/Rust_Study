use std::io;

fn main() {
    let mut get_num1 = String::new();
    let mut get_num2 = String::new();
    
    io::stdin().read_line(&mut get_num1).expect("입력받지 못했습니다.");
    let _get_num1: u32 = match get_num1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("숫자를 입력해주세요");
            return;
        }
    };

    io::stdin().read_line(&mut get_num2).expect("입력받지 못했습니다.");
    let _get_num1: u32 = match get_num2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("숫자를 입력해주세요");
            return;
        }
    };

    print!("1. 더하기\n2. 빼기\n3. 곱하기\n4. 나누기\n");
    
    let mut menu_num = String::new();
    
    io::stdin().read_line(&mut menu_num).expect("입력받지 못했습니다.");
    let _menu_num: u32 = match menu_num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("숫자를 입력해주세요");
            return;
        }
    };
    
    
}
