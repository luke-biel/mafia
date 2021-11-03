package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"github.com/r3labs/sse"
	"io"
	"io/ioutil"
	"net/http"
)

func Action(player string, actionDto ActionDTO) bool {
	action, err1 := json.Marshal(actionDto)
	if err1 != nil {
		panic(err1)
	}
	reqBody := bytes.NewBuffer(action)
	req, err2 := http.NewRequest(http.MethodPost, "http://localhost:5069/action", reqBody)
	if err2 != nil {
		panic(err2)
	}
	req.AddCookie(&http.Cookie{Name: "mafia-guid", Value: player})
	resp, err3 := http.DefaultClient.Do(req)
	if err3 != nil {
		panic(err3)
	}
	return resp.StatusCode == 200
}

func Admin(body string) {
	reqBody := bytes.NewBuffer([]byte(body))
	_, err1 := http.Post("http://localhost:5069/admin", "text/plain", reqBody)
	if err1 != nil {
		panic(err1)
	}
}

func Events(id string, channel chan *sse.Event) {
	client := sse.NewClient("http://localhost:5069/events")
	client.Headers["Cookie"] = fmt.Sprintf("mafia-guid=%s", id)
	err := client.SubscribeChan(id, channel)
	if err != nil {
		panic(err)
	}
}

func Register(name string) string {
	player, err1 := json.Marshal(map[string]string{
		"name": name,
	})
	if err1 != nil {
		panic(err1)
	}

	reqBody := bytes.NewBuffer(player)
	resp, err2 := http.Post("http://localhost:5069/register", "application/json", reqBody)

	if err2 != nil {
		panic(err1)
	}
	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			panic(err)
		}
	}(resp.Body)

	body, err3 := ioutil.ReadAll(resp.Body)

	if err3 != nil {
		panic(err3)
	}

	var response = RegisterDTO{}
	err4 := json.Unmarshal(body, &response)

	if err4 != nil {
		panic(err4)
	}

	return response.Guid
}
