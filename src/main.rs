use serde::Deserialize;

#[tokio::main]

async fn main() {
    let url = get_cat_image_url().await.unwrap();
    println!("The image is at {}", url);
}

async fn get_cat_image_url() -> color_eyre::Result<String> {
    let api_url = "https://api.thecatapi.com/v1/images/search";
    let res = reqwest::get(api_url).await?;

    // Check if the request was successful
    if !res.status().is_success() {
        return Err(color_eyre::eyre::eyre!(
            "The Cat API returned HTTP {}",
            res.status()
        ));
    }

    // Define a struct to deserialize the JSON response
    #[derive(Deserialize)]
    struct CatImage {
        url: String,
    }

    // Return the URL of the first image
    let mut images: Vec<CatImage> = res.json().await?;
    let Some(image) = images.pop() else {
        return Err(color_eyre::eyre::eyre!("The cat API returned no images"));
    };

    Ok(image.url)
}
