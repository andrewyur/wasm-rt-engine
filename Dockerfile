FROM rust:latest as builder

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /app

COPY . .

RUN wasm-pack build --target web --release

FROM nginx:alpine as prod

COPY --from=builder /app/pkg /usr/share/nginx/html/pkg
COPY ./index.html /usr/share/nginx/html/

# Optional: Add a custom nginx configuration
# COPY ./nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
