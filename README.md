A small Rust utility, designed to be run on cron, that'll fetch the PagerDuty on call people and then post them to a Slack incoming webhook.


To compile: `cargo build`

## Compiling for ARM
To compile for the intended arm architecture of Raspberry Pi, with [cross](https://github.com/rust-embedded/cross).

```
cross build --target arm-unknown-linux-gnueabihf --release
docker buildx build --platform linux/arm,linux/arm64 -t arranf/pd-to-slack:0.1.6 . --push
```