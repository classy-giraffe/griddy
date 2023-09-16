#[cfg(test)]
mod tests {
    use griddy::prelude::*;

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

    #[test]
    fn test_ihdr() {
        let test_ihdr = IHDRChunk::new((850, 566), ColorLayout::Rgb8, 0, 0, 0);
        let image = Image::new(r"./samples/sample.png").unwrap();
        assert_eq!(image.ihdr_parse().unwrap(), test_ihdr);
    }
}
