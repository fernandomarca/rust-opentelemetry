FROM rust:1.78-slim-bullseye

RUN apt-get update && \ 
  apt-get install -y build-essential \ 
  pkg-config \
  # ca-certificates \ 
  # tzdata \
  curl \
  cmake \
  libpq-dev \
  librust-openssl-sys-dev \
  libssl-dev \
  libsqlite3-dev

WORKDIR /usr/src/omnichannel

CMD [ "tail","-f","/dev/null" ]