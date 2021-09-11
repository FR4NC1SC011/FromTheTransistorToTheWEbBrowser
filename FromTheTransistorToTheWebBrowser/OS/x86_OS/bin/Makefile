# Crate the OS.bin file (use 'Make OS')
OS: bootSect.bin kernel.bin fileTable.bin calculator.bin
	cat bootSect.bin kernel.bin fileTable.bin calculator.bin > OS.bin

bootSect.bin:
	fasm ../src/asm/bootSect.asm
	mv ../src/asm/bootSect.bin ./

fileTable.bin:
	fasm ../src/asm/fileTable.asm
	mv ../src/asm/fileTable.bin ./

kernel.bin:
	fasm ../src/asm/kernel.asm
	mv ../src/asm/kernel.bin ./

calculator.bin:
	fasm ../src/asm/calculator.asm
	mv ../src/asm/calculator.bin ./

clean:
	rm *.bin
