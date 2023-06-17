FROM rust:latest as builder
WORKDIR /usr/src/trelloBack
COPY . .
RUN cargo install --path .
RUN cargo install diesel_cli --no-default-features --features mysql


FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libmariadb-dev sqlite3 libpq-dev libpq5 libssl-dev default-libmysqlclient-dev && apt install  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/back /usr/local/bin/trelloBack
COPY --from=builder /usr/src/trelloBack/migrations /usr/local/migrations
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
ENV PATH=$PATH:/usr/local/bin
RUN chmod a+x /usr/local/bin/diesel
RUN chmod a+x /usr/local/bin/trelloBack
CMD ["/bin/bash","-c", "sleep 3;echo migrations;diesel setup;diesel migration run --migration-dir /usr/local/migrations;trelloBack"]
