package main

import (
	"fmt"
	"github.com/r3labs/sse"
	"github.com/stretchr/testify/assert"
	"testing"
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
	AssertEventReceived(t, "{\"requiresResponse\":false,\"msg\":\"GameStart\"}", ch1, ch2, ch3)
	// no faction has 0 night role, thus we can move to day 1 immediately
	AssertEventReceived(t, "{\"requiresResponse\":false,\"msg\":\"TimePasses\",\"details\":{\"day\":1,\"timeOfDay\":\"Day\"}}", ch1, ch2, ch3)
	// only option is to propose day vote
	AssertEventReceived(t, "{\"requiresResponse\":true,\"msg\":\"ProposeVote\"}", ch1, ch2, ch3)

	assert.True(
		t,
		Action(p1, ActionDTO{
			Action: VoteProposal,
			Details: &ActionDetails{
				Id:       p3,
				VoteKind: Kill,
			},
		}),
	)

	assert.True(
		t,
		Action(p2, ActionDTO{
			Action: VoteProposal,
			Details: &ActionDetails{
				Id:       p3,
				VoteKind: Check,
			},
		}),
	)

	assert.True(
		t,
		Action(p3, ActionDTO{
			Action: VoteSkip,
		}),
	)

	// When all voted, time advances
	AssertEventReceived(t, "{\"requiresResponse\":false,\"msg\":\"TimePasses\",\"details\":{\"day\":1,\"timeOfDay\":\"Dusk\"}}", ch1, ch2, ch3)
	// Players are asked to cast vote
	AssertEventReceived(t, "{\"requiresResponse\":true,\"msg\":\"CastVote\"}", ch1, ch2, ch3)

	assert.True(t, Action(p1, ActionDTO{
		Action: VoteTarget,
		Details: &ActionDetails{
			Id:       p3,
			VoteKind: Kill,
		},
	}))

	assert.True(t, Action(p2, ActionDTO{
		Action: VoteTarget,
		Details: &ActionDetails{
			Id:       p3,
			VoteKind: Kill,
		},
	}))

	assert.True(t, Action(p3, ActionDTO{
		Action: VoteTarget,
		Details: &ActionDetails{
			Id:       p3,
			VoteKind: Check,
		},
	}))

	debugChannels(ch1, ch2, ch3)
}

func debugChannels(channels ...chan *sse.Event) {
	for _, ch := range channels {
		ev := <-ch
		fmt.Printf("%s\n", string(ev.Data[:]))
	}
}
