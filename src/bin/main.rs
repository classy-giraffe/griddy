use griddy::prelude::*;

fn main() {
    let image = Png::new(r"./samples/sample.png").unwrap();
    println!("{}", image)
}
