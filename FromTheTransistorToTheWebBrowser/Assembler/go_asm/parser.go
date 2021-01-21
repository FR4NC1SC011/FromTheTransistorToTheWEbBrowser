package main

import "fmt"

type Parser struct {
	scanner      Scanner
	symbols      map[string]int
	instructions []Instruction
	nAddr        int
}

func (p *Parser) Init(src []byte) {
	p.scanner.Init(src)
	p.nAddr = 16
	p.symbols = map[string]int{
		"R0": 0, "R1": 1, "R2": 2, "R3": 3, "R4": 4, "R5": 5, "R6": 6, "R7": 7, "R8": 8,
		"R9": 9, "R10": 10, "R11": 11, "R12": 12, "R13": 13, "R14": 14, "R15": 15,
		"SCREEN": 16384, "KBD": 24576,
		"SP": 0, "LCL": 1, "ARG": 2, "THIS": 3, "THAT": 4,
	}
}

func (p *Parser) Parse() HackFile {
loop:
	for {
		tok, lit := p.scanner.Scan()
		switch tok {
		case EOF:
			break loop
		case LABEL:
			p.symbols[lit] = len(p.instructions)
		case A_INSTRUCTION:
			p.instructions = append(p.instructions, &AInstruction{lit: lit})
		case C_INSTRUCTION:
			p.instructions = append(p.instructions, &CInstruction{lit: lit})
		}
	}

	for _, instr := range p.instructions {
		switch i := instr.(type) {
		case *AInstruction:
			p.parseAInstruction(i)
		case *CInstruction:
			p.parseCInstruction(i)
		}
	}
	return HackFile{Instructions: p.instructions}
}

func (p *Parser) parseAInstruction(i *AInstruction) {
	fmt.Println("Esto es una Instruccion typo 'A'")
	fmt.Println(i)

}

func (p *Parser) parseCInstruction(i *CInstruction) {
	fmt.Println("Esto es una Instruccion typo 'C'")
	fmt.Println(i)

}
