use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    loop{
        let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
        println!("Новая игра! Угадайте число");
        let game_over: bool = false;
        let mut attempts: u32 = 0;
        while !game_over{
            //println!("Секретное число: {}", secret_number);
            println!("Введите догадку: ");
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Не получилось прочитать строку");
            let guess: u32 = match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("Вы загадали: {}", guess);
            attempts += 1;
            println!("{} попытка.", &attempts);
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Слишком малое число.\n"),
                Ordering::Greater => println!("Слишком большое число.\n"),
                Ordering::Equal => {
                    println!("\nПоздравляю! Верное число!\nЭто заняло у вас {} попыток", &attempts);
                    break;
                }
            }
        }
    }
}
