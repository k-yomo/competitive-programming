package main

import (
	"fmt"
	"strings"
)

func main() {
	var str string
	fmt.Scan(&str)
	strArr := strings.Split(str, "")
	var joiCount, ioiCount int
	for i := 0; i < len(str)-2; i++ {
		s := strArr[i] + strArr[i+1] + strArr[i+2]
		if s == "JOI" {
			joiCount++
		}
		if s == "IOI" {
			ioiCount++
		}
	}
	fmt.Println(joiCount)
	fmt.Println(ioiCount)
}
