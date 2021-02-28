package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

var graph [][]int

func main() {
	io, flush := NewIO()
	defer flush()
	h, w := io.ScanInt(), io.ScanInt()
	maze := io.Scan2DGraph(h)
	graph = PrepareEmpty2DintArray(h*w, h*w)
	for y := 0; y < h; y++ {
		for x := 0; x < w; x++ {
			for _, s := range Surroundings(maze, h, w, y, x) {
				idx := y + x*y
				if graph[idx] == nil {
					graph[idx] = append(graph[idx], make([]int, h*w)...)
				}
				if y+(x+y*w) >= h*w {
					continue
				}
				graph[y+x+y*w][s[0]+s[1]*s[0]*w] = 1
			}
		}
	}
	WarshalFloyd(h * w)
	fmt.Println(graph)
	var max int
	for _, row := range graph {
		for _, cell := range row {
			if cell < 1000 && cell > max {
				max = cell
			}
		}
	}
	io.Println(max)
}

func Surroundings(maze [][]string, h, w, y, x int) [][]int {
	surroundingDiffs := [][]int{{1, 0}, {1, 1}, {0, 1}, {-1, 1}, {- 1, 0}, {-1, -1}, {0, -1}, {1, -1}}
	var surroundings [][]int
	for _, diff := range surroundingDiffs {
		a := y + diff[0]
		b := x + diff[1]
		if a < 0 || b < 0 || a >= h || b >= w {
			continue
		}
		if maze[a][b] != "#" {
			surroundings = append(surroundings, []int{a, b})
		}
	}
	return surroundings
}

func WarshalFloyd(n int) {
	for k := 0; k < n; k++ {
		for i := 0; i < n; i++ {
			for j := 0; j < n; j++ {
				graph[i][j] = int(math.Min(float64(graph[i][j]), float64(graph[i][k]+graph[k][j])))
			}
		}
	}
}

func PrepareEmptyIntArray(n int) []int {
	arr := make([]int, n)
	for i := 0; i < n; i++ {
		arr[i] = 1000
	}
	return arr
}

func PrepareEmpty2DintArray(x, y int) [][]int {
	arr := make([][]int, y)
	for i := 0; i < y; i++ {
		arr[i] = PrepareEmptyIntArray(x)
	}
	return arr
}

func PrepareEmptyBoolArray(n int) []bool {
	arr := make([]bool, n)
	for i := 0; i < n; i++ {
		arr[i] = false
	}
	return arr
}

func PrepareEmpty2DBoolArray(y, x int) [][]bool {
	arr := make([][]bool, y, x)
	for i := 0; i < y; i++ {
		arr[i] = PrepareEmptyBoolArray(x)
	}
	return arr
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
