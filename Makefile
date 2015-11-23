ARCH ?= i386
#ARCH ?= x86_64

BUILD := build/$(ARCH)
KERNEL := $(BUILD)/kernel.bin
ISO := $(BUILD)/os.iso

LINKER_SCRIPT := src/linker-$(ARCH).ld
GRUB_MAKE := grub-mkrescue

AS = nasm
LD = ld
LDFLAGS = -m elf_$(ARCH)
MKDIR = mkdir -p
CP = cp
RM = rm

ifeq ($(ARCH), x86_64)
	GRUB_CFG := src/grub.cfg
else
	GRUB_CFG := src/menu.lst
endif

ifeq ($(ARCH), x86_64)
	ASFLAGS = -f elf64
else
	ASFLAGS = -f elf
endif

ifeq ($(ARCH), x86_64)
	TARGET = x86_64-unknown-linux-gnu
else
	TARGET = i686-unknown-linux-gnu
endif

RUST_OS := target/$(TARGET)/debug/librustos.a

.PHONY: directories all iso qemu clean

$(BUILD):
	$(MKDIR) $@

directories: $(BUILD)

all: directories $(KERNEL)

qemu: $(ISO)
	@qemu-system-$(ARCH) -hda $(ISO)

iso: $(ISO)

$(ISO): $(KERNEL) $(GRUB_CFG)
	$(MKDIR) $(BUILD)/iso/boot/grub
	$(CP) $(KERNEL) $(BUILD)/iso/boot/kernel.bin
	$(CP) $(GRUB_CFG) $(BUILD)/iso/boot/grub
	$(GRUB_MAKE) -o $(ISO) $(BUILD)/iso 2> /dev/null
	$(RM) -r $(BUILD)/iso

$(KERNEL): cargo $(BUILD)/kernel.o $(LINKER_SCRIPT)
	$(LD) $(LDFLAGS) -o $@ -n -T $(LINKER_SCRIPT) $(BUILD)/kernel.o $(RUST_OS)

cargo:
	@cargo build --target $(TARGET)

$(BUILD)/kernel.o: src/kernel.asm
	$(AS) $(ASFLAGS) -o $@ -D ARCH_$(ARCH) $< -isrc/

clean:
	$(RM) -rf build target
