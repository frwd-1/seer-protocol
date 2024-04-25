package main

import (
	"github.com/frwd-1/SeerProtocol/node"
)

func main() {
	node := node.NewSeerNode(node.NodeConfig{
		EthereumNodeURL: "http://localhost:8545",
		SeerNodeAddress: "/ip4/
	})