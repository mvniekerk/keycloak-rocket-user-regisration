# Can be twilio or smsportal
FROM openfaas/of-watchdog:0.7.7-x86_64 as watchdog
FROM rustlang/rust:nightly-stretch as build
ARG SMS_BACKEND=twilio
ENV SMS_BACKEND ${SMS_BACKEND}
WORKDIR /workspace
RUN mkdir -p /workspace/src && touch /workspace/src/main.rs
COPY . /workspace/

RUN echo "Building for backend $SMS_BACKEND" && cargo +nightly build --release --features $SMS_BACKEND

FROM debian:9-slim

RUN apt-get update \
    && apt-get install --reinstall -y openssl libssl1.1 curl  \
    && apt-get clean && apt-get autoclean && rm -rf /var/cache/apt/archives/

RUN addgroup --system app && adduser --system app
COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog

USER app

ENV cgi_headers="true"
ENV fprocess="function"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:8000"

ENV exec_timeout="20s"
ENV write_timeout="25s"
ENV read_timeout="25s"

COPY --from=build /workspace/target/release/user-sms-registration /usr/bin/function
WORKDIR /usr/bin

HEALTHCHECK --interval=1s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]
