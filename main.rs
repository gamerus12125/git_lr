use core::num;
use std::io::stdin;

fn calc(n1: i32, n2: i32) -> i32 {
    return n1 * n2;
}

fn calc1(n1: i32, n2: i32) -> f32 {
    return n1 as f32 / n2 as f32;
}

fn main() {
    println!("Введите первое число");
    let mut s1: String = String::new();
    let mut s2: String = String::new();
    stdin().read_line(&mut s1).unwrap();
    stdin().read_line(&mut s2).unwrap();
    let num1: i32 = s1.strip_suffix("\n").unwrap().parse().unwrap();
    let num2: i32 = s2.strip_suffix("\n").unwrap().parse().unwrap();
    println!("Деление {}", calc1(num1, num2));
    println!("Умножение двух чисел {}", calc(num1, num2));
}
