# AnotaAí Back-End Test

This repository hosts code for an implementation of the
[AnotaAí back-end test](https://github.com/githubanotaai/new-test-backend-nodejs),
but written in Rust. (Both the back-end API and the AWS lambda code.)

The core intent of the repository was self-learning, specifically
with regards to using the AWS SDK for Rust, which seems to be
pretty mature!

## High-level Description

The service is intended to provide some endpoints for creation of
_products_ and _categories_ of such products. Furthermore, _catalogs_
of products should be available in a S3 bucket in a per-owner basis.

It seems that the use-case is something akin to restaurants that
have an owner and a menu. After updating the menu, it should be
available for consumption via some website that will just ingest
the corresponding S3 entry. This mitigates having to query the
database every time a client of the restaurant would like to see
the menu.

## Running

### API

#### Running locally

To run the API locally against a local Mongo DB instance, you
need to set the following environment variables:

- MONGO_SERVICE: Address to the Mongo DB service.
- MONGO_USER: User for the Mongo DB service.
- MONGO_PASSWORD: Password for the Mongo DB service.

Then just run

```bash
cargo run -r
```

And use the URLs <http://localhost:3000/{endpoint}>.

#### Running in AWS

To run the API in AWS, you need to set the following environment
variables instead:

- AWS_TOPIC_ARN: ARN of the SNS topic to use for publishing the changes.
- MONGO_AWS: Address to the Mongo DB service in AWS.

(Ideally, we could change the three environment variables of the local
version to be the same as MONGO_AWS and collapse all into a unique
"MONGO_URL" environment variable for consistency.)

### AWS Lambda (Publishing the catalog)

To deploy the AWS Lambda code, it suffices to build and deploy
it with [cargo lambda](https://github.com/cargo-lambda/cargo-lambda):

```bash
cargo lambda build --release && cargo lambda deploy
```

You need the following environment variables for your Lambda
deployment:

- BASE_URL: URL of the API.
- BUCKET_NAME: Name of the S3 bucket to use for the catalogs.
