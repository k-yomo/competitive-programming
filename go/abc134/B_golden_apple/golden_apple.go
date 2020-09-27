package main

import "fmt"

func main()  {
	var n, d int
	fmt.Scan(&n, &d)
	l := d * 2 + 1
	if n % l == 0 {
		fmt.Println(n / l)
	} else {
		fmt.Println(n / l + 1)
	}
}
