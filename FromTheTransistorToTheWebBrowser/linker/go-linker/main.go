package main

import (
	"fmt"
	"os"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: go-linker elf_file")
		os.Exit(1)
	}

	ReadElf(os.Args[1])
}
