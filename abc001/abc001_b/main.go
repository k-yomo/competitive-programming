package main

import "fmt"

func main()  {
	var m float64
	fmt.Scan(&m)
	m = m/1000
	switch {
	case m < 0.1:
		fmt.Println("00")
	case 0.1 <= m && m <= 5:
		m = m*10
		if m < 10 {
			fmt.Printf("0%d\n", int(m))
		} else {
			fmt.Println(int(m))
		}
	case 6 <= m && m <= 30:
		fmt.Println(int(m)+50)
	case 35 <= m && m <= 70:
		fmt.Println(int((m-30)/5+80))
	case 70 <= m  :
		fmt.Println(89)
	}
}
