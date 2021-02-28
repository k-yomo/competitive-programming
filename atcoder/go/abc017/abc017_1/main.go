package main

import "fmt"

func main() {
	var s1, e1, s2, e2, s3, e3 int
	fmt.Scan(&s1, &e1, &s2, &e2, &s3, &e3)

	fmt.Println(s1*e1/10 + s2*e2/10 + s3*e3/10)
}
