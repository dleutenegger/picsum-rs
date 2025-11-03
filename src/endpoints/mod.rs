use thiserror::Error;

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
