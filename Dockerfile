# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.79.0
ARG APP_NAME=rustium_template

FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app
RUN apt-get update
RUN apt-get -y install clang
COPY . .
RUN cargo build -p $APP_NAME --locked --release
RUN cp ./target/release/$APP_NAME /bin/server

FROM debian:bullseye-slim AS final
COPY --from=build /bin/server /bin/
EXPOSE 8080 8080
CMD ["/bin/server"]
