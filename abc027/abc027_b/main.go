package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)

	islandPopulations := make([]int, n)
	var totalPopulation int
	for i := range islandPopulations {
		var population int
		fmt.Scan(&population)
		islandPopulations[i] = population
		totalPopulation += population
	}

	if totalPopulation%n != 0 {
		fmt.Println(-1)
		return
	}

	for i := 0; i <  {

	}
}
