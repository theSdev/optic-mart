FROM rust
LABEL maintainer thesdev@outlook.com
RUN apt-get install pkg-config libssl-dev -y
# ADD http://www.convert-unix-time.com/api?timestamp=now /tmp/bustcache
ADD https://google.com cache_bust
COPY target/debug/user-command-service .
EXPOSE 8090
CMD ["./user-command-service"]
