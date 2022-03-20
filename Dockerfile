FROM rust

RUN apt update -y
RUN apt install zsh git -y
RUN sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"