package main

import "fmt"

func main() {
	var a, b, c, d float64
	fmt.Scan(&a, &b, &c, &d)

	takahashiWonRate := b / a
	aokiTeamWonRate := d / c
	if takahashiWonRate == aokiTeamWonRate {
		fmt.Println("DRAW")
	} else if takahashiWonRate > aokiTeamWonRate {
		fmt.Println("TAKAHASHI")
	} else {
		fmt.Println("AOKI")
	}
}
