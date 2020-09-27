package util

import (
	"fmt"
	"strings"
)

func PrepareEmptyBoolArray(n int) []bool {
	arr := make([]bool, n)
	for i := 0; i < n; i++ {
		arr[i] = false
	}
	return arr
}

func PrepareEmpty2DBoolArray(y, x int) [][]bool {
	arr := make([][]bool, y, x)
	for i := 0; i < y; i++ {
		arr[i] = PrepareEmptyBoolArray(x)
	}
	return arr
}

func PrepareEmptyIntArray(n int) []int {
	arr := make([]int, n)
	for i := 0; i < n; i++ {
		arr[i] = 0
	}
	return arr
}

func PrepareEmpty2DintArray(x, y int) [][]int {
	arr := make([][]int, y)
	for i := 0; i < y; i++ {
		arr[i] = PrepareEmptyIntArray(x)
	}
	return arr
}

func ReverseStrings(s []string) {
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		s[i], s[j] = s[j], s[i]
	}
}

func ReverseBytes(s []byte) {
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		s[i], s[j] = s[j], s[i]
	}
}

func JoinInts(ints []int) string {
	return strings.Trim(strings.Join(strings.Fields(fmt.Sprint(ints)), ""), "[]")
}

type IntQueue []int

func (queue *IntQueue) Enqueue(i int) {
	*queue = append(*queue, i)
}

func (queue *IntQueue) Dequeue() int {
	result := (*queue)[0]
	*queue = (*queue)[1:]
	return result
}

