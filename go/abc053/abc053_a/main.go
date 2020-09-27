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

// https://atcoder.jp/contests/abc053/tasks/abc053_a
func main() {
	s := NewScanner()
	w := NewWriter()
	defer w.Flush()

	if ScanInt(s) < 1200 {
		fmt.Fprintln(w, "ABC")
	} else {
		fmt.Fprintln(w, "ARC")
	}
}

