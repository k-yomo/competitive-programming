package main

import (
	"fmt"
)

func main() {
	var n, m int
	fmt.Scan(&n, &m)

	// 2人の老人は大人1 + 赤ちゃん1に分けられるため
	if m%2 != 0 {
		for adultCount := 0; adultCount < n; adultCount++ {
			babyCount := n - 1 - adultCount
			if adultCount*2+babyCount*4+3 == m {
				fmt.Println(adultCount, 1, babyCount)
				return
			}
		}
	} else {
		for adultCount := 0; adultCount <= n; adultCount++ {
			babyCount := n - adultCount
			if adultCount*2+babyCount*4 == m {
				fmt.Println(adultCount, 0, babyCount)
				return
			}
		}
	}
	fmt.Println("-1 -1 -1")
}

func main2() {
	var n, m int
	fmt.Scan(&n, &m)

	for elderlyCount := 0; elderlyCount <= n; elderlyCount++ {
		babyCount := (m - 2*n) / 2
		adultCount := (-m + 4*n) / 2
		if babyCount < 0 || adultCount < 0 {
			continue
		}

		if adultCount+elderlyCount+babyCount == n && adultCount*2+elderlyCount*3+babyCount*4 == m {
			fmt.Println(adultCount, elderlyCount, babyCount)
			return
		}
	}
	fmt.Println("-1 -1 -1")
}
