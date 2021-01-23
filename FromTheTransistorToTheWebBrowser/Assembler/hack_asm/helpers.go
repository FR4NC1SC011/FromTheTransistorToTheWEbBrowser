package main

import (
	"strconv"
	"strings"
)

func TryParseInt(s string, base int) (int64, error) {
	i, err := strconv.ParseInt(s, base, 64)
	if err != nil {
		return 0, err
	}
	return i, nil
}

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

func IsCInstruction(instruction string) bool {
	if strings.HasPrefix(instruction, "@") && strings.HasPrefix(instruction, "(") {
		return true
	}
	return false
}

func SetABit(comp string) string {
	if ok := strings.IndexByte(comp, 'M'); ok != -1 {
		return "1"
	} else {
		return "0"
	}

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
