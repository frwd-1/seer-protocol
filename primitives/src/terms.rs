pub trait Terms {
    fn validate(&self) -> bool;
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
