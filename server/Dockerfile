FROM ubuntu:latest

ENV SHELL=/bin/bash
ENV HOME="/root"
ENV PORT=80

RUN apt-get update && apt-get install -y ca-certificates

COPY . $HOME/app
WORKDIR $HOME/app

EXPOSE 80
CMD ["./server"]
