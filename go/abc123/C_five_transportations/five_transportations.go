package main

import (
	"fmt"
	"math"
	"sort"
)

func main() {
	var n int
	fmt.Scan(&n)
	carrieableNums := ScanNums(5)
	sort.Ints(carrieableNums)
	fmt.Println(int(math.Ceil(float64(n)/float64(carrieableNums[0]))) + 4)
}

// brute force solution
func main2() {
	var n int
	fmt.Scan(&n)
	carrieableNums := ScanNums(5)

	totalMins := 0
	peopleNumInCities := [6]int{n}

	for peopleNumInCities[5] != n {
		totalMins++
		for i := len(peopleNumInCities) - 2; i >= 0; i-- {
			if peopleNumInCities[i] > carrieableNums[i] {
				peopleNumInCities[i+1] += carrieableNums[i]
				peopleNumInCities[i] -= carrieableNums[i]
			} else {
				peopleNumInCities[i+1] += peopleNumInCities[i]
				peopleNumInCities[i] = 0
			}
		}
	}
	fmt.Println(totalMins)
}

func ScanNums(len int) (nums []int) {
	var num int
	for i := 0; i < len; i++ {
		fmt.Scan(&num)
		nums = append(nums, num)
	}
	return
}
