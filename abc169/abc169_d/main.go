package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)
	pfs := PrimeFactors(n)
	fmt.Println(pfs)
	var i, count int
	for {
		i++
		for _, pf := range pfs {

		}
		if n == 1 {

		}
	}
}

func PrimeFactors(n int) (pfs []int) {
	for n%2 == 0 {
		pfs = append(pfs, 2)
		n = n / 2
	}

	for i := 3; i*i <= n; i = i + 2 {
		for n%i == 0 {
			pfs = append(pfs, i)
			n = n / i
		}
	}

	if n > 2 {
		pfs = append(pfs, n)
	}

	return
}
