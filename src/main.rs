use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    loop{
        println!("Угадайте число");
        let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
        println!("Секретное число: {}", secret_number);
        println!("Введите догадку: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Не получилось прочитать строку");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Вы загадали: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком малое число.\n"),
            Ordering::Greater => println!("Слишком большое число.\n"),
            Ordering::Equal => {
                println!("Верное число!\n");
                break;
            }
        }
    }
}
