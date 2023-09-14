use griddy::image::Image;
use griddy::image::FileError;

fn main() {
    let image = Image::new(r".\samples\sample.png").unwrap();
    println!("{}", image);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_png() {
        let image = Image::new(r".\samples\sample.png");
        assert!(image.is_ok());
    }

    #[test]
    fn test_file_not_found() {
        let image = Image::new(r".\samples\sample2.png");
        assert_eq!(image.unwrap_err(), FileError::FileNotFound);
    }
}
