use crate::environment::Vector2;
mod environment;

fn main() {
    let data = Vector2::new(0, 1);

    println!("{}:{}", &data.x, &data.y);
}
