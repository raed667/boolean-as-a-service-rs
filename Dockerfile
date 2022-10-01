FROM rust:latest

ENV DATABASE_URL=/target/release/boolean.db
ENV RUST_BACKTRACE=1

COPY ./ ./

RUN cargo install diesel_cli --no-default-features --features sqlite

RUN cargo build --release

RUN cd ./target/release && diesel migration run && diesel migration redo

EXPOSE 8080

CMD ["./target/release/boolean-as-a-service"]
