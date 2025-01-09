FROM rust:1.67

WORKDIR /app/
#COPY . .
COPY . /app/.
COPY ./public /app/.

RUN mkdir -p bin

RUN cargo install --path .

COPY ./public ./bin/first_server_web

RUN cargo run




