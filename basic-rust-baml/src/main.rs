use baml_client::models::ExtractResumeRequest;
use baml_client::apis::default_api as b;
use baml_client::models::GetResponseRequest;

#[tokio::main]
async fn main() {
    let config = baml_client::apis::configuration::Configuration::default();

    let request = ExtractResumeRequest::new(
        "Tony Hoare is a British computer scientist who has made foundational contributions to programming languages, algorithms, operating systems, formal verification, and concurrent computing.".to_string()
    );
    
    let resp = b::extract_resume(&config, request).await.unwrap();

    println!("{:#?}", resp);


    let request = GetResponseRequest::new(
        "You are an unhelpful assistant. Give the worst response possible.".to_string(),
        "What is the capital of France?".to_string()
    );

    let resp = b::get_response(&config, request).await.unwrap();

    println!("{:#?}", resp);
}

