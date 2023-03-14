
TARGET=kernel.elf
override PWD=$(shell pwd)
override SRC_DIR=$(PWD)/src
override ASM_DIR=$(SRC_DIR)
CC := aarch64-linux-gnu-gcc
AS := $(CC)
GDB := aarch64-linux-gnu-gdb
EX_CFLAGS := 
OUT_DIR :=$(PWD)/out

override C_SRCS := $(shell find $(SRC_DIR) -name "*.c")
override ASM_SRCS := $(shell find $(ASM_DIR) -name "*.S")
override INCLUDE := $(foreach dir, $(shell find $(PWD)/include -type d), -I$(dir))
override HEADERS := $(shell find $(PWD)/include -name "*.h")

override OBJS = $(C_SRCS:$(SRC_DIR)/%.c= $(OUT_DIR)/%.o) $(ASM_SRCS:$(SRC_DIR)/%.c= $(OUT_DIR)/%.o)
EX_CFLAGS=
override CFLAGS  = -g -Wall $(INCLUDE)
override LDFLAGS = -T linker.ld -lgcc -nostdlib -g -Wl,--Map=$(OUT_DIR)/kernel.map
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
		-kernel $(OUT_DIR)/$(TARGET).bin
endef

$(OUT_DIR)/%.o: $(SRC_DIR)/%.c $(HEADERS)
	@echo [CC] $<
	@mkdir -p $(dir $@)
	@$(CC) -c -o $@ $< $(CFLAGS) $(EX_CFLAGS)

$(OUT_DIR)/%.o: $(ASM_DIR)/%.S $(HEADERS)
	@echo [AS] $<
	@mkdir -p $(dir $@)
	@$(AS) -c -o $@ $< $(CFLAGS) $(EX_CFLAGS)

$(TARGET): pre_check $(OBJS) 
	@echo [LINK] $@
	@$(CC) -o $(OUT_DIR)/$(TARGET) $(OBJS) $(CFLAGS)  $(LDFLAGS) $(EX_CFLAGS)

.DEFAULT_GOAL := all

all: $(OUT_DIR)/$(TARGET).bin

$(OUT_DIR)/$(TARGET).bin: pre_check $(TARGET)
	@echo Build $@
	@rust-objcopy --binary-architecture=aarch64 --strip-all -O binary $(OUT_DIR)/$(TARGET) $(OUT_DIR)/$(TARGET).bin


run: $(OUT_DIR)/$(TARGET).bin
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
	@/usr/bin/xfce4-terminal -e \
		'$(QEMU) $(QEMU_ARGS) -s -S'
	#@lldb -O "target create $(OUT_DIR)/$(TARGET)" -O "gdb-remote localhost:1234"
	@riscv64-linux-gnu-gdb $(OUT_DIR)/$(TARGET) -ex "target remote :1234"

test_pre: clean
	$(eval EX_CFLAGS= -D__RUN_TEST__)

test: test_pre run

dump_dtb:
	@$(QEMU) -smp 2 -machine virt -cpu rv64 -machine dumpdtb=$(OUT_DIR)/riscv64-virt.dtb > /dev/null 2>&1
	@dtc -O dts -o $(OUT_DIR)/riscv64-virt.dts  $(OUT_DIR)/riscv64-virt.dtb > /dev/null 2>&1
	@rm $(OUT_DIR)/riscv64-virt.dtb -f
	@echo "$(OUT_DIR)/riscv64-virt.dts dumped"

clean:
	@rm $(OUT_DIR)/ -rf
