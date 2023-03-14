# Makefile

TARGET=kernel.elf

# dir
override PWD=$(shell pwd)
override SRC_DIR=$(PWD)/src
override ASM_DIR=$(SRC_DIR)/asm
override INC_DIR=$(PWD)/include
override BUILD_DIR:=$(PWD)/build
override OUT_DIR:=$(BUILD_DIR)/out

# build tools
override CC := aarch64-linux-gnu-gcc
override GDB := aarch64-linux-gnu-gdb
override OBJCOPY := aarch64-linux-gnu-objcopy
override QEMU = qemu-system-aarch64
override AS := $(CC)

# build args
override CFLAGS  = -g -Wall
override LDFLAGS = -T linker.ld -lgcc -nostdlib -g -Wl,--Map=$(OUT_DIR)/kernel.map
override EX_CFLAGS :=

# build file
override C_SRCS := $(shell find $(SRC_DIR) -name "*.c")
override ASM_SRCS := $(shell find $(ASM_DIR) -name "*.S")
override OBJS = $(C_SRCS:$(SRC_DIR)/%.c= $(OUT_DIR)/%.o)
override OBJS += $(ASM_SRCS:$(SRC_DIR)/%.S=$(OUT_DIR)/%_s.o)
override INCS = $(sort $(shell find $(INC_DIR) -name "*.h" -exec dirname {} \;))
override INCLUDES = $(addprefix -I, $(INC_DIR_COMMON) $(INCS))

# qemu args
# -device sdhci-pci -device sd-card,drive=hd0
# -device virtio-blk-device,scsi=off,drive=hd0
# -device virtio-blk-pci,scsi=off,drive=hd0
# -global virtio-mmio.force-legacy=false
# -device virtio-blk-device,scsi=off,drive=hd0
# -drive if=none,file=hd.img,format=raw,id=hd0
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

# Depencies
-include $(OBJS:%.o=%.d)

pre_check:
	@if [ ! -f "$$(which $(CC))" ] || \
		[ ! -f "$$(which $(GDB))" ] || \
		[ ! -f "$$(which $(QEMU))" ];then \
		echo -ne "\033[91mMust install aarch64-linux-gnu-gcc, aarch64-linux-gnu-gdb and aarch64-linux-gnu-objcopy!\033[0m\n"; \
		exit 1; \
	fi
	@mkdir -p $(OUT_DIR)

# build compile and link
$(OUT_DIR)/%.o: $(SRC_DIR)/%.c $(INCLUDES)
	@echo [CC] $<
	@mkdir -p $(dir $@)
	@$(CC) -c -o $@ $< $(CFLAGS) $(EX_CFLAGS) $(INCLUDES)

$(OUT_DIR)/asm/%_s.o: $(ASM_DIR)/%.S $(INCLUDES)
	@echo [CC] $<
	@echo [AS] $<
	@mkdir -p $(dir $@)
	@$(AS) -c -o $@ $< $(CFLAGS) $(EX_CFLAGS) $(INCLUDES)

$(TARGET): pre_check $(OBJS) 
	@echo [LINK] $@
	@$(CC) -o $(OUT_DIR)/$(TARGET) $(OBJS) $(CFLAGS)  $(LDFLAGS) $(EX_CFLAGS)

$(OUT_DIR)/$(TARGET).bin: pre_check $(TARGET)
	@echo Build $@
	@$(OBJCOPY) -O binary $(OUT_DIR)/$(TARGET) $(OUT_DIR)/$(TARGET).bin


# build taget
.DEFAULT_GOAL := bin

all: bin

bin: $(OUT_DIR)/$(TARGET).bin

debug: all
	@/usr/bin/xfce4-terminal -e \
		'$(QEMU) $(QEMU_ARGS) -s -S'
	#@lldb -O "target create $(OUT_DIR)/$(TARGET)" -O "gdb-remote localhost:1234"
	@aarch64-linux-gnu-gdb $(OUT_DIR)/$(TARGET) -ex "target remote :1234"

run: $(OUT_DIR)/$(TARGET).bin
	@$(QEMU) $(QEMU_ARGS)

test: test_pre run

dump_dtb:
	@$(QEMU) -smp 2 -machine virt -cpu rv64 -machine dumpdtb=$(OUT_DIR)/aarch64-virt.dtb > /dev/null 2>&1
	@dtc -O dts -o $(OUT_DIR)/aarch64-virt.dts  $(OUT_DIR)/aarch64-virt.dtb > /dev/null 2>&1
	@rm $(OUT_DIR)/aarch64-virt.dtb -f
	@echo "$(OUT_DIR)/aarch64-virt.dts dumped"

clean:
	@rm $(BUILD_DIR)/ -rf

test_pre: clean
	$(eval EX_CFLAGS= -D__RUN_TEST__)

