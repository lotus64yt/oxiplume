use oxiplume::api::OxiPlume;
use tokio;

#[tokio::main]
async fn main() {
    let api = OxiPlume::new();

    match api.emoji_mix("😭", "👻").await {
        Ok(response) => println!("Message: {}", response.emoji_url),
        Err(e) => eprintln!("Erreur: {:?}", e),
    }
}