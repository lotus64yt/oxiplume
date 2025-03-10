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