package main

import "fmt"

func main() {
	var n int
	var s string
	fmt.Scan(&n, &s)

	if n < 2 {
		fmt.Println(0)
		return
	}

	ACounts := make([]int, len(s)+1)
	TCounts := make([]int, len(s)+1)
	GCounts := make([]int, len(s)+1)
	CCounts := make([]int, len(s)+1)
	for i := 0; i < n; i++ {
		switch s[i] {
		case 'A':
			ACounts[i+1] = 1 + ACounts[i]
			TCounts[i+1] = TCounts[i]
			GCounts[i+1] = GCounts[i]
			CCounts[i+1] = CCounts[i]
		case 'T':
			TCounts[i+1] = 1 + TCounts[i]
			ACounts[i+1] = ACounts[i]
			GCounts[i+1] = GCounts[i]
			CCounts[i+1] = CCounts[i]
		case 'G':
			GCounts[i+1] = 1 + GCounts[i]
			ACounts[i+1] = ACounts[i]
			TCounts[i+1] = TCounts[i]
			CCounts[i+1] = CCounts[i]
		case 'C':
			CCounts[i+1] = 1 + CCounts[i]
			ACounts[i+1] = ACounts[i]
			TCounts[i+1] = TCounts[i]
			GCounts[i+1] = GCounts[i]
		}
	}

	var count int
	i := 0
	length := 2
	for {
		aCounts := ACounts[i+length]-ACounts[i]
		tCounts := TCounts[i+length]-TCounts[i]
		cCounts := CCounts[i+length]-CCounts[i]
		gCounts := GCounts[i+length]-GCounts[i]
		if aCounts == tCounts && cCounts == gCounts {
			count++
		}
		i++
		if i+length > n {
			i = 0
			length += 2
			if length > n {
				break
			}
		}
	}
	fmt.Println(count)
}
