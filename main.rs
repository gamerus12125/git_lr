use std::io::stdin;

fn calc(n1: i32, n2: i32) -> i32 {
    return n1 * n2;
}

fn main() {
    println!("Введите первое число");
    let mut s1: String = String::new();
    let mut s2: String = String::new();
    stdin().read_line(&mut s1).unwrap();
    stdin().read_line(&mut s2).unwrap();
    let num1: i32 = s1.strip_suffix("\n").unwrap().parse().unwrap();
    let num2: i32 = s2.strip_suffix("\n").unwrap().parse().unwrap();
    println!("Деление {}", num1 / num2);
}
