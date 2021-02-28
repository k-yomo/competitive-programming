package main

import (
	"fmt"
)

func main() {
	var n, k int
	var s string
	fmt.Scan(&n, &k)
	fmt.Scanf("%s", &s)
	lastChar := s[0]
	count := 0
	dups := make([]int, 0)
	for i := 1; i < n; i++ {
		if s[i] == lastChar {
			count++
		} else {
			dups = append(dups, count+1)
			count = 0
			lastChar = s[i]
		}
	}
	dups = append(dups, count+1)
	isEdge := false
	if s[0] != s[1] && s[len(s)-1] != s[len(s)-2] {
		isEdge = true
	}

	happyPersonNum := 0
	c := 1
	if len(dups)-1 <= k {
		happyPersonNum = n - 1
	} else {
		for i := len(dups) - 1; i >= 0; i-- {
			if c == k {
				if isEdge && i < 2 {
					happyPersonNum += dups[i]
				} else {
					happyPersonNum += dups[i] + 1
				}
			} else if c <= k {
				happyPersonNum += dups[i] + 1
			} else {
				if dups[i]-1 > 0 {
					happyPersonNum += dups[i] - 1
				}
			}
			c++
		}
	}
	fmt.Println(happyPersonNum)
}
