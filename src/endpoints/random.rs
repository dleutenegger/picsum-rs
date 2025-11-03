use crate::endpoints::RequestError::{InvalidRequest, ServerError, UnexpectedError};
use crate::endpoints::{FileType, Image, RequestError};
use crate::{BASE_URL, PicsumClient};
use reqwest::StatusCode;
use std::cmp::min;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct RandomImageParameters {
    width: u16,
    height: u16,
    #[builder(default = false)]
    grayscale: bool,
    #[builder(default = 0)]
    blur: u8,
    #[builder(default=FileType::Jpeg)]
    file_type: FileType,
}

impl PicsumClient {
    /// Retrieve a random image with the given settings
    ///
    /// # Examples
    ///
    /// ```
    /// use picsum_rs::endpoints::random::RandomImageParameters;
    /// use picsum_rs::PicsumClient;
    ///
    /// // Retrieve a random 400x400px image.
    /// PicsumClient::default()
    ///     .get_random_image(
    ///         RandomImageParameters::builder()
    ///             .width(400)
    ///             .height(400)
    ///             .build()
    ///     );
    /// ```
    pub async fn get_random_image(
        &self,
        parameters: RandomImageParameters,
    ) -> Result<Image, RequestError> {
        let mut query_params = vec![("grayscale", parameters.grayscale.to_string())];
        if parameters.blur > 0 {
            query_params.push(("blur", min(10, parameters.blur).to_string()))
        }

        let response = self
            .inner
            .client
            .get(format!(
                "{}/{}/{}.{}",
                BASE_URL,
                parameters.width,
                parameters.height,
                parameters.file_type.as_string()
            ))
            .query(&query_params)
            .send()
            .await;

        match response {
            Ok(r) => match r.error_for_status() {
                Ok(res) => {
                    let id = match res.headers().get("picsum-id") {
                        None => {
                            return Err(UnexpectedError(
                                "Couldn't retrieve `picsum-id` header.".into(),
                            ));
                        }
                        Some(v) => match v.to_str() {
                            Ok(value) => value,
                            Err(e) => return Err(UnexpectedError(e.to_string())),
                        },
                    };

                    Ok(Image {
                        id: id.into(),
                        image: match res.bytes().await {
                            Ok(bytes) => bytes.to_vec(),
                            Err(err) => {
                                return Err(UnexpectedError(format!(
                                    "Couldn't read response body: {}",
                                    err
                                )));
                            }
                        },
                    })
                }
                Err(err) => match err.status() {
                    Some(StatusCode::BAD_REQUEST) => Err(InvalidRequest(err.to_string())),
                    Some(StatusCode::INTERNAL_SERVER_ERROR) => Err(ServerError(err.to_string())),
                    Some(code) => Err(UnexpectedError(format!("{} {}", code, err))),
                    None => Err(UnexpectedError(err.to_string())),
                },
            },
            Err(err) => Err(UnexpectedError(err.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retrieve() {
        let client = PicsumClient::default();

        let response = client
            .get_random_image(
                RandomImageParameters::builder()
                    .width(400)
                    .height(400)
                    .build(),
            )
            .await;

        assert!(
            response.is_ok(),
            "Random image request failed: {}",
            response.unwrap_err().to_string()
        );
        let image = response.unwrap();
        assert!(image.image.len() > 0);
        assert!(image.id.len() > 0)
    }
}
