package main

import (
	"flag"
)

func main() {
	hack_file := flag.String("file", "hack.asm", "The hack file")
	flag.Parse()

	instructions := ReadFile(*hack_file)

	Parse(instructions)

}
