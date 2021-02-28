package main

import (
	"bufio"
	"fmt"
	"github.com/k-yomo/atcoder/go/util"
	"math"
	"os"
	"strconv"
	"strings"
)

type balloon struct {
	initialHeight    int
	riseHeightPerSec int
}

func main() {
	io, flush := NewIO()
	defer flush()

	n := io.ScanInt()
	balloons := make([]*balloon, n)
	for i := 0; i < n; i++ {
		balloons[i] = &balloon{
			initialHeight:    io.ScanInt(),
			riseHeightPerSec: io.ScanInt(),
		}
	}
	ng := balloons[0].initialHeight
	ok := 0
	for i := 0; i < n; i++ {
		balloonMaxHeight := balloons[i].initialHeight + balloons[i].riseHeightPerSec*(n-1)
		if balloonMaxHeight > ok {
			ok = balloonMaxHeight
		}
	}

	ok := util.BinarySearch(ok, ng, func(mid int) bool {
		return 1
	})
	for {
		if AbsDiff(ng, ok) <= 1 {
			break
		}
		mid = (ok + ng) / 2
		if isOK(n, maxSum, mid, balloons) {
			fmt.Println(ng, ok, mid, "ok")
			ok = mid
		} else {
			fmt.Println(ng, ok, mid, "ng")
			ng = mid
		}
	}
	fmt.Println(mid + 1)
}

func BinarySearch(ok, ng int, isOK func(mid int) bool) int {
	for int(math.Abs(float64(ok-ng))) > 1 {
		mid := (ok + ng) / 2
		if isOK(mid) {
			ok = mid
		} else {
			ng = mid
		}
	}

	return ok
}

func isOK(n, maxSum, score int, balloons []*balloon) bool {
	var total int
	for i := 0; i < n; i++ {
		var count int
		for j := 0; j < n; j++ {
			count++
			curHeight := balloons[i].initialHeight + balloons[i].riseHeightPerSec*j
			if curHeight > score {
				break
			}
		}
		total += n - count
		restMaxSum := (n - i - 1) * (n - i - 2) / 2
		if maxSum-total < restMaxSum {
			fmt.Println("トータル", total, "max",maxSum, "残り", restMaxSum)
			return false
		}
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
