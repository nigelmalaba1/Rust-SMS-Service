use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://api.twilio.com/2010-04-01/Accounts/AC9c8f99d060f2bae017f5c900274e39af/Messages.json";

    let mut body = HashMap::new();
    body.insert("Body", "Hello from Twilio");
    body.insert("From", "+18559612819");
    body.insert("To", "+19197237407");
    body.insert("MediaUrl", "https://i.ibb.co/vwR14XL/passkey.png");

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .basic_auth("AC9c8f99d060f2bae017f5c900274e39af", Some("52ee40e80b96982b0756063733f901fb"))
        .form(&body)
        .send()
        .await?;

    println!("{:#?}", response.text().await?);

    Ok(())
}
