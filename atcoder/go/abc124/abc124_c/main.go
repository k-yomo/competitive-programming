package main

import (
	"fmt"
	"strconv"
	"strings"
)

// https://atcoder.jp/contests/abc124/tasks/abc124_c
func main() {
	var str string
	fmt.Scan(&str)
	tiles := strings.Split(str, "")

	var oddSum, evenSum int
	for i, tile := range tiles {
		if strconv.Itoa(i%2) == tile {
			evenSum++
		} else {
			oddSum++
		}
	}

	if oddSum < evenSum {
		fmt.Println(oddSum)
	} else {
		fmt.Println(evenSum)
	}
}
