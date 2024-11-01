



cargo build --target x86_64-y_os.json

qemu-system-x86_64 -drive format=raw,file=target/x86_64-y_os/debug/bootimage-yOS.bin  ==> cargo run