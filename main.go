package main

import (
	"context"
	"log"
	"os"

	"bytes"
	"encoding/json"
	"net/http"

	"github.com/joho/godotenv"

	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"

	"github.com/libp2p/go-libp2p/core/host"
)

type TransactionData struct {
	Nonce    uint64 `json:"nonce"`
	GasPrice string `json:"gasPrice"`
	GasLimit uint64 `json:"gasLimit"`
	To       string `json:"to,omitempty"`
	Value    string `json:"value"`
	Data     string `json:"data"`
}

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

	// p2pHost, err := enode.NewHost()
	// if err != nil {
	// 	log.Fatalf("Failed to create P2P host: %v", err)
	// }

	return &SeerNode{
		config:    config,
		ethClient: ethClient,
		// p2pHost:   p2pHost,
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
	txData := TransactionData{
		Nonce:    tx.Nonce(),
		GasPrice: tx.GasPrice().String(),
		GasLimit: tx.Gas(),
		To: func() string {
			if tx.To() == nil {
				return ""
			} else {
				return tx.To().Hex()
			}
		}(),
		Value: tx.Value().String(),
		Data:  string(tx.Data()),
	}

	jsonData, err := json.Marshal(txData)
	if err != nil {
		log.Printf("Failed to serialize transaction data: %v", err)
		return
	}

	// HTTP POST to local Python service
	_, err = http.Post("http://localhost:5000/transaction", "application/json", bytes.NewBuffer(jsonData))
	if err != nil {
		log.Printf("Failed to send transaction data: %v", err)
	}
}

func main() {
	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	ethereumNodeURL := os.Getenv("ETHEREUM_NODE_URL")
	seerNodeAddress := os.Getenv("SEER_NODE_ADDRESS")

	config := NodeConfig{
		EthereumNodeURL: ethereumNodeURL,
		SeerNodeAddress: seerNodeAddress,
	}
	seerNode := NewSeerNode(config)
	go seerNode.MonitorBlockchain()

	select {}
}
