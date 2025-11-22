use crate::PicsumClient;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::cmp::min;
use thiserror::Error;
use typed_builder::TypedBuilder;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
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

#[derive(Error, Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
pub enum RequestError {
    #[error("Request error: {0}")]
    InvalidRequest(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub data: Vec<u8>,
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

    pub fn is_grayscale(&self) -> bool {
        self.grayscale
    }

    fn generate_query_params(&self) -> Vec<(&str, String)> {
        [
            self.is_grayscale()
                .then_some(("grayscale", "true".to_string())),
            self.has_blur()
                .then_some(("blur", self.get_blur_value().to_string())),
        ]
        .iter()
        .filter_map(|param| param.clone())
        .collect()
    }
}

impl Default for ImageSettings {
    fn default() -> Self {
        Self {
            width: 400,
            height: 400,
            grayscale: false,
            blur: 0,
            file_type: FileType::Jpeg,
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
pub struct ImageDetails {
    pub id: String,
    pub author: String,
    pub width: u16,
    pub height: u16,
    pub url: String,
    pub download_url: String,
}

impl PicsumClient {
    /// Retrieve image details of a specific image id.
    ///
    /// # Examples
    ///
    /// ```
    /// use picsum_rs::PicsumClient;
    /// # use picsum_rs::api::ImageDetails;
    ///
    /// # tokio_test::block_on(async {
    /// // Retrieve the image details for the image with the id `1`.
    /// # let result =
    /// match PicsumClient::default().get_image_details("1").await {
    ///     Ok(image_details) => {
    /// #       Ok(
    ///         image_details
    /// #       )
    ///     }
    ///     Err(e) => {
    ///         // Do your error handling
    ///         # Err(e)
    ///     }
    /// }
    /// # ;
    /// # assert!(
    /// #     result.is_ok(),
    /// #     "Retrieving the image details for the image with the id 1 failed: {}",
    /// #     result.unwrap_err().to_string()
    /// # );
    /// # let details = result.unwrap();
    /// # let expected_details = ImageDetails {
    /// #     id: "1".to_string(),
    /// #     author: "Alejandro Escamilla".to_string(),
    /// #     width: 5000,
    /// #     height: 3333,
    /// #     url: "https://unsplash.com/photos/LNRyGwIJr5c".to_string(),
    /// #     download_url: "https://picsum.photos/id/1/5000/3333".to_string(),
    /// # };
    /// # assert_eq!(expected_details, details);
    /// # });
    pub async fn get_image_details(&self, id: &str) -> Result<ImageDetails, RequestError> {
        let response = self
            .inner
            .client
            .get(format!("{}/id/{}/info", self.inner.base_url, id))
            .send()
            .await;

        match response {
            Ok(r) => match r.error_for_status() {
                Ok(res) => res
                    .json::<ImageDetails>()
                    .await
                    .map_err(|err| RequestError::InvalidResponse(err.to_string())),
                Err(err) => match err.status() {
                    Some(StatusCode::BAD_REQUEST) => {
                        Err(RequestError::InvalidRequest(err.to_string()))
                    }
                    Some(StatusCode::INTERNAL_SERVER_ERROR) => {
                        Err(RequestError::ServerError(err.to_string()))
                    }
                    Some(code) => Err(RequestError::UnexpectedError(format!("{} {}", code, err))),
                    None => Err(RequestError::UnexpectedError(err.to_string())),
                },
            },
            Err(err) => Err(RequestError::UnexpectedError(err.to_string())),
        }
    }

    /// Retrieve a list of available images.
    ///
    /// # Examples
    ///
    /// ```
    /// use picsum_rs::PicsumClient;
    ///
    /// # tokio_test::block_on(async {
    /// // Retrieve a list of images, fetching page 1 with a limit of 10 images per page.
    /// # let result =
    /// match PicsumClient::default().get_images(1, 10).await {
    ///     Ok(image_list) => {
    /// #       Ok(
    ///         image_list
    /// #       )
    ///     }
    ///     Err(e) => {
    ///         // Do your error handling
    ///         # Err(e)
    ///     }
    /// }
    /// # ;
    /// # assert!(
    /// #   result.is_ok(),
    /// #   "Retrieving page one with a limit of 10 images per page failed: {}",
    /// #   result.unwrap_err().to_string()
    /// # );
    /// # let page1 = result.unwrap();
    /// # assert_eq!(10, page1.len());
    /// # })
    /// ```
    pub async fn get_images(
        &self,
        page: u16,
        limit: u8,
    ) -> Result<Vec<ImageDetails>, RequestError> {
        let response = self
            .inner
            .client
            .get(format!("{}/v2/list", self.inner.base_url))
            .query(&vec![("page", page), ("limit", limit as u16)])
            .send()
            .await;

        match response {
            Ok(r) => match r.error_for_status() {
                Ok(res) => res
                    .json::<Vec<ImageDetails>>()
                    .await
                    .map_err(|err| RequestError::InvalidResponse(err.to_string())),
                Err(err) => match err.status() {
                    Some(StatusCode::BAD_REQUEST) => {
                        Err(RequestError::InvalidRequest(err.to_string()))
                    }
                    Some(StatusCode::INTERNAL_SERVER_ERROR) => {
                        Err(RequestError::ServerError(err.to_string()))
                    }
                    Some(code) => Err(RequestError::UnexpectedError(format!("{} {}", code, err))),
                    None => Err(RequestError::UnexpectedError(err.to_string())),
                },
            },
            Err(err) => Err(RequestError::UnexpectedError(err.to_string())),
        }
    }

    /// Retrieve a specific image by its id.
    ///
    /// # Examples
    ///
    /// ```
    /// use picsum_rs::PicsumClient;
    /// use picsum_rs::api::ImageSettings;
    ///
    /// # tokio_test::block_on(async {
    /// # let result =
    /// // Retrieve the image with the id `1` in the size 400x400px.
    /// match PicsumClient::default()
    ///     .get_image("1", &ImageSettings::builder().width(400).height(400).build())
    ///     .await
    /// {
    ///     Ok(image_list) => {
    /// #       Ok(
    ///         image_list
    /// #       )
    ///     }
    ///     Err(e) => {
    ///         // Do your error handling
    ///         # Err(e)
    ///     }
    /// }
    /// # ;
    /// # assert!(
    /// #    result.is_ok(),
    /// #    "Retrieving the image with the id 1 failed: {}",
    /// #    result.unwrap_err().to_string()
    /// # );
    /// # let image = result.unwrap();
    /// # assert_eq!(
    /// #    "1".to_string(),
    /// #    image.id,
    /// #    "Expected image id to be `1`, actually: {}",
    /// #    image.id
    /// # );
    /// # assert!(image.data.len() > 0);
    /// # })
    /// ```
    pub async fn get_image(
        &self,
        id: &str,
        image_settings: &ImageSettings,
    ) -> Result<Image, RequestError> {
        let response = self
            .inner
            .client
            .get(format!(
                "{}/id/{}/{}/{}.{}",
                self.inner.base_url,
                id,
                image_settings.width,
                image_settings.height,
                image_settings.file_type.as_string()
            ))
            .query(&image_settings.generate_query_params())
            .send()
            .await;

        match response {
            Ok(r) => match r.error_for_status() {
                Ok(res) => {
                    let id = match res.headers().get("picsum-id") {
                        None => {
                            return Err(RequestError::UnexpectedError(
                                "Couldn't retrieve `picsum-id` header.".into(),
                            ));
                        }
                        Some(v) => match v.to_str() {
                            Ok(value) => value,
                            Err(e) => return Err(RequestError::UnexpectedError(e.to_string())),
                        },
                    };

                    Ok(Image {
                        id: id.into(),
                        data: match res.bytes().await {
                            Ok(bytes) => bytes.to_vec(),
                            Err(err) => {
                                return Err(RequestError::UnexpectedError(format!(
                                    "Couldn't read response body: {}",
                                    err
                                )));
                            }
                        },
                    })
                }
                Err(err) => match err.status() {
                    Some(StatusCode::BAD_REQUEST) => {
                        Err(RequestError::InvalidRequest(err.to_string()))
                    }
                    Some(StatusCode::INTERNAL_SERVER_ERROR) => {
                        Err(RequestError::ServerError(err.to_string()))
                    }
                    Some(code) => Err(RequestError::UnexpectedError(format!("{} {}", code, err))),
                    None => Err(RequestError::UnexpectedError(err.to_string())),
                },
            },
            Err(err) => Err(RequestError::UnexpectedError(err.to_string())),
        }
    }

    /// Retrieve a random image with the given settings
    ///
    /// # Examples
    ///
    /// ```
    /// use picsum_rs::PicsumClient;
    /// use picsum_rs::api::ImageSettings;
    ///
    /// # tokio_test::block_on(async {
    /// # let result =
    /// // Retrieve a random 400x400px image.
    /// match PicsumClient::default()
    ///     .get_random_image(&ImageSettings::builder().width(400).height(400).build())
    ///     .await
    /// {
    ///     Ok(image_list) => {
    /// #       Ok(
    ///         image_list
    /// #       )
    ///     }
    ///     Err(e) => {
    ///         // Do your error handling
    ///         # Err(e)
    ///     }
    /// }
    /// # ;
    /// # assert!(
    /// #    result.is_ok(),
    /// #    "Random image request failed: {}",
    /// #    result.unwrap_err().to_string()
    /// # );
    /// # let image = result.unwrap();
    /// # assert!(image.data.len() > 0);
    /// # assert!(image.id.len() > 0)
    /// # })
    /// ```
    pub async fn get_random_image(
        &self,
        image_settings: &ImageSettings,
    ) -> Result<Image, RequestError> {
        let response = self
            .inner
            .client
            .get(format!(
                "{}/{}/{}.{}",
                self.inner.base_url,
                image_settings.width,
                image_settings.height,
                image_settings.file_type.as_string()
            ))
            .query(&image_settings.generate_query_params())
            .send()
            .await;

        match response {
            Ok(r) => match r.error_for_status() {
                Ok(res) => {
                    let id = match res.headers().get("picsum-id") {
                        None => {
                            return Err(RequestError::UnexpectedError(
                                "Couldn't retrieve `picsum-id` header.".into(),
                            ));
                        }
                        Some(v) => match v.to_str() {
                            Ok(value) => value,
                            Err(e) => return Err(RequestError::UnexpectedError(e.to_string())),
                        },
                    };

                    Ok(Image {
                        id: id.into(),
                        data: match res.bytes().await {
                            Ok(bytes) => bytes.to_vec(),
                            Err(err) => {
                                return Err(RequestError::UnexpectedError(format!(
                                    "Couldn't read response body: {}",
                                    err
                                )));
                            }
                        },
                    })
                }
                Err(err) => match err.status() {
                    Some(StatusCode::BAD_REQUEST) => {
                        Err(RequestError::InvalidRequest(err.to_string()))
                    }
                    Some(StatusCode::INTERNAL_SERVER_ERROR) => {
                        Err(RequestError::ServerError(err.to_string()))
                    }
                    Some(code) => Err(RequestError::UnexpectedError(format!("{} {}", code, err))),
                    None => Err(RequestError::UnexpectedError(err.to_string())),
                },
            },
            Err(err) => Err(RequestError::UnexpectedError(err.to_string())),
        }
    }
}
