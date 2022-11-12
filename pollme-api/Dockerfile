FROM rust:1.65.0 as builder

RUN USER=root cargo new --bin pollme-api
# maybe absolute path is not needed
WORKDIR ./pollme-api
COPY ./Cargo.toml ./Cargo.toml
# Build empty app with downloaded dependencies to produce a stable image layer for next build
RUN cargo build --release

# Build the actual app
RUN rm src/*.rs
ADD . ./
RUN rm ./target/release/deps/pollme_api*
RUN cargo build --release

FROM debian:bullseye-slim
RUN ldd --version 
ARG APP=/usr/src/app

ARG JWT_SECRET
ARG DB_URL
ARG DB_NAME
ARG DB_USER
ARG DB_PASSWORD
ARG DB_PORT
ARG DB_POOL_SIZE
ARG DB_SSL_MODE

RUN apt-get update \
  && apt-get install -y ca-certificates tzdata \
  && rm -rf /var/lib/apt/lists/*

EXPOSE 3000

ENV TZ=Etc/UTC \
  APP_USER=appuser 

RUN groupadd $APP_USER \
  && useradd -g $APP_USER $APP_USER \
  && mkdir -p ${APP}

COPY --from=builder /pollme-api/target/release/pollme-api ${APP}/pollme-api
COPY --from=builder /pollme-api/migrations ${APP}/migrations

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER

WORKDIR ${APP}


CMD [ "./pollme-api" ]