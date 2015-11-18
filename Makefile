all:
	cd src/arch/i386; make
	cd src/arch/x86_64; make

run:
	cd src/arch/i386; make run

clean:
	cd src/arch/i386; make clean
	cd src/arch/x86_64; make clean