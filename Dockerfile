FROM ubuntu:latest

SHELL [ "/bin/bash", "-c" ]
ENV SSH_USER=root
ENV SSH_HOST=134.209.220.76
ENV SHELL=/bin/bash
ENV HOME="/root"
ENV PATH="$HOME/.cargo/bin:$PATH"
ENV REMOTE_WORKDIR="/${SSH_USER}/app"

RUN apt-get update -y && apt-get install -y build-essential unzip pkg-config libssl-dev curl openssh-client
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN cargo --version

COPY . $HOME/code
COPY ./secrets $HOME/code/secrets
COPY ./server/.env $HOME/code/server/.env
WORKDIR $HOME/code

CMD \
    # build the server
    cargo clean && \
    cargo build --release && \
    # ensure REMOTE_WORKDIR exists
    ssh -o StrictHostKeyChecking=no -i ./secrets/hey_cli ${SSH_USER}@${SSH_HOST} "mkdir -p ${REMOTE_WORKDIR}" && \
    # copy the binary to the server
    scp -o StrictHostKeyChecking=no -i ./secrets/hey_cli -r ./target/release/server ${SSH_USER}@${SSH_HOST}:${REMOTE_WORKDIR}/server && \
    # copy the .env file to the server
    scp -o StrictHostKeyChecking=no -i ./secrets/hey_cli -r ./server/.env ${SSH_USER}@${SSH_HOST}:${REMOTE_WORKDIR}/.env && \
    # copy server/docker-compose.yml
    scp -o StrictHostKeyChecking=no -i ./secrets/hey_cli -r ./server/docker-compose.yml ${SSH_USER}@${SSH_HOST}:${REMOTE_WORKDIR}/docker-compose.yml && \
    # copy server/Dockerfile
    scp -o StrictHostKeyChecking=no -i ./secrets/hey_cli -r ./server/Dockerfile ${SSH_USER}@${SSH_HOST}:${REMOTE_WORKDIR}/Dockerfile && \
    # ensure docker is installed
    ssh -o StrictHostKeyChecking=no -i ./secrets/hey_cli ${SSH_USER}@${SSH_HOST} "snap install docker" && \
    # start the server
    ssh -o StrictHostKeyChecking=no -i ./secrets/hey_cli ${SSH_USER}@${SSH_HOST} "cd ${REMOTE_WORKDIR} && docker-compose up --build -d" && \
    # done
    echo "Server deployed successfully"
