use serde::Deserialize;

#[tokio::main]
async fn main() {
    // Make a request to the cat API
    let res = reqwest::get("https://api.thecatapi.com/v1/images/search")
    .await
    .unwrap();
    
    
    // Check if the request was successful
    if !res.status().is_success() {
        panic!("Request failed with HTTP code / status: {}", res.status());
    }
   
    // Define a struct to deserialize the JSON response
    #[derive(Deserialize)]
    struct CatImage {
        url: String,
    }

    // Deserialize the JSON response
    let images: Vec<CatImage> = res.json().await.unwrap();
    let image = images
    .first()
    .expect("The cat API should return at least one image");

    println!("The image is at {}", image.url);

}
