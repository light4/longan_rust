# 编译

```bash
cargo build --release
riscv32-elf-objcopy -O binary target/riscv32imac-unknown-none-elf/release/longan_rust firmware.bin
```

# USB DFU 下载

使用 `dfu-util` 下载

- 首次使用需要安装 `libusb` 驱动程序
- 准备 USB Type-c 数据线
- 使用数据线连接电脑与开发板
- 开发板按住 BOOT 键，再按 RESET 键重启开发板后再松开 BOOT 键，进入 DFU 模式

```bash
dfu-util -l
dfu-util -a 0 -s 0x08000000:leave -D firmware.bin
```

# 参考资料

- <https://wiki.sipeed.com/soft/longan/zh/get_started/blink.html>
- <http://dl.sipeed.com/shareURL/LONGAN/Nano>
- <https://longan.sipeed.com/zh/examples/badapple.html>
- <https://github.com/riscv-rust/longan-nano>
