FROM ubuntu:latest

WORKDIR /app
EXPOSE 9000

RUN apt update && apt install -y curl vim build-essential libpq-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . "$HOME/.cargo/env" && cargo install diesel_cli --no-default-features --features postgres

COPY ./entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
