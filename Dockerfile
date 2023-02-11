FROM rust

WORKDIR app
COPY . /app

RUN cargo add tokio reqwest
