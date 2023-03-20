# Makefile

TARGET=kernel.elf

# dir
override PWD=$(shell pwd)
override SRC_DIR=$(PWD)/src
override ARCH_DIR=$(SRC_DIR)/arch
override INC_DIR=$(PWD)/include
override BUILD_DIR:=$(PWD)/build
override OUT_DIR:=$(BUILD_DIR)/obj
override OUT_ARCH_DIR:=$(OUT_DIR)/arch

# build tools
# override CC := clang --target=aarch64-none-unknown-elf --sysroot=/usr/aarch64-none-elf/
override CC := aarch64-none-elf-gcc
override GDB := aarch64-linux-gnu-gdb
override OBJCOPY := aarch64-none-elf-objcopy
override QEMU = qemu-system-aarch64
override AS := $(CC)

# build args
override CFLAGS  = -g -Wall -Wextra -MMD -fno-builtin -nostdinc -march=armv8.2-a -mcpu=cortex-a76 -Wa,-mcpu=cortex-a76 -ffreestanding
override LDFLAGS = -T linker.ld -nostdlib -g -Wl,--Map=$(BUILD_DIR)/kernel.map -nostartfiles -Wl,--no-warn-rwx-segment -Wa,-mcpu=cortex-a76
override EX_CFLAGS :=

# build file
override C_SRCS := $(shell find $(SRC_DIR) -name "*.c")
override ARCH_SRCS := $(shell find $(ARCH_DIR) -name "*.S")
override OBJS = $(C_SRCS:$(SRC_DIR)/%.c= $(OUT_DIR)/%.o)
override OBJS += $(ARCH_SRCS:$(SRC_DIR)/%.S=$(OUT_DIR)/%_s.o)
override HEADERS = $(sort $(shell find $(INC_DIR) -name "*.h";))
override INC_DIRS = $(addprefix -I, $(sort $(shell find $(INC_DIR) -name "*.h" -exec dirname {} \;)))

# qemu args
# -device sdhci-pci -device sd-card,drive=hd0
# -device virtio-blk-device,scsi=off,drive=hd0
# -device virtio-blk-pci,scsi=off,drive=hd0
# -global virtio-mmio.force-legacy=false
# -device virtio-blk-device,scsi=off,drive=hd0
# -drive if=none,file=hd.img,format=raw,id=hd0
# -kernel $(BUILD_DIR)/$(TARGET).bin
define QEMU_ARGS
		-smp 2 \
		-cpu cortex-a76 \
		-machine virt \
		-chardev stdio,id=ttys0 \
		-serial chardev:ttys0 \
		-monitor tcp::1122,server,nowait \
		-nographic \
		-device loader,file=$(BUILD_DIR)/$(TARGET),addr=0x040100000 \
		-device loader,addr=0x040100000,cpu-num=0
endef

# Depencies
-include $(OBJS:%.o=%.d)

pre_check:
	@if [ ! -f "$$(which $(CC))" ] || \
		[ ! -f "$$(which $(GDB))" ] || \
		[ ! -f "$$(which $(QEMU))" ];then \
		echo -ne "\033[91mMust install $(CC), $(GDB), $(QEMU) and $(OBJCOPY)!\033[0m\n"; \
		exit 1; \
	fi
	@mkdir -p $(OUT_DIR)

# build compile and link
$(OUT_DIR)/%.o: $(SRC_DIR)/%.c $(HEADERS)
	@echo [CC] $<
	@mkdir -p $(dir $@)
	@$(CC) -c -o $@ $< $(CFLAGS) $(EX_CFLAGS) $(INC_DIRS)

$(OUT_ARCH_DIR)/%_s.o: $(ARCH_DIR)/%.S $(HEADERS)
	@echo [AS] $<
	@mkdir -p $(dir $@)
	@$(AS) -c -o $@ $< $(CFLAGS) $(EX_CFLAGS) $(INC_DIRS)

$(TARGET): pre_check $(OBJS) 
	@echo [LINK] $@
	@$(CC) -o $(BUILD_DIR)/$(TARGET) $(OBJS) $(CFLAGS) $(LDFLAGS) $(EX_CFLAGS)

$(BUILD_DIR)/$(TARGET).bin: pre_check $(TARGET)
	@echo [BIN] $@
	@$(OBJCOPY) --strip-all -O binary $(BUILD_DIR)/$(TARGET) $(BUILD_DIR)/$(TARGET).bin


# build taget
.DEFAULT_GOAL := bin

all: bin

bin: $(BUILD_DIR)/$(TARGET).bin

#@lldb -O "target create $(BUILD_DIR)/$(TARGET)" -O "gdb-remote localhost:1234"
debug: all
	@/usr/bin/xfce4-terminal -e \
		'$(QEMU) $(QEMU_ARGS) -s -S'
	#@lldb -O "target create $(BUILD_DIR)/$(TARGET)" -O "gdb-remote localhost:1234"
	@$(GDB) $(BUILD_DIR)/$(TARGET) -ex "target remote :1234"

run: $(BUILD_DIR)/$(TARGET).bin
	@$(QEMU) $(QEMU_ARGS)

test: test_pre run

dump_dtb:
	@$(QEMU) $(QEMU_ARGS) -machine dumpdtb=$(BUILD_DIR)/aarch64-virt.dtb > /dev/null 2>&1
	@dtc -O dts -o $(BUILD_DIR)/aarch64-virt.dts  $(BUILD_DIR)/aarch64-virt.dtb > /dev/null 2>&1
	@rm $(BUILD_DIR)/aarch64-virt.dtb -f
	@echo "$(BUILD_DIR)/aarch64-virt.dts dumped"

clean:
	@rm $(BUILD_DIR)/ -rf

test_pre: clean
	$(eval EX_CFLAGS= -D__RUN_TEST__)

