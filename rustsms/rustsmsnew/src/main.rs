use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://api.twilio.com/2010-04-01/Accounts/AC9c8f99d060f2bae017f5c900274e39af/Messages.json";

    let mut body = HashMap::new();
    body.insert("Body", "Check out this image!");
    body.insert("From", "+18559612819");
    body.insert("To", "+19197237407");
    //body.insert("MediaUrl", "/image.jpg");

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .basic_auth("AC9c8f99d060f2bae017f5c900274e39af", Some("a101663ca49d3a19ce2f23f7d122b02c"))
        .form(&body)
        .send()
        .await?;

    println!("{:#?}", response.text().await?);

    Ok(())
}