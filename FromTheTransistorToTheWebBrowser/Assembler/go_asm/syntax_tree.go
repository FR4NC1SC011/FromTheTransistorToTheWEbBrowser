package main

import "fmt"

type Instruction interface {
	BinaryString() string
}

type HackFile struct {
	Instructions []Instruction
}

type AInstruction struct {
	lit  string
	addr int
}

type CInstruction struct {
	lit  string
	dest string
	comp string
	jump string
}

func (a *AInstruction) BinaryString() string {
	return fmt.Sprintf("0%015b\n", a.addr)
}

func (c *CInstruction) BinaryString() string {
	return fmt.Sprintf("111%07b%03b%03b\n", c.comp, c.dest, c.jump)
}
