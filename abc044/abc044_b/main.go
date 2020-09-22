package main

import "fmt"

func main() {
	var w string
	fmt.Scan(&w)
	charCountMap := map[int32]int{}
	for _, char := range w {
		charCountMap[char]++
	}
	for _, count := range charCountMap {
		if count%2 != 0 {
			fmt.Println("No")
			return
		}
	}
	fmt.Println("Yes")
}
