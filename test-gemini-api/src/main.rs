use reqwest::Client;
use serde_json::json;
use dotenvy::dotenv;
use std::env;
use semire_core::Readable;
use serde_json::Value;
use std::fs;
use base64::{Engine as _, engine::general_purpose};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("Type in a question and see what gemini says\n");
    println!("Choose 1. Text question, 2. Visual question");

    let mut user_choice = String::read();
    let mut question = String::new();

    match user_choice.as_str()
    {
        "1" => {
        question= String::read();
        send_request(question).await?;

        }
        "2" => {
            println!("Paste your image file path");
            let image_base64 = image_handler()?;

            println!("Ask your visual question:");
            let visual_question = String::read();

            gemini_vision(image_base64, visual_question).await?;
        }
        _ => {
            println!("Invalid choice!");
        }
    }
    Ok(())
}

fn image_handler() -> Result<String, Box<dyn std::error::Error>> {
    
    let image_path: String = Readable::read();
    let image_bytes = std::fs::read(&image_path)?;

    let image_base64 =
    general_purpose::STANDARD.encode(&image_bytes);

    println!("Image size: {} bytes", &image_bytes.len());

    Ok(image_base64)
}
/*
curl "https://generativelanguage.googleapis.com/v1beta/models/gemini-flash-latest:generateContent" \
  -H 'Content-Type: application/json' \
  -H 'X-goog-api-key: I am not gonna leak this again' \
  -X POST \
  -d '{
    "contents": [
      {
        "parts": [
          {
            "text": "Explain how AI works in a few words"
          }
        ]
      }
    ]
  }'
  */

async fn send_request(question:String) -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok(); // loads .env file

    let secret_key = env::var("GEMINI_API_KEY")?;    
    let client = Client::new();

    let response = client
        .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent")
        .header("x-goog-api-key", secret_key)
        .header("Content-Type", "application/json")
        .json(&json!({
            "contents": [
                {
                    "parts": [
                        {
                            "text" : question
                        }
                    ]
                }
            ]
          
        }))
        .send()
        .await?;

    //println!("SIMULATION RESPONSE:\n{}", response.text().await?);

    let text = response.text().await?;

    let parsed: Value = serde_json::from_str(&text)?;

    let ai_response = &parsed["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap_or("Gemini does not feel like answering you");

    println!("{}", ai_response);

    Ok(())
}
/*
# First, ensure you have the image file locally.
# Encode the image to base64
IMAGE_BASE64=$(base64 -w 0 my-image.png)

curl -X POST \
  "https://generativelanguage.googleapis.com/v1beta/models/gemini-robotics-er-1.6-preview:generateContent \
  -H "x-goog-api-key: $GEMINI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "contents": [
      {
        "parts": [
          {
            "inlineData": {
              "mimeType": "image/png",
              "data": "'"${IMAGE_BASE64}"'"
            }
          },
          {
            "text": "Point to no more than 10 items in the image. The label returned should be an identifying name for the object detected. The answer should follow the json format: [{\"point\": [y, x], \"label\": <label1>}, ...]. The points are in [y, x] format normalized to 0-1000."
          }
        ]
      }
    ],
    "generationConfig": {
      "temperature": 0.5,
      "thinkingConfig": {
        "thinkingBudget": 0
      }
    }
  }'
*/
async fn gemini_vision(image_base64: String, visual_question: String) -> Result<(), Box<dyn std::error::Error>>
{
    dotenv().ok(); // loads .env file
    let secret_key = env::var("GEMINI_API_KEY")?;   

    let client = Client::new();

    let response = client
    .post("https://generativelanguage.googleapis.com/v1beta/models/gemini-robotics-er-1.6-preview:generateContent")
    .header("x-goog-api-key", secret_key)
    .header("Content-Type", "application/json")
    .json(&json!({
        "contents": [
            {
                "parts": [
                    {
                        "inlineData": {
                            "mimeType": "image/png",
                            "data": image_base64
                        }
                    },
                    {
                        "text": visual_question
                    }
                ]
            }
        ],
    
        "generationConfig": {
            "temperature": 0.5,
            "thinkingConfig": {
                "thinkingBudget": 0
            }
        }
    })).send().await?;

    //println!("SIMULATION RESPONSE:\n{}", response.text().await?);

    let text = response.text().await?;

let parsed: Value = serde_json::from_str(&text)?;

let output = parsed["candidates"][0]["content"]["parts"][0]["text"]
    .as_str()
    .unwrap_or("No response");

println!("{}", output);

    Ok(())
}