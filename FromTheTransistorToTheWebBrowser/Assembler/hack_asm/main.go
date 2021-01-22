package main

import (
	"flag"
	"fmt"
)

func main() {
	hack_file := flag.String("file", "hack.asm", "The hack file")
	flag.Parse()

	instructions := ReadFile(*hack_file)

	for n, inst := range instructions {
		fmt.Println(n, inst)
	}

	Parse(instructions)

}
