package main

import (
	"context"
	"log"

	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"
	libp2p "github.com/libp2p/go-libp2p"
	"github.com/libp2p/go-libp2p/core/host"
)

type NodeConfig struct {
	EthereumNodeURL string
	SeerNodeAddress string
}

type SeerNode struct {
	config    NodeConfig
	ethClient *ethclient.Client
	p2pHost   host.Host
}

func NewSeerNode(config NodeConfig) *SeerNode {
	ethClient, err := ethclient.Dial(config.EthereumNodeURL)
	if err != nil {
		log.Fatalf("Failed to connect to Ethereum node: %v", err)
	}

	p2pHost, err := libp2p.New(
		libp2p.ListenAddrStrings("/ip4/0.0.0.0/tcp/0"), // Specify network listening options
	)
	if err != nil {
		log.Fatalf("Failed to create P2P host: %v", err)
	}

	return &SeerNode{
		config:    config,
		ethClient: ethClient,
		p2pHost:   p2pHost,
	}
}

func (node *SeerNode) MonitorBlockchain() {
	headers := make(chan *types.Header)
	sub, err := node.ethClient.SubscribeNewHead(context.Background(), headers)
	if err != nil {
		log.Fatal(err)
	}
	for {
		select {
		case err := <-sub.Err():
			log.Fatal(err)
		case header := <-headers:
			node.processBlock(header)
		}
	}
}

func (node *SeerNode) processBlock(header *types.Header) {
	block, err := node.ethClient.BlockByHash(context.Background(), header.Hash())
	if err != nil {
		log.Printf("Failed to fetch block: %v", err)
		return
	}
	for _, tx := range block.Transactions() {
		node.processTransaction(tx)
	}
}

func (node *SeerNode) processTransaction(tx *types.Transaction) {
	log.Printf("Processing transaction: %s", tx.Hash().Hex())
}

func main() {
	config := NodeConfig{
		EthereumNodeURL: "ws://localhost:8546",
		SeerNodeAddress: "example-seer-node-address",
	}
	seerNode := NewSeerNode(config)
	go seerNode.MonitorBlockchain()

	select {}
}
