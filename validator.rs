const PENALTY_PERCENTAGE: u64 = 10; // 10%

struct Validator { address: Address, stake: u64, } struct Nominator { address: Address, stakes: Vec<Validator>, }

fn penalize_validator(validator: &mut Validator) {
    // reduce the validator's stake by a fixed percentage
    validator.stake = validator.stake - (validator.stake * PENALTY_PERCENTAGE / 100);
}
