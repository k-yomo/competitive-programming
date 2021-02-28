package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var roots [][]bool

func main() {
	io, flush := NewIO()
	h, w := io.ScanInt(), io.ScanInt()
	roots = PrepareEmpty2DBoolArray(h, w)
	img := io.Scan2DGraph(h)

	io.Println("possible")
	for y := 0; y < h; y++ {
		for x := 0; x < w; x++ {
			if img[y][x] == "." {
				io.Print(".")
				continue
			}
			if isRoot(img, y, x) {
				roots[y][x] = true
				io.Print("#")
				continue
			}

			var rootExist bool
			for _, s := range Surroundings(h, w, y, x) {
				if isRoot(img, s[0], s[1]) {
					rootExist = true
					break
				}
			}
			if !rootExist {
				fmt.Println("impossible")
				return
			}
			io.Print(".")
		}
		io.Println("")
	}
	flush()
}

func isRoot(graph [][]string, y, x int) bool {
	if roots[y][x] {
		return true
	}
	for _, s := range Surroundings(len(graph), len(graph[0]), y, x) {
		if isWhite(graph, s[0], s[1]) {
			return false
		}
	}
	return true

}

func Surroundings(h, w, y, x int) [][]int {
	var surroundings [][]int
	for _, diff := range [][]int{{1, 0}, {1, 1}, {0, 1}, {-1, 1}, {- 1, 0}, {-1, -1}, {0, -1}, {1, -1}} {
		a := y + diff[0]
		b := x + diff[1]
		if a < 0 || b < 0 || a >= h || b >= w {
			continue
		}
		surroundings = append(surroundings, []int{a, b})
	}
	return surroundings
}

func isWhite(graph [][]string, y, x int) bool {
	if graph[y][x] == "#" {
		return false
	}
	return true
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
	s.Buffer(make([]byte, 1000005), 1000005)
	s.Split(bufio.ScanWords)
	return s
}

func newWriter() *bufio.Writer {
	return bufio.NewWriter(os.Stdout)
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

func (io *IO) Scan2DStrings(x, y int) [][]string {
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

func (io *IO) Scan2DInts(x, y int) [][]int {
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

func (io *IO) Print(s string) {
	fmt.Fprint(io.writer, s)
}

func (io *IO) Println(s string) {
	fmt.Fprintln(io.writer, s)
}

func PrepareEmptyBoolArray(n int) []bool {
	arr := make([]bool, n)
	for i := 0; i < n; i++ {
		arr[i] = false
	}
	return arr
}

func PrepareEmpty2DBoolArray(y, x int) [][]bool {
	arr := make([][]bool, y)
	for i := 0; i < y; i++ {
		arr[i] = PrepareEmptyBoolArray(x)
	}
	return arr
}
