# Build stage.
FROM rust:1.75.0-slim-buster as builder

WORKDIR /app

RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
COPY Cargo.toml .

# RUN sed -i 's#src/main_no_shuttle.rs#main.rs#' Cargo.toml
RUN cargo build --release
# RUN sed -i 's#main.rs#src/main_no_shuttle.rs#' Cargo.toml
RUN rm src/main.rs

COPY src ./src
COPY global-bundle.pem .

RUN cargo build --release

# Production stage.
FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/ifood /app/ifood

WORKDIR /app
CMD ["./ifood"]
