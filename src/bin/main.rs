use griddy::prelude::*;

fn main() {
    let image = Png::new(r"./samples/big_sample.png").unwrap();
    println!("{}", image)
}
