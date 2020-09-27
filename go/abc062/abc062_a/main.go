package main

import "fmt"

func ContainsInt(arr []int, i int) bool {
	for _, a := range arr {
		if a == i {
			return true
		}
	}
	return false
}

func main() {
	var x, y int
	fmt.Scan(&x, &y)
	g1 := []int{1, 3, 5, 7, 8, 10, 12}
	g2 := []int{4, 6, 9, 11}
	g3 := []int{2}
	if (ContainsInt(g1, x) && ContainsInt(g1, y)) || (ContainsInt(g2, x) && ContainsInt(g2, y)) || (ContainsInt(g3, x) && ContainsInt(g3, y)) {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
