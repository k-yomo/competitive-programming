package main

import "fmt"

func main() {
	var price int
	fmt.Scan(&price)

	change := 1000 - price
	coins := []int{500, 100, 50, 10, 5, 1}

	coinNum := 0
	for _, coin := range coins {
		coinNum += change / coin
		change = change % coin
	}

	fmt.Println(coinNum)
}
