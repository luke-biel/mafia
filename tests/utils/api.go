package utils

import (
	"bytes"
	"encoding/json"
	"fmt"
	"github.com/r3labs/sse"
	"io"
	"io/ioutil"
	"net/http"
)

func Action(player string, actionDto ActionDTO) {

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
	player, _ := json.Marshal(map[string]string{
		"name": name,
	})

	reqBody := bytes.NewBuffer(player)
	resp, err1 := http.Post("http://localhost:5069/register", "application/json", reqBody)

	if err1 != nil {
		panic(err1)
	}
	defer func(Body io.ReadCloser) {
		err := Body.Close()
		if err != nil {
			panic(err)
		}
	}(resp.Body)

	body, err2 := ioutil.ReadAll(resp.Body)

	if err2 != nil {
		panic(err2)
	}

	var response = RegisterDTO{}
	err3 := json.Unmarshal(body, &response)

	if err3 != nil {
		panic(err3)
	}

	return response.Guid
}
