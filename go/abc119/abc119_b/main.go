package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)

	var sum float64
	for i := 0; i < n; i++ {
		var currency string
		var amount float64
		fmt.Scan(&amount, &currency)
		if currency == "BTC" {
			sum += 380000.0 * amount
		} else {
			sum += amount
		}
	}
	fmt.Println(sum)
}
