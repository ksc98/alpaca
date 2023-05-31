# Use the official Rust image as the base image
FROM rust:latest
RUN apt-get update && apt-get install --no-install-recommends --assume-yes protobuf-compiler

# Set the working directory inside the container
WORKDIR /app

COPY ./protos /protos

# Copy the Cargo.toml and Cargo.lock files to the container
#COPY Cargo.toml Cargo.lock ./

ARG CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Build the dependencies. This step is separate from copying the source code
# to take advantage of Docker's layer caching. It will only re-run this step
# if the dependencies change.
# RUN cargo build --release

EXPOSE 4321

COPY ./engine/Cargo.toml .
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo build --release
RUN rm -rf ./src
COPY ./engine/src ./src
COPY ./engine/build.rs .
RUN touch -a -m ./src/main.rs

RUN cargo build --release

CMD ["target/release/engine"]