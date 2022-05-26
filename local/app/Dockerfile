FROM rust:1.61.0

WORKDIR /src
ENV RUST_BACKTRACE 1

COPY . .

ENTRYPOINT ["cargo", "run", "--bin", "pdfer"]
EXPOSE 8080
