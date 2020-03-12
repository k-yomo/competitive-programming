package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func NewScanner() *bufio.Scanner {
	s := bufio.NewScanner(os.Stdin)
	s.Buffer(make([]byte, 1000005), 1000005)
	s.Split(bufio.ScanWords)
	return s
}

func NewWriter() *bufio.Writer {
	return bufio.NewWriter(os.Stdout)
}

func ScanString(s *bufio.Scanner) string {
	if !s.Scan() {
		panic("scan string failed")
	}
	return s.Text()
}

// https://atcoder.jp/contests/abc051/tasks/abc051_a
func main() {
	s := NewScanner()
	w := NewWriter()
	defer w.Flush()
	fmt.Fprintln(w, strings.Replace(ScanString(s), ",", " ", -1))
}
