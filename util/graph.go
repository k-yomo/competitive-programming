package util

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

