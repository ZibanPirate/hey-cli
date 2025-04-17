# --platform linux/amd64,linux/arm64
FROM --platform=linux/amd64 ubuntu:latest

ENV SHELL=/bin/bash
ENV HOME="/root"
ENV PORT=80

# Install CA certificates
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY ./target/x86_64-unknown-linux-gnu/release-server/hey-cli-server $HOME/app/server
WORKDIR $HOME/app

EXPOSE 80
CMD ["./server"]
