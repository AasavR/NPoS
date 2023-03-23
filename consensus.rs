fn select_validators(nominators: &Vec<Nominator>, num_validators: usize, validators: &Vec<Validator>) -> Vec<Validator> {
    // Calculate the total stake of each validator
    let mut validator_stakes: HashMap<String, u64> = HashMap::new();
    for nominator in nominators {
        for validator in &nominator.stakes {
            *validator_stakes.entry(validator.address.clone()).or_insert(0) += nominator.stakes.iter().find(|v| v.address == validator.address).unwrap().stake;
        }
    }

    // Sort the validators by their stake in descending order
    let mut sorted_validators: Vec<(&String, &u64)> = validator_stakes.iter().collect();
    sorted_validators.sort_by_key(|&(_, stake)| Reverse(stake));

    // Return the top num_validators validators
    let mut selected_validators: Vec<Validator> = Vec::new();
    for (address, _) in sorted_validators.iter().take(num_validators) {
        selected_validators.push(validators.iter().find(|validator| validator.address == **address).unwrap().clone());
    }

    selected_validators
}
  fn distribute_rewards(validators: &Vec<Validator>, nominators: &Vec<Nominator>) {
    // Calculate the total stake of each validator
    let mut validator_stakes: HashMap<String, u64> = HashMap::new();
    for nominator in nominators {
        for validator in &nominator.stakes {
            *validator_stakes.entry(validator.address.clone()).or_insert(0) += nominator.stakes.iter().find(|v| v.address == validator.address).unwrap().stake;
        }
    }

    // Calculate the total reward for this round
    let total_reward: u64 = 100;

    // Calculate the reward for each validator based on their stake
    for validator in validators {
        let validator_stake = validator_stakes.get(&validator.address).unwrap();
