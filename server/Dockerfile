FROM rust:latest

WORKDIR /code
RUN cargo init
COPY ./Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY . /code

EXPOSE 8080

CMD ["cargo", "run"]
