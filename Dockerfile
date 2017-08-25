FROM nginx:stable-alpine
# Add edge repo
RUN sed -i -e 's/v3\.[0-9]/edge/g' /etc/apk/repositories
# Install Rust and Cargo
RUN apk add rust cargo --update --no-cache

COPY . /app
COPY nginx.conf /etc/nginx/nginx.conf

RUN cd /app &&\
    cargo build --release

ENTRYPOINT ["/bin/sh",  "-c", "/app/entrypoint.sh"]
