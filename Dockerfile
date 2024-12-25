FROM ubuntu:22.04

SHELL [ "/bin/bash", "-c" ]
ENV SSH_USER=root
ENV SSH_HOST=134.209.220.76
ENV SHELL=/bin/bash
ENV HOME="/root"
ENV PATH="$HOME/.cargo/bin:$PATH"

RUN apt-get update -y && apt-get install -y build-essential unzip pkg-config libssl-dev curl openssh-client
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN cargo --version

COPY . $HOME/code
COPY ./secrets $HOME/code/secrets
WORKDIR $HOME/code

CMD \
    cargo clean && \
    cargo build --release && \
    # copy the binary to the server
    scp -o StrictHostKeyChecking=no -i ./secrets/hey_cli -r ./target/release/server ${SSH_USER}@${SSH_HOST}:~/server-new && \
    # copy the .env file to the server
    scp -o StrictHostKeyChecking=no -i ./secrets/hey_cli -r ./server/.env ${SSH_USER}@${SSH_HOST}:~/.env-new
