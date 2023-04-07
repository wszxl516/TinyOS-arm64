# Makefile

TARGET=kernel

# dir
override PWD=$(shell pwd)
override SRC_DIR=$(PWD)/src
override ARCH_DIR=$(SRC_DIR)/arch
override INC_DIR=$(PWD)/include
override BUILD_DIR:=$(PWD)/build
override OUT_DIR:=$(BUILD_DIR)/obj
override OUT_ARCH_DIR:=$(OUT_DIR)/arch

# build tools
override CC := aarch64-none-elf-gcc
override GDB := aarch64-linux-gnu-gdb
override OBJCOPY := aarch64-none-elf-objcopy
override QEMU = qemu-system-aarch64
override AS := $(CC)
override NM := aarch64-none-elf-nm

# build args
override define CFLAGS
	-g -Wall \
	-Wextra \
	-MMD \
	-fno-builtin \
	-nostdinc \
	-march=armv8.2-a \
	-mcpu=cortex-a76 \
	-Wa,-mcpu=cortex-a76 \
	-ffreestanding
endef

override define LDFLAGS
	-T linker.ld \
	-nostdlib \
	-Wl,--Map=$(BUILD_DIR)/kernel.map \
	-nostartfiles \
	-Wl,--no-warn-rwx-segment \
	-Wa,-mcpu=cortex-a76 \
	-Wl,--no-omagic \
	-Wl,--discard-none \
	-Wl,--check-sections \
	-Wl,--no-demangle \
	-Wl,--sort-section=name
endef
override EX_CFLAGS :=

# build file
override C_SRCS := $(shell find $(SRC_DIR) -name "*.c")
override ASM_SRCS := $(shell find $(ARCH_DIR) -name "*.S")
override OBJS = $(C_SRCS:$(SRC_DIR)/%.c= $(OUT_DIR)/%.o) $(ASM_SRCS:$(SRC_DIR)/%.S=$(OUT_DIR)/%_s.o)
override HEADERS = $(sort $(shell find $(INC_DIR) -name "*.h" -or -name "*.S";))
override INC_DIRS = $(addprefix -I, $(sort $(shell find $(INC_DIR) -type d))) 

# qemu args
# -device sdhci-pci -device sd-card,drive=hd0
# -device virtio-blk-device,scsi=off,drive=hd0
# -device virtio-blk-pci,scsi=off,drive=hd0
# -global virtio-mmio.force-legacy=false
# -device virtio-blk-device,scsi=off,drive=hd0
# -drive if=none,file=hd.img,format=raw,id=hd0
# -kernel $(BUILD_DIR)/$(TARGET).bin
# -chardev tty,id=ttys0 
# -nographic
define QEMU_ARGS
		-smp 2 \
		-cpu cortex-a76 \
		-m 128 \
		-machine virt,gic-version=2 \
		-chardev stdio,id=ttys0 \
		-serial chardev:ttys0 \
		-monitor tcp::1122,server,nowait \
		-nographic
endef

define QEMU_RUN_ARGS
	$(QEMU_ARGS) \
	-device loader,cpu-num=0,file=$(BUILD_DIR)/$(TARGET).bin,addr=0x040100000
endef

define QEMU_DEBUG_ARGS
	$(QEMU_RUN_ARGS) \
	-d in_asm,int,mmu,page \
	-D $(BUILD_DIR)/qemu.log \
	-s -S
endef

define generate_symbols
    $(NM) --defined-only --print-size --print-armap --size-sort --radix=x $1 | sort -s | \
	sed "/\.L\.str/d" | \
	awk -F " " '{ print "SYMBOL(" "0x" $$1 ", 0x0" $$2 ", \x27" $$3  "\x27, "  $$4  ")" }' > $2
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
$(OUT_DIR)/%.o: $(SRC_DIR)/%.c $(HEADERS) $(ASM_HEADERS)
	@echo [CC] $<
	@mkdir -p $(dir $@)
	@$(CC) -c -o $@ $< $(CFLAGS) $(EX_CFLAGS) $(INC_DIRS)

$(OUT_ARCH_DIR)/%_s.o: $(ARCH_DIR)/%.S $(HEADERS) $(ASM_HEADERS)
	@echo [AS] $<
	@mkdir -p $(dir $@)
	@$(AS) -c -o $@ $< $(CFLAGS) $(EX_CFLAGS) $(INC_DIRS)

$(BUILD_DIR)/$(TARGET).o: $(OBJS) 
	@echo [LINK] $@
	@$(CC) -nostdlib -Wl,-no-pie -Wl,-relocatable -o $(BUILD_DIR)/$(TARGET).o $(OBJS) $(CFLAGS)

$(BUILD_DIR)/__symbols__.o: $(BUILD_DIR)/$(TARGET).o
	@echo [SYMBOL] $@
	@$(call generate_symbols, $(BUILD_DIR)/$(TARGET).o, $(BUILD_DIR)/__symbols__.h)
	@$(CC) -x c  $(SRC_DIR)/mm/__symbols__ -c -o $(BUILD_DIR)/__symbols__.o $(CFLAGS) $(EX_CFLAGS) $(INC_DIRS) -I $(BUILD_DIR)

$(TARGET): $(BUILD_DIR)/__symbols__.o $(BUILD_DIR)/$(TARGET).o
	@echo [LINK] $@
	@$(CC) -o $(BUILD_DIR)/$(TARGET) $(BUILD_DIR)/$(TARGET).o $(BUILD_DIR)/__symbols__.o $(CFLAGS) $(LDFLAGS) $(EX_CFLAGS)

$(TARGET).bin: pre_check $(TARGET)
	@echo [BIN] $@
	@$(OBJCOPY) --strip-all -O binary $(BUILD_DIR)/$(TARGET) $(BUILD_DIR)/$(TARGET).bin


# build taget
.DEFAULT_GOAL := bin

all: bin

bin: $(TARGET).bin

#@lldb -O "target create $(BUILD_DIR)/$(TARGET)" -O "gdb-remote localhost:1234"
debug: all
	@/usr/bin/xfce4-terminal -e \
		'$(QEMU) $(QEMU_DEBUG_ARGS)'
	@$(GDB) $(BUILD_DIR)/$(TARGET) -ex "target remote :1234"

run: $(TARGET).bin
	@$(QEMU) $(QEMU_RUN_ARGS)

dump_dtb:
	@$(QEMU) $(QEMU_ARGS) -machine dumpdtb=$(BUILD_DIR)/aarch64-virt.dtb 
	@dtc -O dts -o $(BUILD_DIR)/aarch64-virt.dts  $(BUILD_DIR)/aarch64-virt.dtb
	@rm $(BUILD_DIR)/aarch64-virt.dtb -f
	@echo "$(BUILD_DIR)/aarch64-virt.dts dumped"

clean:
	@rm $(BUILD_DIR)/ -rf


