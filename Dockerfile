FROM rust:1.67

WORKDIR /app/
#COPY . .
COPY . /app/.
RUN cargo install --path .