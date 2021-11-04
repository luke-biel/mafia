package main

import (
	"encoding/json"
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

	Admin("start 3,5,8")

	// start game is broadcast
	AssertEventReceived(t, "{\"requiresResponse\":false,\"msg\":\"GameStart\"}", ch1, ch2, ch3)

	var city, mafia, syndicate string
	var cityChan, mafiaChan, syndicateChan chan *sse.Event

	assignRole(ch1, p1, &city, &mafia, &syndicate, &cityChan, &mafiaChan, &syndicateChan)
	assignRole(ch2, p2, &city, &mafia, &syndicate, &cityChan, &mafiaChan, &syndicateChan)
	assignRole(ch3, p3, &city, &mafia, &syndicate, &cityChan, &mafiaChan, &syndicateChan)

	// no faction has 0 night role, thus we can move to day 1 immediately
	AssertEventReceived(t, "{\"requiresResponse\":false,\"msg\":\"TimePasses\",\"details\":{\"day\":1,\"timeOfDay\":\"Day\"}}", cityChan, mafiaChan, syndicateChan)

	// only option is to propose day vote
	AssertEventReceived(t, "{\"requiresResponse\":true,\"msg\":\"ProposeVote\"}", cityChan, mafiaChan, syndicateChan)

	assert.True(
		t,
		Action(city, ActionDTO{
			Action: VoteProposal,
			Details: &ActionDetails{
				Id:       syndicate,
				VoteKind: Kill,
			},
		}),
	)

	assert.True(
		t,
		Action(mafia, ActionDTO{
			Action: VoteProposal,
			Details: &ActionDetails{
				Id:       syndicate,
				VoteKind: Check,
			},
		}),
	)

	assert.True(
		t,
		Action(syndicate, ActionDTO{
			Action: VoteSkip,
		}),
	)

	// When all voted, time advances
	AssertEventReceived(t, "{\"requiresResponse\":false,\"msg\":\"TimePasses\",\"details\":{\"day\":1,\"timeOfDay\":\"Dusk\"}}", cityChan, mafiaChan, syndicateChan)
	// Players are asked to cast vote
	AssertEventReceived(t, "{\"requiresResponse\":true,\"msg\":\"CastVote\"}", cityChan, mafiaChan, syndicateChan)

	assert.True(t, Action(city, ActionDTO{
		Action: VoteTarget,
		Details: &ActionDetails{
			Id:       syndicate,
			VoteKind: Kill,
		},
	}))

	assert.True(t, Action(mafia, ActionDTO{
		Action: VoteTarget,
		Details: &ActionDetails{
			Id:       syndicate,
			VoteKind: Kill,
		},
	}))

	assert.True(t, Action(syndicate, ActionDTO{
		Action: VoteTarget,
		Details: &ActionDetails{
			Id:       syndicate,
			VoteKind: Check,
		},
	}))

	// For city and mafia the game advances into night
	AssertEventReceived(t, "{\"requiresResponse\":false,\"msg\":\"TimePasses\",\"details\":{\"day\":1,\"timeOfDay\":\"Night\"}}", cityChan, mafiaChan)
	// Syndicate gets killed by dusk voting
	AssertEventReceived(t, "{\"requiresResponse\":false,\"msg\":\"Killed\"}", syndicateChan)

	// Mafia gets a shot
	AssertEventReceived(t, "{\"requiresResponse\":true,\"msg\":\"Shoot\"}", mafiaChan)

	assert.True(t, Action(mafia, ActionDTO{
		Action: ShootTarget,
		Details: &ActionDetails{
			Id: city,
		},
	}))

	// Game ends for everyone
	AssertEventReceived(t, "{\"requiresResponse\":false,\"msg\":\"GameEnd\",\"details\":{\"faction\":\"Mafia\"}}", cityChan, mafiaChan, syndicateChan)
}

func assignRole(ch chan *sse.Event, id string, city *string, mafia *string, syndicate *string, cityChan *chan *sse.Event, mafiaChan *chan *sse.Event, syndicateChan *chan *sse.Event) {
	chRole := <-ch

	type Details struct {
		Card string `json:"card"`
	}

	var faction struct {
		Details Details `json:"details"`
	}
	bytes := chRole.Data[:]
	err := json.Unmarshal(bytes, &faction)
	if err != nil {
		panic(err)
	}

	switch faction.Details.Card {
	case "CityBlank":
		*city = id
		*cityChan = ch
		break
	case "SyndicateBlank":
		*syndicate = id
		*syndicateChan = ch
		break
	case "MafiaBlank":
		*mafia = id
		*mafiaChan = ch
		break
	}
}

func debugChannels(channels ...chan *sse.Event) {
	for _, ch := range channels {
		ev := <-ch
		fmt.Printf("%s\n", string(ev.Data[:]))
	}
}
