package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
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

func ScanInt(s *bufio.Scanner) int {
	return int(ScanInt64(s))
}

func ScanInt64(s *bufio.Scanner) int64 {
	i, err := strconv.ParseInt(ScanString(s), 10, 64)
	if err != nil {
		panic(err)
	}
	return i
}

// https://atcoder.jp/contests/abc054/tasks/abc054_a
func main() {
	s := NewScanner()
	w := NewWriter()
	defer w.Flush()

	aliceHand, bobHand := ScanInt(s), ScanInt(s)
	if aliceHand == bobHand {
		fmt.Fprintln(w, "Draw")
	} else if aliceHand == 1 {
		fmt.Fprintln(w, "Alice")
	} else if bobHand == 1 {
		fmt.Fprintln(w, "Bob")
	} else if aliceHand > bobHand {
		fmt.Fprintln(w, "Alice")
	} else {
		fmt.Fprintln(w, "Bob")
	}
}
