#!/bin/bash

PI_IP=rpi5
PI_USER=alarm
TARGET=aarch64-unknown-linux-gnu # Pi 64-bit
#TARGET=armv7-unknown-linux-gnueabihf # Pi 2/3/4 # 32-bit
#TARGET=arm-unknown-linux-gnueabihf # Pi 0/1

# build binary
# cargo rustc --target aarch64-unknown-linux-gnu --release
cargo build --target $TARGET --release

# upload binary
#echo sshpass -p 'raspberry' 
scp -r ./target/$TARGET/debug/rpi_gpio $PI_USER@$PI_IP:~/workspace

# execute binary
# echo sshpass -p 'raspberry'
ssh $PI_USER@$PI_IP './workspace/rpi_gpio'
