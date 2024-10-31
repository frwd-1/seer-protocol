// trading primitives

pub enum Underlying {
    Contract(Contract),
    RWA(RWA),
}

pub struct Terms {}

pub struct Token {
    pub address: Address,
    pub decimals: u8,
    pub security: Security,
    pub underlying: Underlying,
}

pub struct TradingPair {
    pub token0: Address,
    pub token1: Address,
}

pub struct TradingPool {
    pub pair: TradingPair,
    pub fee: u32,
}

struct SwapTerms {
    // specific swap terms fields
}

impl Terms for SwapTerms {
    fn validate(&self) -> bool {
        // implement validation logic
        true
    }
}

struct LendingTerms {
    // specific lending terms fields
}

impl Terms for LendingTerms {
    fn validate(&self) -> bool {
        // implement validation logic
        true
    }
}
