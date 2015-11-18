OBJECTS = multiboot_header.o boot.o 
LDFLAGS = -n -T link.ld
AS = nasm
ASFLAGS = -f elf64

all: kernel.elf

kernel.elf: $(OBJECTS)
	ld $(LDFLAGS) $(OBJECTS) -o iso/boot/kernel.elf

os.iso: kernel.elf
	grub-mkrescue -o os.iso iso

run: os.iso
	bochs -f bochsrc.txt -q

%.o: %.s
	$(AS) $(ASFLAGS) $< -o $@

clean:
	rm -rf *.o iso/boot/kernel.elf os.iso