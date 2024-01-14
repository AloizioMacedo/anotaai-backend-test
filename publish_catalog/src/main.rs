use std::path::Path;

use aws_lambda_events::event::sqs::SqsEvent;
use aws_sdk_s3::{
    error::SdkError,
    operation::put_object::{PutObjectError, PutObjectOutput},
    primitives::ByteStream,
    Client,
};
use dotenvy::dotenv;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Body {
    #[serde(rename = "Message")]
    message: String,
}

async fn get_catalog(base_url: &str, owner: &str) -> String {
    let client = reqwest::Client::new();
    let req = client
        .get(format!("{base_url}/catalog"))
        .query(&[("owner", owner)])
        .build()
        .expect("Should be able to build request");

    let res = client
        .execute(req)
        .await
        .expect("Should be able to get response from URL");

    res.text().await.expect("Response should have text")
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    _ = dotenv();

    let base_url =
        std::env::var("BASE_URL").expect("'BASE_URL' env variable for API should be set");
    let bucket_name = std::env::var("BUCKET_NAME")
        .expect("'BUCKET_NAME' env variable for S3 bucket should be set");

    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    if let Some(record) = event.payload.records.iter().last() {
        if let Some(body) = &record.body {
            let body: Body = serde_json::from_str(body)
                .unwrap_or_else(|_| panic!("ERROR: Body should be parseable, but is {body}"));

            let message = body.message;

            if let Some((_, owner)) = message.split_once(": ") {
                let catalog = get_catalog(&base_url, owner).await;

                std::fs::write(format!("/tmp/{owner}_catalog.json"), catalog).unwrap();

                upload_object(
                    &client,
                    &bucket_name,
                    &format!("/tmp/{owner}_catalog.json"),
                    &format!("{owner}.json"),
                )
                .await
                .expect("Should be able to upload object");
            } else {
                eprintln!("INFO: Message {message} does not correspond to format 'owner: <owner>'");
            }
        } else {
            eprintln!("INFO: No body in record {record:?}");
        }
    } else {
        eprintln!("INFO: No record in event {event:?}");
    }

    // Extract some useful information from the request

    Ok(())
}

pub async fn upload_object(
    client: &Client,
    bucket_name: &str,
    file_name: &str,
    key: &str,
) -> Result<PutObjectOutput, SdkError<PutObjectError>> {
    let body = ByteStream::from_path(Path::new(file_name)).await;
    client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(body.unwrap())
        .send()
        .await
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
