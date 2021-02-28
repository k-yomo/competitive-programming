package main

import "fmt"

func main() {
	var a, b, c string
	fmt.Scan(&a, &b, &c)
	fmt.Println(fmt.Sprintf("%s%s%s", string(a[0]), string(b[1]), string(c[2])))
}
