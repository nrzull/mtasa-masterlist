FROM rust:1.36.0-stretch as build

RUN USER=root cargo new mtasa-masterlist
WORKDIR /mtasa-masterlist
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/mtasa_masterlist*
RUN cargo build --release

FROM rust:1.36.0-slim-stretch

COPY --from=build /mtasa-masterlist/target/release/mtasa-masterlist .

CMD [ "./mtasa-masterlist" ]
