FROM rust:1.67

WORKDIR /app/
#COPY . .
COPY . /app/.
COPY ./public /app/.

RUN cargo install --path .

RUN cargo build --release

COPY ./public /app/target/release/



