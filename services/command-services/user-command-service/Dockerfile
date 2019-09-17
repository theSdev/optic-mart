FROM rust
LABEL maintainer thesdev@outlook.com
RUN apt-get install pkg-config libssl-dev -y
ADD http://www.convert-unix-time.com/api?timestamp=now /tmp/bustcache
RUN git clone -b poc --single-branch https://github.com/theSdev/optic-mart.git
WORKDIR optic-mart/services/command-services/user-command-service
EXPOSE 8090
CMD ["cargo", "run"]
