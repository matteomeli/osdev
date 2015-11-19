OBJECTS = multiboot_header.o boot.o 
LDFLAGS = -n -T link.ld
AS = nasm
ASFLAGS = -f elf64

ARCH = x86_64
TARGET ?= $(ARCH)-unknown-linux-gnu
RUST_OS := ../../../target/$(TARGET)/debug/libmy_os.a

all: kernel.elf

kernel.elf: cargo $(OBJECTS)
	ld $(LDFLAGS) $(OBJECTS) $(RUST_OS) -o iso/boot/kernel.elf

os.iso: kernel.elf
	grub-mkrescue -o os.iso iso

run: os.iso
	bochs -f bochsrc.txt -q

%.o: %.s
	$(AS) $(ASFLAGS) $< -o $@

cargo:
	cd ../../../
	@cargo build --target $(TARGET)

clean:
	rm -rf *.o iso/boot/kernel.elf os.iso bochslog.txt