use serde::Deserialize;

#[derive(Deserialize)]
pub struct EightBallsResponse {
    pub answer: String
}

#[derive(Deserialize)]
pub struct EmojiMixReponse {
    pub emoji_url: String
}

#[derive(Deserialize)]
pub struct FunFactResponse {
    pub text: String,
    pub source_url: String,
    pub language: String
}

pub struct IssImageResponse {
    pub image: Vec<u8>,
}

impl Buffer for IssImageResponse {
    fn as_slice(&self) -> &[u8] {
        &self.image
    }
}

pub trait Buffer {
    fn as_slice(&self) -> &[u8];
}

#[derive(Deserialize)]
pub struct IssInfosResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub timestamp: u64,
    pub velocity: f64,
}

#[derive(Deserialize)]
pub struct JokeResponse {
    pub question: String,
    pub answer: String
}

#[derive(Deserialize)]
pub struct MathReponse {
    pub result: Option<String>
}

#[derive(Deserialize)]
pub struct MemeResponse {
    pub title: String,
    pub downvotes: u64,
    pub upvotes: u64,
    pub url: String,
    pub image_url: String,
    pub comments: u64,
    pub author: String
}

#[derive(Deserialize)]
pub struct NasaApodResponse {
    pub date: String,
    pub title: String,
    pub url: String,
    pub hd_url: String,
    pub explanation: String,
    pub page_url: String,
    pub media_type: String
}

#[derive(Deserialize)]
pub struct NpmResponse {
    pub name: String,
    pub description: String,
    pub version: String,
    pub keywords: Vec<String>,
    pub author_username: String,
    pub author_email: String,
    pub repository_url: String,
    pub npm_url: String,
    pub last_published_date: String,
    pub last_published: u64,
}