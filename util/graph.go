package util

import "math"

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
