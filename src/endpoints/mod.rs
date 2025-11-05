use std::cmp::min;
use thiserror::Error;
use typed_builder::TypedBuilder;

pub mod image;
pub mod random;

pub enum FileType {
    Jpeg,
    Webp,
}

impl FileType {
    fn as_string(&self) -> &'static str {
        match self {
            FileType::Jpeg => "jpg",
            FileType::Webp => "webp",
        }
    }
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum RequestError {
    #[error("Request error: {0}")]
    InvalidRequest(String),

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Image {
    id: String,
    image: Vec<u8>,
}

#[derive(TypedBuilder)]
pub struct ImageSettings {
    #[builder(setter(doc = "Set `width`."))]
    width: u16,
    #[builder(setter(doc = "Set `height`."))]
    height: u16,
    #[builder(
        default = false,
        setter(
            doc = "Set `grayscale`. Defines if the image should be grayscale. Defaults to false."
        )
    )]
    grayscale: bool,
    #[builder(
        default = 0,
        setter(
            doc = "Set `blur`. Defines the amount of blur between 0-10. Defaults to no blur (0)."
        )
    )]
    blur: u8,
    #[builder(
        default = FileType::Jpeg,
        setter(
            doc = "Set `file_type`. Defines the file type of the requested image. Defaults to no jpeg."
        )
    )]
    file_type: FileType,
}

impl ImageSettings {
    pub fn get_blur_value(&self) -> u8 {
        min(10, self.blur)
    }

    pub fn has_blur(&self) -> bool {
        self.blur > 0
    }
}
