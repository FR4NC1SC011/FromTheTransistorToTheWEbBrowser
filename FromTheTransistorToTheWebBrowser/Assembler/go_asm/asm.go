package main

import (
	"bufio"
	"fmt"
	"os"
)

func Asm(asm_file string) {
	asm, err := os.Open(asm_file)
	Check(err)

	sc := bufio.NewScanner(asm)

	for sc.Scan() {
		fmt.Println(sc.Text())
	}

}

func Check(e error) {
	if e != nil {
		panic(e)
	}
}
