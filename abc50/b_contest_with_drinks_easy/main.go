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

func Scan2DInts(s *bufio.Scanner, x, y int) [][]int {
	ints := make([][]int, y)
	for i := 0; i < y; i++ {
		ints[i] = ScanInts(s, x)
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

// https://atcoder.jp/contests/abc050/tasks/abc050_b
func main() {
	s := NewScanner()
	w := NewWriter()
	defer w.Flush()

	n := ScanInt(s)
	times := ScanInts(s, n)
	m := ScanInt(s)
	drinks := Scan2DInts(s, 2, m)

	for _, d := range drinks {
		var total int
		for i, t := range times {
			if i+1 == d[0] {
				total += d[1]
				continue
			}
			total += t
		}
		fmt.Fprintln(w, total)
	}
}
