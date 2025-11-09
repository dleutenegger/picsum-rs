use crate::endpoints::RequestError;
use crate::endpoints::RequestError::{
    InvalidRequest, InvalidResponse, ServerError, UnexpectedError,
};
use crate::{BASE_URL, PicsumClient};
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
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
    /// use picsum_rs::endpoints::ImageSettings;
    /// use picsum_rs::PicsumClient;
    ///
    /// // Retrieve the image details for the image with the id `1`.
    /// PicsumClient::default()
    ///     .get_image_details(
    ///         "1",
    ///     );
    ///
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
    /// use picsum_rs::endpoints::ImageSettings;
    /// use picsum_rs::PicsumClient;
    ///
    /// // Retrieve a list of images, fetching page 1 with a limit of 10 images per page.
    /// PicsumClient::default()
    ///     .get_images(1, 10);
    ///
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_retrieve_image_details() {
        let client = PicsumClient::default();

        let response = client.get_image_details("1").await;

        assert!(
            response.is_ok(),
            "Retrieving the image details for the image with the id 1 failed: {}",
            response.unwrap_err().to_string()
        );
        let details = response.unwrap();
        let expected_details = ImageDetails {
            id: "1".to_string(),
            author: "Alejandro Escamilla".to_string(),
            width: 5000,
            height: 3333,
            url: "https://unsplash.com/photos/LNRyGwIJr5c".to_string(),
            download_url: "https://picsum.photos/id/1/5000/3333".to_string(),
        };
        assert_eq!(expected_details, details);
    }

    #[tokio::test]
    async fn test_retrieve_images() {
        let client = PicsumClient::default();

        let response = client.get_images(1, 10).await;

        assert!(
            response.is_ok(),
            "Retrieving page one with a limit of 10 images per page failed: {}",
            response.unwrap_err().to_string()
        );
        let page1 = response.unwrap();

        assert_eq!(10, page1.len());
    }
}
