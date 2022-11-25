FROM rust:alpine as builder
RUN apk add --no-cache musl-dev
RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk

WORKDIR /build
COPY . .
WORKDIR /build/apps/frontend
RUN trunk build

FROM nginx:stable-alpine

COPY --from=builder /build/apps/frontend/dist /usr/share/nginx/html
COPY ./apps/frontend/nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
