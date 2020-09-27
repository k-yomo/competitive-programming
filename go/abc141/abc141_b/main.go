package main

import (
	"fmt"
)

func main() {
	var s string
	fmt.Scan(&s)

	for i, move := range s {
		if i%2 == 0 {
			switch string(move) {
			case "R", "U", "D":
				continue
			default:
				fmt.Println("No")
				return
			}
		} else {
			switch string(move) {
			case "L", "U", "D":
				continue
			default:
				fmt.Println("No")
				return
			}
		}
	}
	fmt.Println("Yes")
}
