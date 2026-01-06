fn main() {
    let mut x = 5;

    const MAX_POINT: u32 = 100_000;

    println!("Значение x равно {}", x);
    x = 6;
    println!("Значение x равно {}", x);
    x = MAX_POINT;
    println!("Const = {}", x);

    let a = 5;

    let a = a + 1;

    let a = a * 2;

    println!("Значение a равно {}", a);

    let spaces = " ";
    let spaces = spaces.len();
    println!("Num {}", spaces);

    let mut spaces = " ";
    // spaces = spaces.len(); Нельзя менять тип переменной таким образом

    let guess: u32 = "42".parse().expect("Не является числом!"); //При изменении типов аннотация необходима
    println!("{}", guess)


}
