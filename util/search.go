package util

import "math"

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
