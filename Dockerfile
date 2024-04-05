FROM rust:1.77.1 AS build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=build-env /app/target/release/short .
COPY --from=build-env /app/Rocket.toml .
CMD [ "./short" ]