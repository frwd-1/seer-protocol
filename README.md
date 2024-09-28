# Seer Protocol

The Seer Protocol is a p2p protocol that enables a distributed and open source reputation system onchain, preserves privacy and enables a more secure ecosystem.

# Background

"An important and general problem seems to be that of tagging a negative behavior source for future recognition." - Nick Szabo, 1996

"Don't make a fool of yourself. Reputations matter" - Timothy May, Cyphernomicon, 1994

"We should allow people to develop reputations based on the quality of their ideas, rather than their job, wealth, age, or status." - Hal Finney, 1993

"The idea here is that the ultimate solution to the low
signal-to-noise ratio on the nets is not a matter of
forcing people to "stand behind their words". People can
stand behind all kinds of idiotic ideas. Rather, there
will need to be developed better systems for filtering news
and mail, for developing "digital reputations" which can be
stamped on one's postings to pass through these smart
filters, and even applying these reputations to pseudonyms.
In such a system, the fact that someone is posting or
mailing pseudonymously is not a problem, since nuisance
posters won't be able to get through." Hal Finney, 1993

Without user interfaces smart contracts are largely invisible

Without user interfaces, the broad scope of onchain actions enabled by smart contracts are largely invisible to ordinary users, making them vulnerable to "hidden actions"

```
if (x == true) {
    printf("x is false");
}
```

"Something that looks like a protocol but does not
accomplish a task is not a protocol—it’s a waste of time" - Bruce Schneier, Applied Cryptography

# getting started

add cargo-seer to path

- start a lighthouse client --
- run command:
  ETHERSCAN_API_KEY="" cargo run -p exex -- \
   node --full \
   --chain mainnet \
   --http \
   --authrpc.jwtsecret "" \
   --authrpc.addr 127.0.0.1 \
   --authrpc.port 8551 \
   --debug.etherscan

https://doc.rust-lang.org/nightly/rustc/platform-support/wasm32-wasip1.html

To build this target first acquire a copy of wasi-sdk. At this time version 22 is the minimum needed.

Next configure the WASI_SDK_PATH environment variable to point to where this is installed. For example:

export WASI_SDK_PATH=/path/to/wasi-sdk-22.0

rustup target add wasm32-wasip1

dynamically loading ExEx's as plugins

"wherever law ends, tyranny begins" - John Locke
