package main

type RegisterDTO struct {
	Guid string `json:"guid"`
}

type ActionDTO struct {
	Kind     ActionKind `json:"kind"`
	Id       string     `json:"id,omitempty"`
	VoteKind VoteKind   `json:"vote_kind,omitempty"`
	Skip     bool       `json:"skip,omitempty"`
}

type ActionKind string
type VoteKind string

const (
	VoteProposal ActionKind = "VoteProposal"
)

const (
	Check VoteKind = "Check"
	Kill           = "Kill"
)
