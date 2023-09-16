use griddy::prelude::*;

fn main() {
    let image = Image::new(r"./samples/sample.png").unwrap();
    println!("{}", image.ihdr_parse().unwrap());
}
