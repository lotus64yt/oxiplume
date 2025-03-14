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

#[derive(Deserialize)]
pub struct QuoteResponse {
    pub quote: String,
    pub author: String
}

#[derive(Deserialize)]
pub struct RandomEmojiMixResponse {
    pub emoji_url: String
}

#[derive(Deserialize)]
pub struct UpsideDownResponse {
    pub text: String
}

#[derive(Deserialize)]
pub struct UrbanDictionaryResponse {
    pub word: String,
    pub url: String,
    pub definition: String,
    pub example: String,
    pub author: String,
    pub thumbs_up: u64,
    pub thumbs_down: u64,
}

#[derive(Deserialize)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub value: String,
}

#[derive(Deserialize)]
pub struct HSL {
    pub h: u16,
    pub s: u8,
    pub l: u8,
    pub value: String,
}

#[derive(Deserialize)]
pub struct HSV {
    pub h: u16,
    pub s: u8,
    pub v: u8,
    pub value: String,
}

#[derive(Deserialize)]
pub struct CMYK {
    pub c: u8,
    pub m: u8,
    pub y: u8,
    pub k: u8,
    pub value: String,
}

#[derive(Deserialize)]
pub struct HEX {
    pub value: String,
    pub clean: String,
}

#[derive(Deserialize)]
pub struct ColorReponse {
    pub rgb: RGB,
    pub hsl: HSL,
    pub hsv: HSV,
    pub cmyk: CMYK,
    pub name: String,
    pub hex: HEX,
    pub url: String,
}