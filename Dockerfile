FROM rust:1-alpine3.11 as build

WORKDIR /dkr_pub

COPY . .

RUN cargo build --release

FROM alpine:3.11

COPY --from=build /dkr_pub/target/release/dkr_pub /bin/.

CMD ["/bin/dkr_pub"]
