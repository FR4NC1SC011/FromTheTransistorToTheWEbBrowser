package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"os"
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
	MakeCTable(instructions)
	results := Translate(instructions)
	WriteFile(results)

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
	first := "111"

	var dest, comp, jump, abit string

	for _, i := range instructions {
		if IsCInstruction(i) {
			semi := strings.IndexByte(i, ';')
			equa := strings.IndexByte(i, '=')

			if equa != -1 && semi != -1 { // dest = comp; jump
				dest = i[:equa]
				comp = i[equa+1 : semi]
				jump = i[semi+1:]
				abit = SetABit(comp)
				ctable[i] = first + abit + comp_table[comp] + dest_table[dest] + jump_table[jump]

			} else if equa == -1 && semi != -1 { // comp; jump
				comp = i[:semi]
				jump = i[semi+1:]
				abit = SetABit(comp)
				ctable[i] = first + abit + comp_table[comp] + dest_table["null"] + jump_table[jump]

			} else if equa != -1 && semi == -1 {
				dest = i[:equa]
				comp = i[equa+1:]
				abit = SetABit(comp)
				ctable[i] = first + abit + comp_table[comp] + dest_table[dest] + jump_table["null"]
			}
		}
	}
}

func Translate(instructions []string) []string {
	f := make([]string, 0)
	for _, i := range instructions {
		if IsAInstruction(i) {
			f = append(f, atable[i])
		} else if IsCInstruction(i) {
			f = append(f, atable[i])
		}
	}
	return f
}

func WriteFile(instructions []string) {
	f, err := os.Create("ouput.hack")
	Check(err)
	defer f.Close()

	for _, instruction := range instructions {
		_, err := f.WriteString(instruction + "\n")
		Check(err)
	}
	fmt.Println("Done")
}

func readComment() {

}

func Check(e error) {
	if e != nil {
		log.Fatal(e)
	}
}
