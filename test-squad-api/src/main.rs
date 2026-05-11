use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    let response = client
        .post("https://sandbox-api-d.squadco.com/transaction/initiate")
        .header("Authorization", "sandbox_sk_90388d1497039e52de8017acbcc5e7f8e20cd76b04f1")
        .header("Content-Type", "application/json")
        .json(&json!({
            "email": "semajayi1234@gmail.com",
            "currency": "NGN",
            "initiate_type": "inline",
            "callback_url": "https://www.linkedin.com/",
            "amount": "20000"
        }))
        .send()
        .await?;

    let text = response.text().await?;

    println!("{}", text);

    attempt_transaction().await;
    Ok(())
}

async fn attempt_transaction() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client
        .post("https://sandbox-api-d.squadco.com/virtual-account/simulate/payment")
        .header("Authorization", "sandbox_sk_90388d1497039e52de8017acbcc5e7f8e20cd76b04f1")
        .json(&json!({
            "virtual_account_number": "9279755518",
            "amount": "20000"
        }))
        .send()
        .await?;

    println!("SIMULATION RESPONSE:\n{}", response.text().await?);

    Ok(())
}