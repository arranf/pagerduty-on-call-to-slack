set -e

cross build --target arm-unknown-linux-gnueabihf --release
docker buildx build --platform linux/arm,linux/arm64 -t arranf/pd-to-slack:0.1.6 . --push
