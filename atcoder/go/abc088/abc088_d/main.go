package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	io, flush := NewIO()
	defer flush()

	h, w := io.ScanInt2()

	maze := io.Scan2DGraph(h)

	start := &Coordinate{X: 0, Y: 0}
	goal := &Coordinate{X: w-1, Y: h-1}
	cost := dfs(maze, start, goal)
	var whiteCount int
	for _, rows := range maze {
		for _, cell := range rows {
			if cell == "." {
				whiteCount++
			}
		}
	}
	if cost == -1 {
		fmt.Println(-1)
		return
	}
	fmt.Println(whiteCount-1-cost)
}

func dfs(maze [][]string, start, goal *Coordinate) int {
	queue := CoordinateQueue{start}
	var reached [][]bool
	for i := 0; i < len(maze); i++ {
		reached = append(reached, make([]bool, len(maze[0])))
	}

	for len(queue) > 0 {
		cur := queue.Dequeue()
		y := cur.Y
		x := cur.X
		if y < 0 || x < 0 || y > len(maze)-1 || x > len(maze[0])-1 || maze[y][x] == "#" || reached[y][x] {
			continue
		}
		if y == goal.Y && x == goal.X {
			return cur.Distance
		}

		reached[y][x] = true
		nextDistance := cur.Distance + 1
		queue.Enqueue(&Coordinate{Y: y + 1, X: x, Distance: nextDistance})
		queue.Enqueue(&Coordinate{Y: y, X: x + 1, Distance: nextDistance})
		queue.Enqueue(&Coordinate{Y: y - 1, X: x, Distance: nextDistance})
		queue.Enqueue(&Coordinate{Y: y, X: x - 1, Distance: nextDistance})
	}
	return -1
}

type Coordinate struct {
	X        int
	Y        int
	Distance int
}

type CoordinateQueue []*Coordinate

func (queue *CoordinateQueue) Enqueue(c *Coordinate) {
	*queue = append(*queue, c)
}

func (queue *CoordinateQueue) Dequeue() *Coordinate {
	result := (*queue)[0]
	*queue = (*queue)[1:]
	return result
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

func (io *IO) ScanInt2() (int, int) {
	return int(io.ScanInt64()), int(io.ScanInt64())
}

func (io *IO) ScanInt3() (int, int, int) {
	return int(io.ScanInt64()), int(io.ScanInt64()), int(io.ScanInt64())
}

func (io *IO) ScanInt4() (int, int, int, int) {
	return int(io.ScanInt64()), int(io.ScanInt64()), int(io.ScanInt64()), int(io.ScanInt64())
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
