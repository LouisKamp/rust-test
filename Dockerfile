FROM rust:1.67-slim as builder
WORKDIR /usr/src/learn
COPY . .
RUN cargo build -r

FROM rust:1.67-slim
WORKDIR /usr/app/learn
COPY --from=builder /usr/src/learn/target/release/learn /usr/app/learn/learn
EXPOSE 3000
CMD ["/usr/app/learn/learn"]
