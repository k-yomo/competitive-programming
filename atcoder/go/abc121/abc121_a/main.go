package main

import "fmt"

func main() {
	var h,w, a,b int
	fmt.Scan(&h, &w, &a,&b)
	fmt.Println((h-a) * (w-b))
}