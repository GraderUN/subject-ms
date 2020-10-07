FROM rustlang/rust:nightly-slim as builder
RUN apt update && apt install default-libmysqlclient-dev -y

WORKDIR /subject-ms
COPY Cargo.* ./
COPY src src
RUN cargo build --release

FROM debian:stable-slim
RUN apt update && apt install default-libmysqlclient-dev -y
EXPOSE 4000

COPY --from=builder /subject-ms/target/release/subject-ms .
COPY Rocket.toml .
CMD ./subject-ms
