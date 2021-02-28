package main

import (
	"fmt"
	"strconv"
)

type op bool

func (o op) String() string {
	if o {
		return "+"
	} else {
		return "-"
	}
}

func main() {
	var n string
	fmt.Scan(&n)

	a, _ := strconv.Atoi(string(n[0]))
	b, _ := strconv.Atoi(string(n[1]))
	c, _ := strconv.Atoi(string(n[2]))
	d, _ := strconv.Atoi(string(n[3]))

	options := [][]op{
		{true, false, false},
		{true, true, false},
		{true, true, true},
		{true, false, true},
		{false, true, true},
		{false, false, true},
		{false, true, false},
		{false, false, false},
	}

	for _, op := range options {
		total := a
		if op[0] {
			total += b
		} else {
			total -= b
		}
		if op[1] {
			total += c
		} else {
			total -= c
		}
		if op[2] {
			total += d
		} else {
			total -= d
		}
		if total == 7 {
			fmt.Println(fmt.Sprintf("%d%s%d%s%d%s%d=7", a, op[0], b, op[1], c, op[2], d))
			return
		}
	}
}
