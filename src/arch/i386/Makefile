OBJECTS = loader.o
LDFLAGS = -T link.ld -melf_i386
AS = nasm
ASFLAGS = -f elf

ARCH = i686
TARGET ?= $(ARCH)-unknown-linux-gnu
RUST_OS := ../../../target/$(TARGET)/debug/libmy_os.a

all: kernel.elf

kernel.elf: cargo $(OBJECTS)
	ld $(LDFLAGS) $(OBJECTS) $(RUST_OS) -o iso/boot/kernel.elf

os.iso: kernel.elf
	genisoimage -R                              \
				-b boot/grub/stage2_eltorito    \
				-no-emul-boot                   \
				-boot-load-size 4               \
				-A os                           \
				-input-charset utf8             \
				-quiet                          \
				-boot-info-table                \
				-o os.iso                       \
				iso

run: os.iso
	bochs -f bochsrc.txt -q

%.o: %.s
	$(AS) $(ASFLAGS) $< -o $@

cargo:
	cd ../../../
	@cargo build --target $(TARGET)

clean:
	rm -rf *.o iso/boot/kernel.elf os.iso bochslog.txt