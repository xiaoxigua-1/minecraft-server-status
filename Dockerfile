FROM rust:1.67

WORKDIR /usr/status/

COPY . .

RUN cargo build --release

FROM archlinux

WORKDIR /usr/status/

COPY --from=0 /usr/status/target/release/minecraft-server-status ./

CMD ["./minecraft-server-status"]
