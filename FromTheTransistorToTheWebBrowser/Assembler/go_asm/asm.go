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

func (s *Scanner) Scan() (tok Token, lit string) {
	s.skipWhiteSpace()
	ch := s.ch
	s.next()

	switch ch {
	case -1:
		tok = EOF
	case '/':
		if s.ch == '/' {
			tok = COMMENT
			lit = s.scanComment()
		}
	case '(':
		tok = LABEL
		lit = s.scanLabel()
	case '@':
		tok = A_INSTRUCTION
		lit = s.scanLine()
	default:
		tok = ILLEGAL
	}

	return
}

func (s *Scanner) scanLine() string {
	offs := s.offset

	for s.ch != '\n' && s.ch != '\r' && s.ch >= 0 && s.ch != ' ' {
		s.next()
	}

	return string(s.src[offs:s.offset])
}

func (s *Scanner) scanLabel() string {
	offs := s.offset

	for {
		ch := s.ch
		if ch == '\n' || ch == '\r' || ch < 0 {
			break
		}
		s.next()
		if ch == ')' {
			break
		}
	}
	return string(s.src[offs : s.offset-1])
}

func (s *Scanner) skipWhiteSpace() {
	for s.ch == ' ' || s.ch == '\t' || s.ch == '\n' || s.ch == '\r' {
		s.next()
	}
}

func (s *Scanner) scanComment() string {
	s.next()
	offs := s.offset
	for s.ch != '\n' && s.ch >= 0 {
		s.next()
	}
	return string(s.src[offs:s.offset])
}

func isCInstruction(ch rune) bool {
	return ch == '0' || ch == '1' || ch == '-' || ch == '!' || ch == 'A' || ch == 'D' || ch == 'M'
}

func Check(e error) {
	if e != nil {
		panic(e)
	}
}
