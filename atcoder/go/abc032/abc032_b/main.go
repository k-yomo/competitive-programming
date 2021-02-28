package main

import "fmt"

func main() {
	var s string
	var k int
	fmt.Scan(&s, &k)

	possiblePasswordMap  := map[string]bool{}
	// 10
	// 0-4, 1-5, 2-6,3-7, 4-8, 5-9
	for i := 0; i <= len(s)-k; i++ {
		possiblePasswordMap[s[i:i+k]] = true
	}
	fmt.Println(len(possiblePasswordMap))
}