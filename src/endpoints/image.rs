use crate::endpoints::RequestError::{InvalidRequest, ServerError, UnexpectedError};
use crate::endpoints::{Image, ImageSettings, RequestError};
use crate::{BASE_URL, PicsumClient};
use reqwest::StatusCode;

impl PicsumClient {
    /// Retrieve a specific image by its id.
    ///
    /// # Examples
    ///
    /// ```
    /// use picsum_rs::endpoints::ImageSettings;
    /// use picsum_rs::PicsumClient;
    ///
    /// // Retrieve the image with the id `1` in the size 400x400px.
    /// PicsumClient::default()
    ///     .get_image(
    ///         "1",
    ///         ImageSettings::builder()
    ///             .width(400)
    ///             .height(400)
    ///             .build(),
    ///     );
    ///
    /// ```
    pub async fn get_image(
        &self,
        id: &str,
        image_settings: ImageSettings,
    ) -> Result<Image, RequestError> {
        let mut query_params = vec![("grayscale", image_settings.grayscale.to_string())];
        if image_settings.has_blur() {
            query_params.push(("blur", image_settings.get_blur_value().to_string()))
        }

        let response = self
            .inner
            .client
            .get(format!(
                "{}/id/{}/{}/{}.{}",
                BASE_URL,
                id,
                image_settings.width,
                image_settings.height,
                image_settings.file_type.as_string()
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
    async fn test_retrieve_image_with_id() {
        let client = PicsumClient::default();

        let response = client
            .get_image("1", ImageSettings::builder().width(400).height(400).build())
            .await;

        assert!(
            response.is_ok(),
            "Retrieving the image with the id 1 failed: {}",
            response.unwrap_err().to_string()
        );
        let image = response.unwrap();
        assert_eq!(
            "1".to_string(),
            image.id,
            "Expected image id to be `1`, actually: {}",
            image.id
        );
        assert!(image.image.len() > 0);
    }
}
