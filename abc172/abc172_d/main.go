package main

import "fmt"

var diviserNumMap = map[int]int{}

func main() {
	var n int
	fmt.Scan(&n)
	var sum int
	for i := 1; i <= n; i++ {
		num := NumberOfDivisors(i)
		diviserNumMap[i] = num
		sum += i * num
	}
	fmt.Println(sum)
}

func PrimeFactorization(n int) (pfs map[int]int) {
	pfs = map[int]int{}

	// Get the number of 2s that divide n
	for n%2 == 0 {
		pfs[2] += 1
		n = n / 2
		if diviserNumMapping[n] != 0 {
			newMap := map[int]int{}
			for k, v := range diviserNumMap[n] {
				newMap[k] = v
			}
			newMap[2] += pfs[2]
			return newMap
		}
	}

	// n must be odd at this point. so we can skip one element
	// (note i = i + 2)
	for i := 3; i*i <= n; i = i + 2 {
		// while i divides n, append i and divide n
		for n%i == 0 {
			if _, ok := pfs[i]; ok {
				pfs[i] += 1
			} else {
				pfs[i] = 1
			}
			n = n / i
			if len(diviserNumMap[n]) != 0 {
				newMap := map[int]int{}
				for k, v := range diviserNumMap[n] {
					newMap[k] = v
				}
				for k, v := range pfs {
					newMap[k] += v
				}
				return newMap
			}
		}
	}

	// This condition is to handle the case when n is a prime number
	// greater than 2
	if n > 2 {
		pfs[n] = 1
	}

	return pfs
}

func NumberOfDivisors(n int) int {
	pfs := PrimeFactorization(n)

	num := 1
	for _, exponents := range pfs {
		num *= exponents + 1
	}

	return num
}
