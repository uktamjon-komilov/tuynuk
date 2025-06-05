use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let resp = reqwest::get("http://localhost:3000").await;

    if let Err(_e) = &resp {
        println!("Couldn't connect to the server");
        return Ok(());
    }

    let response = resp?;

    if !&response.status().is_success() {
        println!("Couldn't get data successfully");
        return Ok(());
    }

    let text = response.text().await?;

    println!("{}", text);

    Ok(())
}
