package main

import "fmt"

func main() {
	var s string
	fmt.Scan(&s)

	var count int
	var curR rune
	for _, r := range s {
		if curR == r {
			count++
			continue
		}

		if count > 0 {
			fmt.Print(count)
		}
		fmt.Print(string(r))
		curR = r
		count = 1
	}
	fmt.Printf("%d\n", count)
}
