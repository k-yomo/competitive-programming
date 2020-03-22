package main

import (
	"bytes"
	"fmt"
)

func main() {
	var s string
	fmt.Scan(&s)

	reversedStr := append([]byte{}, []byte(s)...)
	ReverseBytes(reversedStr)
	substr := s[(len(s)+3)/2-1 : len(s)]
	reversedSubstr := append([]byte{}, []byte(substr)...)
	ReverseBytes(reversedSubstr)
	if bytes.Equal([]byte(s), reversedStr) && bytes.Equal([]byte(substr), reversedSubstr) {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}

func ReverseBytes(s []byte) {
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		s[i], s[j] = s[j], s[i]
	}
}
