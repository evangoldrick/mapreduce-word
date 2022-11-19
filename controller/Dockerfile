FROM rust

EXPOSE 8080

COPY . /app/

WORKDIR /app/

RUN cargo build

CMD cargo run
