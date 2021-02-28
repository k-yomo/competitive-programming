package main

import "fmt"

func main() {
	var x string
	fmt.Scan(&x)

	for len(x) > 0 {
		switch {
		case len(x) >= 2 && x[len(x)-2:] == "ch":
			x = x[:len(x)-2]
		case x[len(x)-1] == 'o' || x[len(x)-1] == 'k' || x[len(x)-1] == 'u':
			x = x[:len(x)-1]
		default:
			fmt.Println("NO")
			return
		}
	}
	fmt.Println("YES")
}
