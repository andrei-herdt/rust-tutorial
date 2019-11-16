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

    let heart_eyed_cat = '😻';
    println!("The heart_eyed_cat is {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 500.1, 10);
    println!("The tuple is x:{},y:{},z:{}", tup.0,tup.1,tup.2);
    let (x,y,z) = tup;
    println!("The tuple is x:{},y:{},z:{}", x,y,z);
}
