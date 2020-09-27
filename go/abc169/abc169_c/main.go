package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	var a int
	var b string
	fmt.Scan(&a, &b)
	bInt, _ := strconv.Atoi(strings.Replace(b, ".", "", -1))
	fmt.Println(a * bInt  / 100)
}

