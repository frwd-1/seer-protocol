use reth_primitives::TransactionSigned;
use reth_provider::Chain;

pub fn decode_chain_into_transactions(chain: &Chain) -> impl Iterator<Item = &TransactionSigned> {
    println!("Decoding chain into transactions");
    chain
        .blocks_iter()
        .flat_map(|block_with_senders| block_with_senders.body.iter())
}
