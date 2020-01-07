FROM rust:1.31

WORKDIR ./
COPY . .

RUN rustup default nightly
RUN cargo install --path .

CMD ["backend"]
