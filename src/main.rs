mod chunk;
mod image;
mod pixel;

use crate::image::Image;


fn main() {
    let image = Image::new(r".\samples\sample.png");
    image.parse_png();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_png() {
        let image = Image::new(r".\samples\sample.png");
        assert!(image.is_png());
    }
}
