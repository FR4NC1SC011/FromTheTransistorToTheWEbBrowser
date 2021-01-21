package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

func main() {
	fmt.Println("Welcome to the GO_HACK Assembler")
	asmFilePath := os.Args[1]
	asmData, _ := ioutil.ReadFile(asmFilePath) // ignoring errors for brevity

	var p Parser
	p.Init(asmData)
	hackFile := p.Parse()

	var b bytes.Buffer
	for _, i := range hackFile.Instructions {
		b.WriteString(i.BinaryString())
	}

	hackFilePath := strings.Replace(asmFilePath, ".asm", ".hack", 1)
	ioutil.WriteFile(hackFilePath, b.Bytes(), 0644)
}
