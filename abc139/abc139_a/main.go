package main

import "fmt"

func main() {
	var s, t []byte
	fmt.Scan(&s, &t)
	var count int
	for i := 0; i < 3; i++ {
		if s[i] == t[i] {
			count++
		}
	}
	fmt.Println(count)
}