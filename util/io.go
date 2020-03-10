package util

import (
	"bufio"
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

func ScanStrings(s *bufio.Scanner, n int) []string {
	strs := make([]string, n)
	for i := 0; i < n; i++ {
		strs[i] = ScanString(s)
	}
	return strs
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

func ScanFloat64(s *bufio.Scanner) float64 {
	i, _ := strconv.ParseFloat(ScanString(s), 64)
	return i
}

func ScanFloat64s(s *bufio.Scanner, n int) []float64 {
	floats := make([]float64, n)
	for i := 0; i < n; i++ {
		floats[i] = ScanFloat64(s)
	}
	return floats
}
