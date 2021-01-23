package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
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
	MakeATable(instructions)

}

func MakeLTable(instructions []string) {
	x := 0

	for _, i := range instructions {
		x++
		if IsLabel(i) {
			x--
			fmt.Println(i[1:len(i)-1], ToBinary(x))
			ltable[i[1:len(i)]] = ToBinary(x)
		}
	}
}

func MakeATable(instructions []string) {
	start := 16

	for _, i := range instructions {
		if IsAInstruction(i) {
			val := i[1:]
			if _, err := TryParseInt(val, 10); err == nil {
				val_int, err := strconv.Atoi(val)
				Check(err)
				atable[i] = ToBinary(val_int)
			} else if _, ok := predef_table[val]; ok {
				atable[i] = ToBinary(predef_table[val])
			} else if _, ok := ltable[val]; ok {
				atable[i] = ltable[val]
			} else if _, ok := atable[i]; ok {
				continue
			} else {
				atable[i] = ToBinary(start)
				start++
			}
		}
	}
}

func MakeCTable(instructions []string) {
	for _, i := range instructions {
		if IsCInstruction(i) {
			semi := strings.IndexByte(i, ';')
			equa := strings.IndexByte(i, '=')

			if equa != -1 && semi != -1 { // dest = comp; jump
				dest := i[:equa]
				comp := i[equa+1 : semi]
				jump := i[semi+1:]
				abit := SetABit(comp)
				ctable[i] = first + abit + comp_table[comp] + dest_table[dest] + jump_table[jump]

			}
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
