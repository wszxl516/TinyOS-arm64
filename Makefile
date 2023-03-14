
TARGET=kernel.elf
override PWD=$(shell pwd)
override SRC_DIR=$(PWD)/src
CC := aarch64-none-elf-gcc
AS := $(CC)
GDB := aarch64-linux-gnu-gdb
OBJCOPY := aarch64-none-elf-objcopy
EX_CFLAGS := 
OUT_DIR :=$(PWD)/out

override C_SRCS := $(shell find $(SRC_DIR) -name "*.c")
override ASM_SRCS := $(shell find $(SRC_DIR) -name "*.S")
override INCLUDE := $(foreach dir, $(shell find $(PWD)/include -type d), -I$(dir))
override HEADERS := $(shell find $(PWD)/include -name "*.h")

override OBJS = $(C_SRCS:$(SRC_DIR)/%.c= $(OUT_DIR)/%.o) $(ASM_SRCS:$(SRC_DIR)/%.S= $(OUT_DIR)/%.o)
EX_CFLAGS=
override CFLAGS  = -g -Wall $(INCLUDE) -fno-builtin -nostdinc
override LDFLAGS = -T linker.ld -nostdlib -g -Wl,--Map=$(OUT_DIR)/kernel.map -nostartfiles -Wl,--no-warn-rwx-segment
override QEMU = qemu-system-aarch64

#-device sdhci-pci -device sd-card,drive=hd0
#-device virtio-blk-device,scsi=off,drive=hd0
#-device virtio-blk-pci,scsi=off,drive=hd0
#-global virtio-mmio.force-legacy=false 
#-device virtio-blk-device,scsi=off,drive=hd0 
#-drive if=none,file=hd.img,format=raw,id=hd0 
define QEMU_ARGS
		-smp 2 \
		-cpu cortex-a72 \
		-machine virt \
		-chardev stdio,id=ttys0 \
		-serial chardev:ttys0 \
		-monitor tcp::1122,server,nowait \
		-nographic \
		-kernel $(OUT_DIR)/$(TARGET)
endef

$(OUT_DIR)/%.o: $(SRC_DIR)/%.c $(HEADERS)
	@echo [CC] $<
	@mkdir -p $(dir $@)
	@$(CC) -c -o $@ $< $(CFLAGS) $(EX_CFLAGS)

$(OUT_DIR)/%.o: $(SRC_DIR)/%.S $(HEADERS)
	@echo [AS] $<
	@mkdir -p $(dir $@)
	@$(AS) -c -o $@ $< $(CFLAGS) $(EX_CFLAGS)

$(OUT_DIR)/$(TARGET): pre_check $(OBJS) 
	@echo [LINK] $@
	@$(CC) -o $(OUT_DIR)/$(TARGET) $(OBJS) $(CFLAGS)  $(LDFLAGS) $(EX_CFLAGS)

.DEFAULT_GOAL := all

all: $(OUT_DIR)/$(TARGET)

run: $(OUT_DIR)/$(TARGET)
	@$(QEMU) $(QEMU_ARGS)

pre_check:
	@if [ ! -f "$$(which $(CC))" ] || \
		[ ! -f "$$(which $(GDB))" ] || \
		[ ! -f "$$(which $(QEMU))" ];then \
		echo -ne "\033[91mMust install riscv64-elf-gcc, riscv64-elf-gdb and qemu-system-riscv64!\033[0m\n"; \
		exit 1; \
	fi
	@mkdir -p $(OUT_DIR)

debug: all
	/usr/bin/xfce4-terminal -e \
		'$(QEMU) $(QEMU_ARGS) -s -S'
	#@lldb -O "target create $(OUT_DIR)/$(TARGET)" -O "gdb-remote localhost:1234"
	@$(GDB) $(OUT_DIR)/$(TARGET) -ex "target remote :1234"

dump_dtb:
	@$(QEMU) $(QEMU_ARGS) -machine dumpdtb=$(OUT_DIR)/aarch64-virt.dtb > /dev/null 2>&1
	@dtc -O dts -o $(OUT_DIR)/aarch64-virt.dts  $(OUT_DIR)/aarch64-virt.dtb > /dev/null 2>&1
	@rm $(OUT_DIR)/aarch64-virt.dtb -f
	@echo "$(OUT_DIR)/aarch64-virt.dts dumped"

clean:
	@rm $(OUT_DIR)/ -rf
