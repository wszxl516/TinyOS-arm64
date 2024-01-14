.PHONY: clean all user kernel run img
override KERNEL_TARGET=aarch64-minimal
override USER_TARGET=init
override PWD = $(shell pwd)
override QEMU = qemu-system-aarch64
override GDB := rust-gdb
override GEN_SYMBOLS = ./parse_symbol.py
override OUT_DIR=$(PWD)/target/aarch64-unknown-none/debug
override NO_OUTPUT= > /dev/null 2>&1
override KERNEL_BINARY=$(OUT_DIR)/$(KERNEL_TARGET).bin
FEATURES :=
all: $(KERNEL_BINARY)

define generate_symbols
    $(GEN_SYMBOLS) $1 $2 $3 -v
endef

#-device loader,file=$(OUT_DIR)/$(TARGET).bin,addr=0x040100000,cpu-num=0,force-raw=on
#,virtualization=true,secure=on
define QEMU_ARGS_RUN
		-smp 2 \
		-m 128M \
		-cpu cortex-a72 \
		-machine virt,gic-version=2,acpi=off \
		-chardev stdio,id=ttys0,signal=on \
		-serial chardev:ttys0 \
		-d in_asm,mmu -D ./qemu.log \
		-monitor tcp::1122,server,nowait, \
		-device virtio-blk-pci,drive=hd0 \
		-drive if=none,file=hd.img,format=raw,id=hd0 \
		-nographic \
		-kernel $(OUT_DIR)/$(KERNEL_TARGET).bin
endef
kernel: user
	@echo Build $@
	@INIT_BIN=$(OUT_DIR)/init.bin cargo build --features=$(FEATURES)
	@$(call generate_symbols, $(OUT_DIR)/$(KERNEL_TARGET), $(OUT_DIR)/symbol_section , 262144) > symbols.log
	@rust-objcopy --update-section .symbols=$(OUT_DIR)/symbol_section --set-section-flags .symbols=data,contents,alloc,load $(OUT_DIR)/$(KERNEL_TARGET)

$(KERNEL_BINARY): kernel
	@echo Build $@
	@rust-objcopy --binary-architecture=aarch64 --strip-debug -O binary $(OUT_DIR)/$(KERNEL_TARGET) $(OUT_DIR)/$(KERNEL_TARGET).bin

user:
	@cd user && cargo b --target-dir=../target
	@rust-objcopy  --binary-architecture=aarch64 --strip-debug -O binary $(OUT_DIR)/$(USER_TARGET) $(OUT_DIR)/$(USER_TARGET).bin

hd.img:
	@dd if=/dev/zero of=hd.img bs=1M count=128 > /dev/null 2>&1
	@mkfs.fat -F 32 hd.img > /dev/null 2>&1
run: $(KERNEL_BINARY) hd.img
	@$(QEMU) $(QEMU_ARGS_RUN)



debug: kernel hd.img
	@/usr/bin/xfce4-terminal -e '$(QEMU) $(QEMU_ARGS_RUN) -s -S'
	#@rust-lldb -O "target create $(OUT_DIR)/$(TARGET)" -O "gdb-remote localhost:1234"
	@RUST_GDB=aarch64-linux-gnu-gdb $(GDB)  $(OUT_DIR)/$(KERNEL_TARGET)  -ex "target remote :1234"

dump_dts: hd.img
	@$(QEMU) $(QEMU_ARGS_RUN) -machine dumpdtb=arm64-virt.dtb $(NO_OUTPUT)
	@dtc -O dts -o ./arm64-virt.dts  ./arm64-virt.dtb $(NO_OUTPUT)
	@rm ./arm64-virt.dtb -f
	@echo "./arm64-virt.dts dumped"


clean:
	@echo clean $(TARGET)...
	@cargo clean && cd user && cargo clean
	@rm -f $(TARGET).bin
	@rm -f os/os.map
