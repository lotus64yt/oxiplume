use reqwest::Client;
use crate::types::{EightBallsResponse, EmojiMixReponse, FunFactResponse, IssImageResponse};

#[derive(Debug)]
pub enum OxiError {
    InvalidResponse(String),
    ReqwestError(reqwest::Error),
}

pub struct OxiPlume {
    base_url: String,
    client: Client,
}
impl From<reqwest::Error> for OxiError {
    fn from(error: reqwest::Error) -> Self {
        OxiError::ReqwestError(error)
    }
}

impl OxiPlume {
    pub fn new() -> Self {
        Self {
            base_url: "https://plume.ptarmigan.xyz/api".to_string(),
            client: Client::new(),
        }
    }

    pub async fn eight_ball(&self, locale: &str) -> Result<EightBallsResponse, OxiError> {
        let url = format!("{}/8ball?locale={}", self.base_url, locale);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(OxiError::ReqwestError)?
            .json::<EightBallsResponse>()
            .await
            .map_err(OxiError::ReqwestError)?;
        Ok(response)
    }

    pub async fn emoji_mix(&self, left: &str, right: &str) -> Result<EmojiMixReponse, OxiError> {
        let url = format!("{}/emoji-mix?left={}&right={}", self.base_url, left, right);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(OxiError::ReqwestError)?
            .json::<EmojiMixReponse>()
            .await
            .map_err(OxiError::ReqwestError)?;
        Ok(response)
    }

    pub async fn funfact(&self, locale: &str) -> Result<FunFactResponse, OxiError> {
        let url = format!("{}/funfact?locale={}", self.base_url, locale);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(OxiError::ReqwestError)?
            .json::<FunFactResponse>()
            .await
            .map_err(OxiError::ReqwestError)?;
        Ok(response)
    }

    pub async fn iss_image(&self, circle: bool) -> Result<IssImageResponse, OxiError> {
        let url = format!("{}/iss-image?circle={}", self.base_url, circle);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(OxiError::ReqwestError)?
            .bytes()
            .await
            .map_err(OxiError::ReqwestError)?;
        Ok(IssImageResponse { image: response.to_vec() })
    }

    // pub async fn get_message(&self) -> Result<ApiResponse, ApiError> {
    //     let url = format!("{}/message", self.base_url);
    //     let response = self.client.get(&url).send().await?.json::<ApiResponse>().await?;
    //     Ok(response);
    // }
}