package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
)

var atable = make(map[string]string)
var ltable = make(map[string]string)
var ctable = make(map[string]string)

func ReadFile(hack_file string) []string {
	var instructions []string

	content, err := ioutil.ReadFile(hack_file)
	Check(err)

	text := string(content)
	instructions_with_comments := strings.Split(text, "\n")

	for _, instruction := range instructions_with_comments {
		if strings.HasPrefix(instruction, "//") { // ignore line with only comments
			readComment()
			continue
		} else if len(instruction) == 0 { // ignore empty lines
			continue
		}

		inst := strings.Split(instruction, "//")[0] // ignore comments in a instruction line
		inst = strings.ReplaceAll(inst, " ", "")    // ignore whitespaces
		instructions = append(instructions, inst)

	}

	return instructions
}

func Parse(instructions []string) {
	MakeLTable(instructions)

}

func MakeLTable(instructions []string) {
	x := 0

	for _, i := range instructions {
		x++
		if IsLabel(i) {
			fmt.Println("LABELLLLLLLLLLLL", i)
			x--
			fmt.Println("Label:", i[1:len(i)-1], "Binary:", ToBinary(x))
			ltable[i[1:len(i)]] = ToBinary(x)
		}
	}
}

func MakeATable(instructions []string) {
	//start := 16

	for _, i := range instructions {
		if IsAInstruction(i) {
			val := i[1:]
			_ = val
		}
	}
}

func readComment() {

}

func Check(e error) {
	if e != nil {
		log.Fatal(e)
	}
}
