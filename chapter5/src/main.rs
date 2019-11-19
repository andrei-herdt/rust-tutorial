fn main() {
    let rect = Rectangle{
        width: 1.,
        height: 2.
    };

    let area = compute_width(&rect);

    println!("The are of rect is: {}", area);
}

fn compute_width(r: &Rectangle) -> f32
{
    r.width*r.height
}

struct Rectangle {
    width: f32,
    height: f32,
}
