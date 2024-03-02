#![no_std]
use gstd::prelude::*;
use gstd::ActorId;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = reputation_io::State;

    pub fn get_skill_ft_by_id(state: State, actor_id: ActorId) -> Vec<(String, u128)> {
        let mut results = Vec::new();

        // Iterate over balances to find entries for actor_id
        for (token_id, balances) in &state.balances {
            if let Some(balance) = balances.iter().find(|(id, _)| *id == actor_id) {
                // Now, find the skill name corresponding to token_id
                if let Some(skill_ft_data) = state.skill_fungible_tokens.iter().find(|(id, _)| *id == *token_id) {
                    if let Some(name) = &skill_ft_data.1.name {
                        results.push((name.clone(), balance.1));
                    }
                }
            }
        }

        results
    }
}
