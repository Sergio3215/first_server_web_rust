FROM rust:1.67

WORKDIR /app/
#COPY . .
COPY . /app/.
COPY ./public /app/.

RUN mkdir -p bin

RUN cargo install --path .

RUN cargo build --release

COPY ./public ./bin/first_server_web




