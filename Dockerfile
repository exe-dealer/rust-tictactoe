FROM rust:1.25
WORKDIR /usr/src/rust-tictactoe
COPY . .
RUN cargo install
CMD rust-tictactoe