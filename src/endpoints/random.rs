use crate::endpoints::RequestError::{InvalidRequest, ServerError, UnexpectedError};
use crate::endpoints::{Image, ImageSettings, RequestError};
use crate::{BASE_URL, PicsumClient};
use reqwest::StatusCode;

impl PicsumClient {
    /// Retrieve a random image with the given settings
    ///
    /// # Examples
    ///
    /// ```
    /// use picsum_rs::PicsumClient;
    /// use picsum_rs::endpoints::ImageSettings;
    ///
    /// # tokio_test::block_on(async {
    /// # let result =
    /// // Retrieve a random 400x400px image.
    /// match PicsumClient::default()
    ///     .get_random_image(ImageSettings::builder().width(400).height(400).build())
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
    /// # assert!(image.image.len() > 0);
    /// # assert!(image.id.len() > 0)
    /// # })
    /// ```
    pub async fn get_random_image(
        &self,
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
                "{}/{}/{}.{}",
                BASE_URL,
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
