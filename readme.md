# aarch64-minimal

minimal OS for aarch64(qemu virt machine)


## Features
- multi task
- task scheduler
- GICv2 interrupt controller
  - sgi ppi spi 
- PCI bus
- UART
  - rx interrupt
- block device
    - virtio-blk-pci
- UNIX-like sys calls
  - read, write, shutdown, exit 
- 48bit of address space by MMU
    - multiple address space
- stack trace
  - Symbol parsing
- command-line interface(sh)

## Toolchain
- rust
  - target aarch64-unknown-none
  - rust-objcopy
- python3
- qemu-system-aarch64

## Build & Run

```
$ make 
```
```
$ make run 
$ make run FEATURES=test
$ make debug
```

## License

MIT License
