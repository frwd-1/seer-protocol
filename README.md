# SeerProtocol

A project for the distributed and open source identification of all on-chain nodes and relationships

# Tenets

The following tenets are laid out here to establish and formalize the fundamental assumptions of the Seer Protocol

- State root hash defines a valid identity based on rules that allow certain actions (valid identities) and disallow others (invalid identities). This is to say - action is identity onchain

- A consensus protocol is historically defined by these rules as agreed upon by a peer to peer network. Nodes come to consensus on the valid hash of each new state

- EVM based chains limit their identification of actions on their network only to a limited scope of valid state transitions and invalid state transitions. Consider the DAO hack.

- Smart contracts enable unlimited possible actions

- Instead of just validating actions, it is possible to use a consensus protocol to define, name, and identify the broader scope of possible actions.

- As the broader scope of onchain actions are named and identified, users and protocols can make judgements about what they value.

- Without user interfaces, the broader scope of onchain actions enabled by smart contracts are largely invisible to ordinary users, making them vulnerable to "hidden actions". ie, those that cannot easily find and interpret the following code in a smart contract are defenseless:

```
if (x == true) {
    printf("x is false");
}
```
