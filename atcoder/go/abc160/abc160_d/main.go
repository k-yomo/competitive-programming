package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	var n, x, y int
	fmt.Scan(&n, &x, &y)
	distanceMap := map[int]int{}
	for i := 1; i < n; i++ {
		for j := i + 1; j <= n; j++ {
			orig := float64(j - i)
			viaX := math.Abs(float64(x-i)) + 1 + math.Abs(float64(y-j))
			distanceMap[int(math.Min(orig, viaX))]++
		}
	}
	io, flush := NewIO()
	defer flush()
	for i := 1; i < n; i++ {
		io.Println(distanceMap[i])
	}
}

func CombCount(left int, right int) int {
	rightFac := Factorial(right, right)
	if rightFac <= 0 {
		return 0
	}
	return Factorial(left, right) / rightFac
}

// Factorial returns factorial (n P times)
func Factorial(n int, times int) (res int) {
	facts := make([]int, n+1)
	if facts[n] != 0 {
		res = facts[n]
		return res
	}

	if n > 0 && times > 0 {
		res = n * Factorial(n-1, times-1)
		return res
	}

	return 1
}

type IO struct {
	scanner *bufio.Scanner
	writer  *bufio.Writer
}

func NewIO() (*IO, func()) {
	io := &IO{
		scanner: newScanner(),
		writer:  newWriter(),
	}
	return io, func() { io.writer.Flush() }
}

func newScanner() *bufio.Scanner {
	s := bufio.NewScanner(os.Stdin)
	s.Buffer(make([]byte, 10000000), 10000000)
	s.Split(bufio.ScanWords)
	return s
}

func newWriter() *bufio.Writer {
	return bufio.NewWriter(os.Stdout)
}

func (io *IO) ScanBytes() []byte {
	if !io.scanner.Scan() {
		panic("scan string failed")
	}
	return io.scanner.Bytes()
}

func (io *IO) ScanString() string {
	if !io.scanner.Scan() {
		panic("scan string failed")
	}
	return io.scanner.Text()
}

func (io *IO) ScanStrings(n int) []string {
	strs := make([]string, n)
	for i := 0; i < n; i++ {
		strs[i] = io.ScanString()
	}
	return strs
}

func (io *IO) Scan2DStrings(y, x int) [][]string {
	strings := make([][]string, y)
	for i := 0; i < y; i++ {
		strings[i] = io.ScanStrings(x)
	}
	return strings
}

func (io *IO) Scan2DGraph(y int) [][]string {
	strs := make([][]string, y)
	for i := 0; i < y; i++ {
		strs[i] = strings.Split(io.ScanString(), "")
	}
	return strs
}

func (io *IO) ScanInt() int {
	return int(io.ScanInt64())
}

func (io *IO) ScanInts(n int) []int {
	ints := make([]int, n)
	for i := 0; i < n; i++ {
		ints[i] = io.ScanInt()
	}
	return ints
}

func (io *IO) Scan2DInts(y, x int) [][]int {
	ints := make([][]int, y)
	for i := 0; i < y; i++ {
		ints[i] = io.ScanInts(x)
	}
	return ints
}

func (io *IO) ScanInt64() int64 {
	i, err := strconv.ParseInt(io.ScanString(), 10, 64)
	if err != nil {
		panic(err)
	}
	return i
}

func (io *IO) ScanFloat64() float64 {
	i, _ := strconv.ParseFloat(io.ScanString(), 64)
	return i
}

func (io *IO) ScanFloat64s(n int) []float64 {
	floats := make([]float64, n)
	for i := 0; i < n; i++ {
		floats[i] = io.ScanFloat64()
	}
	return floats
}

func (io *IO) Println(a ...interface{}) {
	fmt.Fprintln(io.writer, a...)
}
