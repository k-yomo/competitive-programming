package main

import (
	"fmt"
	"strings"
)

func main() {
	var s,t string
	fmt.Scan(&s, &t)

	if s == "Y" {
		fmt.Println(strings.ToUpper(t));
	} else {
		fmt.Println(t);
	}
}