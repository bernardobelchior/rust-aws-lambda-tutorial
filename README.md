### Rust AWS Lambda Tutorial

This repository hosts the code for my [Rust AWS Lambda Tutorial](), using [rust-aws-lambda](https://github.com/srijs/rust-aws-lambda) and [docker-lambda](https://github.com/lambci/docker-lambda). It also includes a way of testing the function locally, so you don't have to provision an AWS Lambda Function to run your program, reducing the feedback loop.

## Compiling

A specific Rust target must be installed, and it can be done as follows:

`$ rustup target add x86_64-unknown-linux-musl`

The `musl-gcc` package must also be installed. On Ubuntu, it would be done as follows:

`$ apt-get install musl`

After installing the target and `musl-gcc`, compiling is only a matter of executing the following command:

`$ cargo build --target x86_64-unknown-linux-musl`

## Running

As mentioned above, this tutorial uses [docker-lambda](https://github.com/lambci/docker-lambda), specifically the Go image, as it allows running Rust binaries natively.
In order to run our compiled program, we would do as follows:

```$ docker run --rm -v "$PWD":/var/task lambci/lambda:go1.x <path-to-executable> <input-to-handler-function>```

The `<input-to-handler-function>` argument is the content the handler function is expecting and can be anything. In this tutorial, we'll be using a JSON object, eg. `{"some": "event"}`.

### Reading from stdin

It's possible to pipe the `<input-to-handler-function>` from another command. In order to achieve that, the following command must be used:

`$ echo '{"some": "event"}' | docker run --rm -v "$PWD":/var/task -i -e DOCKER_LAMBDA_USE_STDIN=1 lambci/lambda:go1.x`
