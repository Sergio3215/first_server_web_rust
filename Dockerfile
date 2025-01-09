FROM rust:1.67

WORKDIR /app/
#COPY . .
COPY . /app/.
COPY ./public /app/.

RUN mkdir -p bin
COPY ./public ./bin/

RUN cargo install --path .




