package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {
	io, flush := NewIO()
	defer flush()
	n, m := io.ScanInt(), io.ScanInt()
	shuttles := io.Scan2DInts(m, 3)
	graph := NewGraph()
	countMap := map[int]int{}
	for _, s := range shuttles {
		countMap[s[0]]++
		countMap[s[1]]++
	}
	var maxCount int
	for _, c := range countMap {
		if c > maxCount {
			maxCount = c
		}
	}
	for _, s := range shuttles {
		graph.AddEdge(s[0], s[1], s[2])
	}
	longestPathMap := map[int]int{}
	for i := 1; i < n; i++ {
		for j := i + 1; j <= n; j++ {
			if countMap[i] != maxCount && countMap[j] != maxCount {
				continue
			}
			distance := graph.GetPath(i, j)
			if distance > longestPathMap[i] {
				longestPathMap[i] = distance
			}
			if distance > longestPathMap[j] {
				longestPathMap[j] = distance
			}
		}
	}
	min := math.MaxInt64 / 2
	for _, p := range longestPathMap {
		if p < min {
			min = p
		}
	}
	fmt.Println(min)
}

type Graph struct {
	nodes map[int][]edge
}

type edge struct {
	node   int
	weight int
}

func NewGraph() *Graph {
	return &Graph{nodes: make(map[int][]edge)}
}

func (g *Graph) AddEdge(origin, destiny int, weight int) {
	g.nodes[origin] = append(g.nodes[origin], edge{node: destiny, weight: weight})
	g.nodes[destiny] = append(g.nodes[destiny], edge{node: origin, weight: weight})
}

func (g *Graph) GetEdges(node int) []edge {
	return g.nodes[node]
}

func (g *Graph) GetPath(origin, destiny int) int {
	h := newHeap()
	h.push(path{value: 0, nodes: []int{origin}})
	visited := make(map[int]bool)

	for len(*h.values) > 0 {
		p := h.pop()
		node := p.nodes[len(p.nodes)-1]

		if visited[node] {
			continue
		}

		if node == destiny {
			return p.value
		}

		for _, e := range g.GetEdges(node) {
			if !visited[e.node] {
				h.push(path{value: p.value + e.weight, nodes: append([]int{}, append(p.nodes, e.node)...)})
			}
		}

		visited[node] = true
	}

	return 0
}

type path struct {
	value int
	nodes []int
}

type minPath []path

func (h minPath) Len() int           { return len(h) }
func (h minPath) Less(i, j int) bool { return h[i].value < h[j].value }
func (h minPath) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *minPath) Push(x interface{}) {
	*h = append(*h, x.(path))
}

func (h *minPath) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

type Heap struct {
	values *minPath
}

func newHeap() *Heap {
	return &Heap{values: &minPath{}}
}

func (h *Heap) push(p path) {
	heap.Push(h.values, p)
}

func (h *Heap) pop() path {
	i := heap.Pop(h.values)
	return i.(path)
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

func (io *IO) Println(s string) {
	fmt.Fprintln(io.writer, s)
}
