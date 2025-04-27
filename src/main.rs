use std::io;
use rand::Rng;


fn main() {
    println!("Угадайте число");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Секретное число: {}", secret_number);
    println!("Введите догадку: ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Не получилось прочитать строку");
    println!("Вы загадали: {}", guess);
}
