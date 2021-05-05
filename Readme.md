# Kraken Edge Heartbeater

This is a mqtt publisher for Kraken IoT. It publish the heartbeat message as edge client of Kraken.
This program was written by rust.

# Build & run

```
git clone https://github.com/bathtimefish/kraken-edge-heartbeater.git
cd kraken-edge-heartbeater
cargo build --release
cp ./target/release/kraken-edge-heartbeater ./
chmod +x ./kraken-edge-heartbeater
./kraken-edge-heartbeater
```
