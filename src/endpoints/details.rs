use crate::endpoints::RequestError;
use crate::endpoints::RequestError::{
    InvalidRequest, InvalidResponse, ServerError, UnexpectedError,
};
use crate::{BASE_URL, PicsumClient};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

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
    /// # use picsum_rs::endpoints::details::ImageDetails;
    /// use picsum_rs::PicsumClient;
    ///
    /// # tokio_test::block_on(async {
    /// // Retrieve the image details for the image with the id `1`.
    /// # let details =
    /// PicsumClient::default().get_image_details("1").await?;
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
    /// ```
    pub async fn get_image_details(&self, id: &str) -> Result<ImageDetails, RequestError> {
        let response = self
            .inner
            .client
            .get(format!("{}/id/{}/info", BASE_URL, id))
            .send()
            .await;

        match response {
            Ok(r) => match r.error_for_status() {
                Ok(res) => res
                    .json::<ImageDetails>()
                    .await
                    .map_err(|err| InvalidResponse(err.to_string())),
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
            .get(format!("{}/v2/list", BASE_URL))
            .query(&vec![("page", page), ("limit", limit as u16)])
            .send()
            .await;

        match response {
            Ok(r) => match r.error_for_status() {
                Ok(res) => res
                    .json::<Vec<ImageDetails>>()
                    .await
                    .map_err(|err| InvalidResponse(err.to_string())),
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
