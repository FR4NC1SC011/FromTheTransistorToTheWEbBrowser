package main

import (
	"debug/elf"
	"fmt"
	"io"
	"os"
)

var elfValues = make(map[string]string)

func ioReader(file string) io.ReaderAt {
	r, err := os.Open(file)
	check(err)
	return r
}

func ReadElf(elf_object string) {
	f := ioReader(elf_object)
	_elf, err := elf.NewFile(f)
	check(err)

	//Read ELF identifier
	var ident [16]uint8
	f.ReadAt(ident[0:], 0)
	check(err)

	if ident[0] != '\x7f' || ident[1] != 'E' || ident[2] != 'L' || ident[3] != 'F' {
		fmt.Printf("Bad magic number at %d\n", ident[0:4])
		os.Exit(1)
	}

	var arch string
	switch _elf.Class.String() {
	case "ELFCLASS64":
		arch = "64 bits"
	case "ELFCLASS32":
		arch = "32 bits"
	default:
		arch = "Unrecognized"
	}

	var mach string
	switch _elf.Machine.String() {
	case "EM_AARCH64":
		mach = "ARM64"
	case "EM_386":
		mach = "x86"
	case "EM_X86_64":
		mach = "x86_64"
	default:
		mach = "Unrecognized"
	}

	elfValues["File Header"] = fmt.Sprintf("%s", _elf.FileHeader)
	elfValues["ELF Class"] = arch
	elfValues["Machine"] = mach
	elfValues["ElF Type"] = fmt.Sprintf("%s", _elf.Type)
	elfValues["ELF Data"] = fmt.Sprintf("%s", _elf.Data)
	elfValues["Entry Point"] = fmt.Sprintf("%s", _elf.Entry)
	//elfValues["Section Addresses"] = fmt.Sprintf("%s", _elf.Sections)

	/*
		fmt.Printf("File Header: ")
		fmt.Println(_elf.FileHeader)
		//fmt.Printf("Magic: %v\n", ident) // to do: Translate from int to hex
		fmt.Printf("ELF Class: %s\n", arch)
		fmt.Printf("Machine: %s\n", mach)
		fmt.Printf("ELF Type: %s\n", _elf.Type)
		fmt.Printf("ELF Data: %s\n", _elf.Data)
		fmt.Printf("Entry Point: %d\n", _elf.Entry)
		fmt.Printf("Section Addresses: %d\n", _elf.Sections)
		//fmt.Printf("OS: %d\n", _elf.OSABI) // Switch maybe...
	*/

	fmt.Println("MAP:")
	for k, v := range elfValues {
		fmt.Println(k, ":", v)
	}

}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
