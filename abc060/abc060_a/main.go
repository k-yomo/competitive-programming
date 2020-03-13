package main

import "fmt"

func main() {
	var a, b, c string
	fmt.Scan(&a, &b, &c)
	if string(a[len(a)-1]) == string(b[0]) && string(b[len(b)-1]) == string(c[0]) {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}
