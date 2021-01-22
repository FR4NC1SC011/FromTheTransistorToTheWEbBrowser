package main

import (
	"strconv"
	"strings"
)

func IsLabel(instruction string) bool {
	if strings.HasPrefix(instruction, "(") {
		if strings.HasSuffix(instruction, ")") {
			return true
		}
	}
	return false
}

func IsAInstruction(instruction string) bool {
	if strings.HasPrefix(instruction, "@") {
		return true
	}
	return false
}

func ToBinary(x int) string {
	n := int64(x)
	num := strconv.FormatInt(n, 2)
	length := len(num)
	diff := 16 - length
	rest := ""
	for x := 0; x < diff; x++ {
		rest += "0"
	}

	binary := rest + num

	return binary
}
