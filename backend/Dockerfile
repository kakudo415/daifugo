FROM rust

WORKDIR /usr/src

RUN cargo new --vcs none --bin backend

WORKDIR /usr/src/backend

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

COPY . .

# HACK: without this, cargo won't recompile main.rs
RUN touch src/main.rs

RUN cargo install --path .

ENTRYPOINT ["backend"]
