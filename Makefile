all:
	cd src/arch/i386; make
	cd src/arch/x86_64; make

run_32:
	cd src/arch/i386; make run

run_64:
	cd src/arch/x86_64; make run

clean:
	cd src/arch/i386; make clean
	cd src/arch/x86_64; make clean