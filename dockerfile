FROM rust:1.70

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["tail", "-f", "/dev/null"]

