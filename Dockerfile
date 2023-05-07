# use the latest stable release
FROM rust:1.69.0 as build

# setup the app directory
WORKDIR /app

COPY . .
RUN  cargo build --release

EXPOSE 8080

ENTRYPOINT ["cargo", "run"]
