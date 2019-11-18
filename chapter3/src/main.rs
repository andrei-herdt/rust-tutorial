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

    let array = [1,2,3,4,5];
    println!("The first element of array is {}", array[0]);

    another_function(5);

    let y = {
        let x = 4;
        x + 1
    };
    println!("The value of y is {}", y);

    println!("The return value of five is {}", five());

    let array = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value at index {} is {}", index, array[index]);
        index = index + 1;
    }

    for x in (1..4).rev() {
        println!("x: {}", x);
    }

    let name = String::from("Andrei Herdt");
    println!("Full name is: {}", name);
    let first_name = take_first_word(&name[..]);
    println!("First name is: {}", first_name);

}

fn take_first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}
fn another_function(x: i32)
{
    println!("The input to another_function is {}", x);
}

fn five () -> i32 {
    5
}
