package main

import (
	"fmt"
	"strings"
)

func main() {
	var n, a, b int
	var users string
	fmt.Scan(&n, &a, &b, &users)
	totalPass := a + b
	var passedNum int
	curForeignStudentRank := 1
	for _, u := range strings.Split(users, "") {
		if u == "a" {
			if passedNum < totalPass {
				fmt.Println("Yes")
				passedNum++
				continue
			}
		}
		if u == "b" {
			if passedNum < totalPass && curForeignStudentRank <= b {
				fmt.Println("Yes")
				passedNum++
				curForeignStudentRank++
				continue
			}
		}
		fmt.Println("No")
	}
}
