fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const MAX_POINTS: u32 = 450;
    println!("The value of MAX_POINTS is {}", MAX_POINTS);

    println!("Shadowing");
    let x = 7;
    println!("The value of x is {}", x);
    let x = x+7;
    println!("The value of x is {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The length of spaces is {}", spaces);
}
