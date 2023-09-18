use griddy::prelude::*;

fn main() {
    let image = Image::new(r"./samples/sample.png").unwrap();
    for chunk in image {
        println!("{}", chunk);
    }
}
