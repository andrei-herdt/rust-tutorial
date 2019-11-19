fn main() {
    let rect = Rectangle{
        width: 1.,
        height: 2.
    };

    let area = compute_width(&rect);

    println!("The are of rect is: {}", area);

    let rect1 = Rectangle { width: 30., height: 50. };
    let rect2 = Rectangle { width: 10., height: 40. };
    let rect3 = Rectangle { width: 60., height: 45. };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn compute_width(r: &Rectangle) -> f32
{
    r.width*r.height
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
