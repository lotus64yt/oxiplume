use oxiplume::api::OxiPlume;
use tokio;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() {
    let api = OxiPlume::new();

    match api.iss_image(true).await {
        Ok(response) => {

            let mut file = File::create("output.txt").expect("Unable to create file");
            file.write_all(&response.image).expect("Unable to write data");
        }
        Err(e) => eprintln!("Erreur: {:?}", e),
    }
}