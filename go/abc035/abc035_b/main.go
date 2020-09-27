package main

import (
	"fmt"
	"math"
	"sort"
)

func main() {
	var s string
	var t int
	fmt.Scan(&s, &t)
	var missingCount int
	dronePosition := []int{0, 0} // x,y
	for _, command := range s {
		switch string(command) {
		case "L":
			dronePosition[0]--
		case "R":
			dronePosition[0]++
		case "U":
			dronePosition[1]++
		case "D":
			dronePosition[1]--
		case "?":
			missingCount++
		}
	}

	possibleDistances := make([]int, 0, missingCount)
	if t == 1 {
		for _, num := range []int{-1, +1} {
			distance := int(math.Abs(float64(dronePosition[0]+num*missingCount)) + math.Abs(float64(dronePosition[1])))
			possibleDistances = append(possibleDistances, distance)
		}
		for _, num := range []int{-1, +1} {
			distance := int(math.Abs(float64(dronePosition[0])) + math.Abs(float64(dronePosition[1]+num*missingCount)))
			possibleDistances = append(possibleDistances, distance)
		}
		sort.Ints(possibleDistances)
		fmt.Println(possibleDistances[3])
	} else {
		absSum := int(math.Abs(float64(dronePosition[0]))) + int(math.Abs(float64(dronePosition[1])))
		if absSum >= missingCount {
			absSum -= missingCount
			missingCount = 0
		} else {
			missingCount -= absSum
			absSum = 0
		}
		fmt.Println(absSum + missingCount%2)
	}
}
