# Based on http://os.phil-opp.com/multiboot-kernel.html

ARCH ?= x86_64
TARGET ?= $(ARCH)-unknown-none-gnu

RUST_OS := target/$(TARGET)/debug/librustos.a
KERNEL := build/kernel-$(ARCH).bin
ISO := build/os-$(ARCH).iso

LINKER_SCRIPT := src/arch/$(ARCH)/linker.ld
GRUB_CFG := src/arch/$(ARCH)/grub.cfg

ASMSRCFILES := $(wildcard src/arch/$(ARCH)/*.asm)
ASMOBJFILES := $(patsubst src/arch/$(ARCH)/%.asm, \
	build/arch/$(ARCH)/%.o, $(ASMSRCFILES))

.PHONY: all fmt clean run debug iso cargo

all: $(KERNEL)

fmt: 
	rustfmt --write-mode overwrite src/lib.rs

clean:
	rm -rf build target

run: $(ISO)
	@echo QEMU $(ISO)
	@qemu-system-$(ARCH) -hda $(ISO) -serial stdio

debug: $(ISO)
	@echo QEMU -d int $(ISO)
	@qemu-system-$(ARCH) -hda $(ISO) -d int -no-reboot -serial stdio

$(ISO): $(KERNEL) $(GRUB_CFG)
	@echo ISO $(ISO)
	@mkdir -p build/isofiles/boot/grub
	@cp $(KERNEL) build/isofiles/boot/kernel.bin
	@cp $(GRUB_CFG) build/isofiles/boot/grub
	@grub-mkrescue -o $(ISO) build/isofiles 2> /dev/null
	@rm -r build/isofiles	

$(KERNEL): cargo $(ASMOBJFILES) $(LINKER_SCRIPT)
	@echo LD $(KERNEL)
	@ld -m elf_$(ARCH) -n --gc-sections -T $(LINKER_SCRIPT) -o $@ $(ASMOBJFILES) $(RUST_OS)

cargo:
	@echo CARGO
	@cargo rustc --target $(TARGET) -- -Z no-landing-pads

build/arch/$(ARCH)/%.o: src/arch/$(ARCH)/%.asm
	@echo NASM $<
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 -o $@ $<

# Recompile Rust for our bare metal target
installed_target_libs := \
	$(shell multirust which rustc | \
		sed s,bin/rustc,lib/rustlib/$(target)/lib,)

runtime_rlibs := \
	$(installed_target_libs)/libcore.rlib

RUSTC := \
	rustc --verbose --target $(TARGET) \
		-Z no-landing-pads \
		--cfg disable_float \
		--out-dir $(installed_target_libs)

.PHONY: runtime

runtime: $(runtime_rlibs)

$(installed_target_libs):
	@mkdir -p $(installed_target_libs)

$(installed_target_libs)/%.rlib: rust/src/libcore/lib.rs $(installed_target_libs)
	@echo RUSTC $<
	@$(RUSTC) $<
	@echo Check $(installed_target_libs)