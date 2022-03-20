FROM rust

RUN apt update -y
RUN apt install zsh git -y
RUN sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
RUN rustup component add rustfmt \
    && rustup target add x86_64-pc-windows-gnu \
    && rustup target add x86_64-unknown-linux-gnu \
    && rustup target add x86_64-apple-ios \
    && cargo install cargo-x