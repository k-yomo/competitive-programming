package util

import (
	"container/heap"
	"math"
)

var reached [][]bool

func Search(maze [][]string, y, x int) {
	if y < 0 || x < 0 || y > len(maze)-1 || x > len(maze[0])-1 || maze[y][x] == "#" || reached[y][x] {
		return
	}
	reached[y][x] = true
	Search(maze, y+1, x)
	Search(maze, y, x+1)
	Search(maze, y-1, x)
	Search(maze, y, x-1)
}

func Surroundings(h, w, y, x int) [][]int {
	surroundingDiffs := [][]int{{1, 0}, {1, 1}, {0, 1}, {-1, 1}, {- 1, 0}, {-1, -1}, {0, -1}, {1, -1}}
	var surroundings [][]int
	for _, diff := range surroundingDiffs {
		a := y + diff[0]
		b := x + diff[1]
		if a < 0 || b < 0 || a >= h || b >= w {
			continue
		}
		surroundings = append(surroundings, []int{a, b})
	}
	return surroundings
}

var graph [][]int

func WarshalFloyd(n int) {
	for k := 0; k < n; k++ {
		for i := 0; i < n; i++ {
			for j := 0; j < n; j++ {
				graph[i][j] = int(math.Min(float64(graph[i][j]), float64(graph[i][k]+graph[k][j])))
			}
		}
	}
}

type Edge struct {
	to, cost int
}

type Elem struct {
	// value: 頂点の番号
	value interface{}
	// cost from start point
	priority int
	index    int
}

type PriorityQueue []*Elem

func (pq PriorityQueue) Len() int {
	return len(pq)
}

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].priority < pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index, pq[j].index = i, j
}

func (pq *PriorityQueue) Push(x interface{}) {
	n := len(*pq)
	elem := x.(*Elem)
	elem.index = n
	*pq = append(*pq, elem)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(*pq)
	elem := old[n-1]
	old[n-1] = nil
	elem.index = -1
	*pq = old[0 : n-1]
	return elem
}

func newSlice(n, x int) []int {
	slice := make([]int, n)
	for i := range slice {
		slice[i] = x
	}
	return slice
}

type Dijkstra struct {
	NodeNum int      // 頂点数
	Origin  int      // 始点
	InfCost int      // 無限大とみなすコストの値
	Pred    []int    // 遷移元
	Cost    []int    // 始点からの最小コスト
	Edges   [][]Edge // 頂点の隣接リスト
}

func NewDijkstra(n int) *Dijkstra {
	edges := make([][]Edge, n)
	for i := range edges {
		edges[i] = make([]Edge, 0)
	}
	inf := 1 << 60
	d := &Dijkstra{
		NodeNum: n,
		InfCost: inf,
		Pred:    newSlice(n, -1),
		Cost:    newSlice(n, inf),
		Edges:   edges,
	}
	return d
}

// fromからtoへの重み付きの辺を追加
func (d *Dijkstra) AddEdge(from, to, cost int) {
	d.Edges[from] = append(d.Edges[from], Edge{to, cost})
}

// 0-indexed
func (d *Dijkstra) Search(origin int) {
	// init
	done := make([]bool, d.NodeNum)
	pq := make(PriorityQueue, 0)
	heap.Init(&pq)
	heap.Push(&pq, &Elem{
		value:    origin,
		priority: 0,
	})

	for pq.Len() > 0 {
		v := heap.Pop(&pq).(*Elem)
		now := v.value.(int)
		done[now] = true
		for _, edge := range d.Edges[now] {
			if done[edge.to] {
				continue
			}
			// 隣接頂点の最小コストを更新
			nextCost := v.priority + edge.cost
			if nextCost < d.Cost[edge.to] {
				d.Cost[edge.to] = nextCost
				d.Pred[edge.to] = now
				heap.Push(&pq, &Elem{
					value:    edge.to,
					priority: nextCost,
				})
			}
		}
	}
}
