package main

import (
	"github.com/r3labs/sse"
	"github.com/stretchr/testify/assert"
	"testing"
)

func AssertEventReceived(t *testing.T, expected string, channels ...chan *sse.Event) {
	for _, ch := range channels {
		ev := <-ch
		assert.JSONEq(t, expected, string(ev.Data[:]))
	}
}
