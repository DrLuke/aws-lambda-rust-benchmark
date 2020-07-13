# aws-lambda-rust-benchmark

Minimal lambda function written in rust. The goal was to gauge 
the potential speedup for cold-start times on AWS Lambda.

Uses [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime) as the runtime.


## Usage

```shell script
$ rustup target add x86_64-unknown-linux-musl
$ npm install aws-cdk
$ pip install -r requirements.txt
$ make deploy
```

The CDK deploy will show you the api gateway endpoint URL once deploy is complete.
Just POST something to it to get it echoed back to you or send a GET request for a generic message.