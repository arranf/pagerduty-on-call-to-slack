# Use distroless as minimal base image to package the manager binary
# Refer to https://github.com/GoogleContainerTools/distroless for more details
FROM arm32v7/debian:stretch-slim
COPY ./target/arm-unknown-linux-gnueabihf/release/pagerduty-to-slack /
CMD ["/pagerduty-to-slack"]