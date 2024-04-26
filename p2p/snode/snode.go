package snode

import (
	libp2p "github.com/libp2p/go-libp2p"
	"github.com/libp2p/go-libp2p/core/host"
)

func NewHost() (host.Host, error) {
	return libp2p.New(
		libp2p.ListenAddrStrings("/ip4/0.0.0.0/tcp/4001"), // You can specify your network listening options here
		// ... add other options as needed
	)
}
