package main

import (
	"fmt"
	"strings"
)

type player struct {
	name  string
	cards []string
}

func (p *player) playCard() string {
	card := p.cards[0]
	p.cards = p.cards[1:]
	return card
}

func main() {
	var a, b, c string
	fmt.Scan(&a, &b, &c)

	playerA := &player{name: "A", cards: strings.Split(a, "")}
	playerB := &player{name: "B", cards: strings.Split(b, "")}
	playerC := &player{name: "C", cards: strings.Split(c, "")}
	cardPlayerMap := map[string]*player{"a": playerA, "b": playerB, "c": playerC}

	curPlayer := playerA
	for {
		if len(curPlayer.cards) == 0 {
			fmt.Println(curPlayer.name)
			return
		}
		curPlayer = cardPlayerMap[curPlayer.playCard()]
	}
}
