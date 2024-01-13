#!/bin/bash
cargo espflash flash --partition-table partitions.csv --port /dev/ttyUSB0 --release --baud 921600 --erase-parts otadata --erase-parts nvs --monitor