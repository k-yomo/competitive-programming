package main

import "fmt"

func main() {
	var n, t int
	fmt.Scan(&n, &t)
	var doorCloseAt, totalDoorOpenDuration int
	for i := 0; i < n; i++ {
		var enteredAt int
		fmt.Scan(&enteredAt)
		if enteredAt < doorCloseAt {
			totalDoorOpenDuration += enteredAt + t - doorCloseAt
			doorCloseAt = enteredAt + t
		} else {
			totalDoorOpenDuration += t
			doorCloseAt = enteredAt + t
		}
	}
	fmt.Println(totalDoorOpenDuration)
}
