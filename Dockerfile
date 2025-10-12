FROM rust:1.90

WORKDIR /usr/src/app

COPY Cargo.toml ./
COPY Cargo.lock ./

RUN #cargo fetch

COPY . .

RUN cargo build --release
CMD ["cargo", "run", "--release"]
