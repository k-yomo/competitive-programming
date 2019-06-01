package main

import (
	"fmt"
)

func main()  {
	var apple, piece int
	fmt.Scan(&apple, &piece)
	fmt.Println((apple * 3 + piece) / 2)
}
