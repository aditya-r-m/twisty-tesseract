FROM debian:latest

RUN apt-get update && apt-get install build-essential curl fzf git procps python3 tmux wget -y

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup component add rust-analyzer
RUN cargo install wasm-pack

RUN wget https://github.com/helix-editor/helix/releases/download/25.01.1/helix-25.01.1-x86_64-linux.tar.xz
RUN tar -xf helix*.tar.xz
RUN mv ./helix*/hx /usr/local/bin
RUN mkdir -p /root/.config/helix
RUN mv ./helix*/runtime /root/.config/helix

COPY bashrc /root/.bashrc

RUN git config --global --add safe.directory '*'

RUN echo 'source $HOME/.cargo/env' >> /root/.bashrc

COPY config-helix.toml /root/.config/helix/config.toml
