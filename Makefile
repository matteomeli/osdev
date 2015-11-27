SHELL := /bin/bash

ARCH ?= x86_64

BUILD := build/$(ARCH)
KERNEL := $(BUILD)/kernel.bin
ISO := $(BUILD)/os.iso

LINKER_SCRIPT := src/linker.ld
GRUB_MAKE := grub-mkrescue

AS = nasm
LD = ld
MKDIR = mkdir -p
CP = cp
RM = rm

GRUB_CFG := src/grub.cfg
TARGET = $(ARCH)-unknown-linux-gnu
RUST_OS := target/$(TARGET)/debug/librustos.a

ASMSRCFILES := $(wildcard src/*.asm)
ASMOBJFILES := $(patsubst src/%.asm, $(BUILD)/%.o, $(ASMSRCFILES))

.PHONY: directories all iso qemu clean

$(BUILD):
	$(MKDIR) $@

directories: $(BUILD)

all: directories $(KERNEL)

run: $(ISO)
	qemu-system-$(ARCH) -hda $(ISO)

iso: $(ISO)

$(ISO): all
	$(MKDIR) $(BUILD)/iso/boot/grub
	$(CP) $(KERNEL) $(BUILD)/iso/boot/kernel.bin
	$(CP) $(GRUB_CFG) $(BUILD)/iso/boot/grub
	$(GRUB_MAKE) -o $(ISO) $(BUILD)/iso 2> /dev/null
	$(RM) -r $(BUILD)/iso

$(KERNEL): cargo $(ASMOBJFILES) $(LINKER_SCRIPT)
	$(LD) -m elf_$(ARCH) -n --gc-sections -T $(LINKER_SCRIPT) -o $@ $(ASMOBJFILES) $(RUST_OS)

cargo:
	@cargo rustc --target $(TARGET) -- -Z no-landing-pads

$(BUILD)/%.o: src/%.asm
	$(AS) -f elf64 -o $@ $<

clean:
	$(RM) -rf build target
