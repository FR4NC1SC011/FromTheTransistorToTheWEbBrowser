package main

type Token int

const (
	ILLEGAL Token = iota
	EOF
	COMMENT
	LABEL
	A_INSTRUCTION
	C_INSTRUCTION
)

type Scanner struct {
	src      []byte
	ch       rune
	offset   int
	rdOffset int
}

const bom = 0xFEFF

func (s *Scanner) Init(src []byte) {
	s.src = src
	s.ch = ' '
	s.offset = 0
	s.rdOffset = 0

	s.next()
	if s.ch == bom {
		s.next()
	}
}

func (s *Scanner) Scan() (tok Token, lit string) {
	return
}

func (s *Scanner) next() {
	if s.rdOffset < len(s.src) {
		s.offset = s.rdOffset
		s.ch = rune(s.src[s.rdOffset])
		s.rdOffset += 1
	} else {
		s.offset = len(s.src)
		s.ch = -1
	}
}

func (s *Scanner) skipWhiteSpace() {
	for s.ch == ' ' || s.ch == '\t' || s.ch == '\n' || s.ch == '\r' {
		s.next()
	}
}

func Check(e error) {
	if e != nil {
		panic(e)
	}
}
