package main

import "fmt"

func main() {
	var k int
	fmt.Scan(&k)

	for i := 0; i < k; i++ {
		fmt.Print("ACL")
	}
	fmt.Println()
}
