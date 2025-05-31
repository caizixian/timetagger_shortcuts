# syntax=docker/dockerfile:1
FROM alpine:3
WORKDIR /timetagger_shortcuts
COPY ./target/x86_64-unknown-linux-musl/release/timetagger_shortcuts /timetagger_shortcuts/timetagger_shortcuts
ENTRYPOINT ["/timetagger_shortcuts/timetagger_shortcuts"]