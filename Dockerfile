FROM rust:1.64 as build

RUN USER=root cargo new --bin cs361_microservice
WORKDIR /cs361_microservice

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release & rm src/*.rs

COPY ./src ./src

RUN rm -f ./target/release/deps/cs361_microservice*

RUN cargo build --release

FROM rust:1.64-slim-buster

COPY --from=build /cs361_microservice/target/release/cs361_microservice .


CMD ["./cs361_microservice"]

# EXPOSE 8080/tcp
