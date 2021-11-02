package main

import (
	"github.com/r3labs/sse"
	"testing"
	"github.com/mafia/utils"
)

func TestBlankFlow(t *testing.T) {
	Admin("reset")

	p1 := Register("p1")
	p2 := Register("p2")
	p3 := Register("p3")

	ch1 := make(chan *sse.Event, 16)
	ch2 := make(chan *sse.Event, 16)
	ch3 := make(chan *sse.Event, 16)

	Events(p1, ch1)
	Events(p2, ch2)
	Events(p3, ch3)

	Admin("roles 3,5,8") // city blank, mafia blank, syndicate blank

	// start game is broadcast
	AssertEventReceived(t, "{\"requiresResponse\":false,\"msg\":\"gameStart\"}", ch1, ch2, ch3)
	// no faction has 0 night role, thus we can move to day 1 immediately
	AssertEventReceived(t, "{\"requiresResponse\":false,\"msg\":\"timePasses\",\"details\":{\"day\":1,\"time_of_day\":\"Day\"}}", ch1, ch2, ch3)
	// only option is to propose day vote
	AssertEventReceived(t, "{\"requiresResponse\":true,\"msg\":\"ProposeVote\"}", ch1, ch2, ch3)

	Action(p1, ActionDTO{
		Kind:     VoteProposal,
		Id:       p3,
		VoteKind: Kill,
	})
}

