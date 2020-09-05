package main

import "fmt"

func main() {
	var n int
	var s string
	fmt.Scan(&n, &s)

	accessory := "b"
	var count int
	for  {
		if len(accessory) > n {
			fmt.Println("-1")
			return
		}
		if accessory == s {
			fmt.Println(count)
			return
		}
		count++
		switch count%3 {
		case 0:
			accessory = fmt.Sprintf("b%sb", accessory)
		case 1:
			accessory = fmt.Sprintf("a%sc", accessory)
		case 2:
			accessory = fmt.Sprintf("c%sa", accessory)
		}
	}
}
