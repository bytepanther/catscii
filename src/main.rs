use serde::Deserialize;

#[derive(Deserialize)]
struct CatImage {
    id: String,
    // this is the only field we will really need, but let's show how we would
    // deserialize the other fields as well
    url: String,
    width: usize,
    height: usize,
}


#[tokio::main]
async fn main() {
    let res = reqwest::get("https://api.thecatapi.com/v1/images/search")
    .await
    .unwrap();
    
    if !res.status().is_success() {
        panic!("Request failed with HTTP code / status: {}", res.status());
    }

    //
    let images: Vec<CatImage> = res.json().await.unwrap();
    let image = images
    .first()
    .expect("The cat API should return at least one image");

    println!("The image is at {}", image.url);

}
