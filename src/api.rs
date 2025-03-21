use reqwest::Client;
use crate::types::{ColorReponse, EightBallsResponse, EmojiMixReponse, FunFactResponse, IssImageResponse, IssInfosResponse, JokeResponse, MathReponse, MemeResponse, NasaApodResponse, NpmResponse, QuoteResponse, UpsideDownResponse, UrbanDictionaryResponse};
use regex::Regex;

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

    pub async fn iss_infos(&self) -> Result<IssInfosResponse, OxiError> {
        let url = format!("{}/iss", self.base_url);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(OxiError::ReqwestError)?
            .json::<IssInfosResponse>()
            .await
            .map_err(OxiError::ReqwestError)?;
        Ok(IssInfosResponse { latitude: response.latitude, longitude: response.longitude, altitude: response.altitude, timestamp: response.timestamp, velocity: response.velocity })
    }

    pub async fn joke(&self, locale: &str) -> Result<JokeResponse, OxiError> {
        let url = format!("{}/joke?locale={}", self.base_url, locale);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(OxiError::ReqwestError)?
            .json::<JokeResponse>()
            .await
            .map_err(OxiError::ReqwestError)?;
        Ok(response)
    }

    pub async fn math(&self, expression: &str) -> Result<MathReponse, OxiError> {
        if expression.len() < 1 || expression.len() > 100 {
            return Err(OxiError::InvalidResponse("Expression length must be between 1 and 100".to_string()));
        }
        let url = format!("{}/math?expression={}", self.base_url, expression);
        let response: MathReponse = self.client.get(&url)
            .send()
            .await
            .map_err(OxiError::ReqwestError)?
            .json()
            .await
            .map_err(OxiError::ReqwestError)?;

        if response.result.is_none() {
            return Err(OxiError::InvalidResponse("Invalid expression".to_string()));
        }
        
        Ok(response)
    }

    pub async fn meme(&self) -> Result<MemeResponse, OxiError> {
        let url = format!("{}/meme", self.base_url);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(OxiError::ReqwestError)?
            .json::<MemeResponse>()
            .await
            .map_err(OxiError::ReqwestError)?;
        Ok(response)
    }

    pub async fn nasa_apod(&self) -> Result<NasaApodResponse, OxiError> {
        let url = format!("{}/nasa-apod", self.base_url);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(OxiError::ReqwestError)?
            .json::<NasaApodResponse>()
            .await
            .map_err(OxiError::ReqwestError)?;
        Ok(response)
    }

    pub async fn npm(&self, package: &str) -> Result<NpmResponse, OxiError> {
        let url = format!("{}/npm?name={}", self.base_url, package);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(OxiError::ReqwestError)?
            .json::<NpmResponse>()
            .await
            .map_err(OxiError::ReqwestError)?;
        Ok(response)
    }

    pub async fn quote(&self, locale: &str) -> Result<QuoteResponse, OxiError> {
        let url = format!("{}/quote?locale={}", self.base_url, locale);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(OxiError::ReqwestError)?
            .json::<QuoteResponse>()
            .await
            .map_err(OxiError::ReqwestError)?;
        Ok(response)
    }

    pub async fn random_emoji_mix(&self) -> Result<EmojiMixReponse, OxiError> {
        let url = format!("{}/random-emoji-mix", self.base_url);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(OxiError::ReqwestError)?
            .json::<EmojiMixReponse>()
            .await
            .map_err(OxiError::ReqwestError)?;
        Ok(response)
    }

    pub async fn upside_down(&self, text: &str) -> Result<UpsideDownResponse, OxiError> {
        let url = format!("{}/upside-down?text={}", self.base_url, text);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(OxiError::ReqwestError)?
            .json()
            .await
            .map_err(OxiError::ReqwestError)?;
        Ok(response)
    }

    pub async fn urban(&self, term: &str) -> Result<UrbanDictionaryResponse, OxiError> {
        let url = format!("{}/urban?word={}", self.base_url, term);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(OxiError::ReqwestError)?
            .json()
            .await
            .map_err(OxiError::ReqwestError)?;
        Ok(response)
    }

    pub async fn color(&self, color: &str) -> Result<ColorReponse, OxiError> {
        if !Regex::new(r"^[A-Fa-f0-9]{6}$").unwrap().is_match(color) {
            return Err(OxiError::InvalidResponse("Invalid color format".to_string()));
        }
        let url = format!("{}/color?color={}", self.base_url, color);
        let response = self.client.get(&url)
            .send()
            .await
            .map_err(OxiError::ReqwestError)?
            .json()
            .await
            .map_err(OxiError::ReqwestError)?;
        Ok(response)
    }
    // pub async fn get_message(&self) -> Result<ApiResponse, ApiError> {
    //     let url = format!("{}/message", self.base_url);
    //     let response = self.client.get(&url).send().await?.json::<ApiResponse>().await?;
    //     Ok(response);
    // }
}