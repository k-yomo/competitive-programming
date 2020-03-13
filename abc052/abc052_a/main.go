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

func ScanInts(s *bufio.Scanner, n int) []int {
	ints := make([]int, n)
	for i := 0; i < n; i++ {
		ints[i] = ScanInt(s)
	}
	return ints
}

func ScanInt64(s *bufio.Scanner) int64 {
	i, err := strconv.ParseInt(ScanString(s), 10, 64)
	if err != nil {
		panic(err)
	}
	return i
}

// https://atcoder.jp/contests/abc052/tasks/abc052_a
func main() {
	s := NewScanner()
	w := NewWriter()
	defer w.Flush()

	ints := ScanInts(s, 4)
	rec1 := ints[0] * ints[1]
	rec2 := ints[2] * ints[3]
	if rec1 > rec2 {
		fmt.Fprintln(w, rec1)
	} else {
		fmt.Fprintln(w, rec2)
	}
}
