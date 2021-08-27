# Crate the OS.bin file (use 'Make OS')
OS: bootSect.bin fileTable.bin kernel.bin
	cat bootSect.bin fileTable.bin kernel.bin > OS.bin

bootSect.bin:
	fasm ../src/asm/bootSect.asm
	mv ../src/asm/bootSect.bin ./

fileTable.bin:
	fasm ../src/asm/fileTable.asm
	mv ../src/asm/fileTable.bin ./

kernel.bin:
	fasm ../src/asm/kernel.asm
	mv ../src/asm/kernel.bin ./

clean:
	rm *.bin
