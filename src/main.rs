use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Угадайте число");
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
    println!("Секретное число: {}", secret_number);
    println!("Введите догадку: ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Не получилось прочитать строку");
    println!("Вы загадали: {}", guess);
    let guess: u32 = guess.trim().parse().expect("Введите число.");
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Слишком малое число."),
        Ordering::Greater => println!("Слишком большое число."),
        Ordering::Equal => println!("Верное число!"),
    }
}
