package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)
	monsterNums := ScanNums(n + 1)
	beatableMonseterNums := ScanNums(n)
	beatedMonsterNum := 0

	for i := 0; i < n; i++ {
		beatableNum := beatableMonseterNums[i]
		if beatableNum <= monsterNums[i] {
			beatedMonsterNum += beatableNum
		} else {
			beatedMonsterNum += monsterNums[i]
			beatableNum -= monsterNums[i]
			if beatableNum > 0 && monsterNums[i+1] > 0 {
				if beatableNum <= monsterNums[i+1] {
					beatedMonsterNum += beatableNum
					monsterNums[i+1] = monsterNums[i+1] - beatableNum
				} else {
					beatedMonsterNum += monsterNums[i+1]
					monsterNums[i+1] = 0
				}
			}
		}
	}
	fmt.Println(beatedMonsterNum)
}

func ScanNums(len int) (nums []int) {
	var num int
	for i := 0; i < len; i++ {
		fmt.Scan(&num)
		nums = append(nums, num)
	}
	return
}
