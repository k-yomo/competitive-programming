package main

import (
	"fmt"
	"strings"
)

// 1st approach which results in TLE..
func main2() {
	var n, q int
	fmt.Scan(&n, &q)
	nodes := make([]int, n)
	nodeChildren := map[int][]int{1: {}}
	nodeMap := map[int]int{}
	for i := 0; i < n-1; i++ {
		var parent, child int
		fmt.Scan(&parent, &child)
		parentNum := parent
		nodeMap[child] = parent
		for parentNum != 0 {
			nodeChildren[parentNum] = append(nodeChildren[parentNum], child)
			parentNum = nodeMap[parentNum]
		}
	}
	actions := ScanTwoNums(q)
	for node, count := range actions {
		nodes[node-1] += count
		children := nodeChildren[node]
		for _, c := range children {
			nodes[c-1] += count
		}
	}

	fmt.Println(strings.Trim(fmt.Sprint(nodes), "[]"))
}

func ScanTwoNums(len int) map[int]int {
	var node, count int
	numMap := map[int]int{}
	for i := 0; i < len; i++ {
		fmt.Scan(&node, &count)
		numMap[node] += count
	}
	return numMap
}
