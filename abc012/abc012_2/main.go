package main

import (
	"fmt"
	"time"
)

func main() {
	var n time.Duration
	fmt.Scan(&n)
	n = n * time.Second

	h := n / time.Hour
	n -= h * time.Hour
	m := n / time.Minute
	n -= m * time.Minute
	s := n / time.Second

	fmt.Printf("%02d:%02d:%02d\n", h, m, s)
}
