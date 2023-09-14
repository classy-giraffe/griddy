use griddy::image::Image;

fn main() {
    let image = Image::new(r"./samples/sample.png").unwrap().ihdr_parse();
    println!("ihdr: {}", image);
}

#[cfg(test)]
mod tests {
    use super::*;
    use griddy::errors::FileError;

    #[test]
    fn test_is_png() {
        let image = Image::new(r"./samples/sample.png");
        assert!(image.is_ok());
    }

    #[test]
    fn test_file_not_found() {
        let image = Image::new(r"./samples/sample2.png");
        assert_eq!(image.unwrap_err(), FileError::FileNotFound.into());
    }

    #[test]
    fn test_not_a_png() {
        let image = Image::new(r"./samples/sample.jpg");
        assert_eq!(image.unwrap_err(), FileError::NotAPng.into());
    }
}
