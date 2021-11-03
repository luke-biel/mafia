package main

type RegisterDTO struct {
	Guid string `json:"guid"`
}

type ActionDTO struct {
	Action  ActionKind     `json:"kind"`
	Details *ActionDetails `json:"details,omitempty"`
}

type ActionDetails struct {
	Id       string   `json:"id,omitempty"`
	VoteKind VoteKind `json:"voteKind,omitempty"`
}

type ActionKind string
type VoteKind string

const (
	VoteProposal ActionKind = "VoteProposal"
	VoteSkip                = "VoteSkip"
	VoteTarget              = "VoteTarget"
)

const (
	Check VoteKind = "check"
	Kill           = "kill"
)
