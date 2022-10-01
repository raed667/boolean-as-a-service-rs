FROM rust:latest

COPY ./ ./

# Install correct version of sqlite (fuuuuuck distros and their slow release cycles)
RUN wget https://www.sqlite.org/2022/sqlite-autoconf-3390400.tar.gz && tar -xvf sqlite-autoconf-3390400.tar.gz && mv sqlite-autoconf-3390400 /opt/sqlite3 && cd /opt/sqlite3 && ./configure --prefix=/usr && make && make install

RUN cargo install diesel_cli --no-default-features --features sqlite

RUN cargo build --release

RUN diesel migration run

ENV DATABASE_URL=/target/release/boolean.db
ENV RUST_BACKTRACE=full

EXPOSE 8080

CMD ["./target/release/boolean-as-a-service"]
