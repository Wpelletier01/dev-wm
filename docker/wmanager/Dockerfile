FROM ubuntu:latest 

WORKDIR /usr/src/dev-wm
RUN groupadd -g 66006 will
RUN useradd will -u 69125 -g 66006 -m -s /bin/bash

RUN apt update -y && apt install \
    curl \
    git build-essential \
    binutils \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxcb1-dev -y

USER will 
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . "$HOME/.cargo/env"



CMD [ "sleep", "3000000" ]

