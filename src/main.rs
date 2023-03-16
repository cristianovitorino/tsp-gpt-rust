use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct OAIResponse {
    // Since the response could be 'null', 'Option' is gonna be used
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>,
}

#[derive(Serialize, Debug)]
struct OAIRequest {
    prompt: String,
    // Limits the amount of tokens (words in the response) since it can become expensive
    max_tokens: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";

    //let preamble = "Given a list of cities and the distances between each pair of cities, what is the shortest possible route that visits each city exactly once and returns to the origin city?";

    let preamble = "In the following list of cities in the state of New Jersey, United States, calculate the best 3 shortest possible routes that visits each city exactly once and returns to the origin city";

    //Newark, Old Bridge, Paterson, Elizabeth, Cherry Hill, Edison, Woodbridge, Vineland, Hamilton township, Trenton

    //newark, elizabeth, paterson, woodbridge, trenton, edison

    let oai_token: String = env::var("OAI_TOKEN").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);

    // For a clean prompt
    println!("{esc}c", esc = 27 as char);

    // No exit conditions for now, 'Ctrl + C' needs to be pressed in order for the CLI to finish
    loop {
        // Input
        print!("> ");
        stdout().flush().unwrap();
        let mut user_text_input = String::new();

        // Read from 'stdin'
        stdin()
            .read_line(&mut user_text_input)
            .expect("Failed to read line");

        // Loading UI
        println!("");
        let mut sp = Spinner::new(Spinners::Dots9, "OpenAI is thinking... ".into());

        // Build request Object
        let oai_request = OAIRequest {
            prompt: format!("{} {}", preamble, user_text_input),
            max_tokens: 100,
        };

        // Build the HTTP request body, serialized version of the 'OAIRequest' struct
        let body = Body::from(serde_json::to_vec(&oai_request)?);

        // Build the request
        let req = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &auth_header_val)
            .body(body)
            .unwrap();
        //println!("Request: {:?}", req);

        // Wait for response and store it
        let res = client.request(req).await?;
        //println!("{:?}", res.status());

        // Get the response body
        let body = hyper::body::aggregate(res).await?;
        // Deserialize the above body into a response struct
        let json: OAIResponse = serde_json::from_reader(body.reader())?;

        sp.stop();
        println!("");

        // Print the response
        println!("{}", json.choices[0].text);
    }
}
