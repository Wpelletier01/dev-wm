FROM ubuntu:latest 

WORKDIR /usr/src/dev-wm

RUN apt update -y && apt install \
    curl \
    git build-essential \
    binutils \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxcb1-dev -y

RUN groupadd -g 9999 dockerman
RUN useradd dockerman -u 9999 -g 9999 -m
USER dockerman

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . "$HOME/.cargo/env"



CMD [ "sleep", "3000000" ]

