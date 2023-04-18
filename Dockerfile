#######################################################
# BUILDER
#######################################################

# Compile Styles
FROM node:lts-alpine AS cssbuilder

WORKDIR /

COPY . .

RUN npm i -g tailwindcss
RUN cd ./frontend && NODE_ENV=production tailwindcss -c ./tailwind.config.js -o ./staic/tailwind.css --minify

# Compile Rust
FROM rust:latest AS builder
ENV RUST_LOG "info"

# Install compilation deps
RUN cargo install --locked trunk
RUN cargo install --locked wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown

WORKDIR /srv
# Compile server
COPY --from=cssbuilder . .

RUN cd ./backend && cargo build --release
RUN cp ./backend/target/release/backend ./server
RUN rm -rf ./backend

# Compile frontend
RUN cd ./frontend && trunk build --release
RUN mkdir dist
RUN cp -r ./frontend/dist/* ./dist
RUN rm -rf ./frontend


# Clean up compilation deps
RUN cargo uninstall trunk
RUN cargo uninstall wasm-bindgen-cli

#######################################################
# RUNNER
#######################################################

FROM debian
WORKDIR /srv
COPY --from=builder /srv .

# finish setup
EXPOSE 3000
CMD ["/srv/server"]