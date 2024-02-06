#![no_std]
use gstd::{collections::HashMap, msg, prelude::*, ActorId};
use reputation_io::*;

#[derive(Debug, Default)]
pub struct Mtk {
    pub tokens: MtkData,
    pub creator: ActorId,
    pub supply: HashMap<TokenId, u128>,
}

static mut CONTRACT: Option<Mtk> = None;

#[no_mangle]
extern "C" fn init() {
    let InitMTK {
        name,
        symbol,
        base_uri,
    } = msg::load().expect("Unable to decode `InitMtk`");

    unsafe {
        CONTRACT = Some(Mtk {
            tokens: MtkData {
                name,
                symbol,
                base_uri,
                ..Default::default()
            },
            creator: msg::source(),
            ..Default::default()
        });
    }
}

#[no_mangle]
extern "C" fn handle() {
    let action: MTKAction = msg::load().expect("Failed to decode `MtkAction` message.");
    let mtk_contract = unsafe { CONTRACT.as_mut().expect("`Mtk` is not initialized.") };

    let reply = match action {
        MTKAction::Mint {
            id,
            amount,
            to,
            token_metadata,
        } => mtk_contract.mint(id, amount, to, token_metadata),

        MTKAction::AddSkillFt { ft_id, nft_id } => mtk_contract.add_ft(ft_id, nft_id),
    };

    msg::reply(reply, 0).expect("Failed to encode or reply with `Result<MtkEvent, MtkError>`.");
}

#[no_mangle]
extern "C" fn state() {
    let contract = unsafe { CONTRACT.take().expect("Unexpected error in taking state") };
    msg::reply::<State>(contract.into(), 0).expect(
        "Failed to encode or reply with `<ContractMetadata as Metadata>::State` from `state()`",
    );
}

impl Mtk {
    fn mint(
        &mut self,
        id: TokenId,
        amounts: u128,
        to: ActorId,
        _metadata: Option<TokenMetadata>,
    ) -> Result<MTKEvent, MTKError> {
        // 1. check if the token id exists
        if self.tokens.fungible_tokens.contains(&id) {
            // 2. if the receipient exists, add to the existing balance; else create a new balance
            let owner_balances = self.tokens.balances.get_mut(&id).unwrap();
            if let Some((_owner, balance)) =
                owner_balances.iter_mut().find(|(owner, _)| to.eq(owner))
            {
                *balance += amounts;
            } else {
                owner_balances.insert(to, amounts);
            }
            return Ok(MTKEvent::MintTo { to, id, amounts });
        }
        // 3. if the token id does not exist, return an error
        Err(MTKError::TokenDoesNotExists)
    }

    fn add_ft(&mut self, ft_id: TokenId, nft_id: TokenId) -> Result<MTKEvent, MTKError> {
        // if token id doens't exist, add it to the list of fungible tokens; else return an error
        if !self.tokens.fungible_tokens.contains(&ft_id) {
            self.tokens.fungible_tokens.push(ft_id);
            self.tokens.balances.insert(ft_id, HashMap::new());
            // add the matching pairs
            self.tokens.matching_pairs.insert(ft_id, nft_id);

            return Ok(MTKEvent::NewFtAdded { ft_id, nft_id });
        }

        Err(MTKError::TokenAlreadyExists)
    }
}

impl From<Mtk> for State {
    fn from(value: Mtk) -> Self {
        let Mtk {
            tokens,
            creator,
            supply,
        } = value;

        let MtkData {
            name,
            symbol,
            base_uri,
            balances,
            matching_pairs,
            fungible_tokens,
            skill_nfts,
            repu_nfts,
            token_metadata,
            owners,
        } = tokens;

        let balances = balances
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(|(a, b)| (a, b)).collect()))
            .collect();
        let matching_pairs = matching_pairs.into_iter().collect();
        let ft_tokens = fungible_tokens;
        let skill_nft_tokens = skill_nfts;
        let repu_nft_tokens = repu_nfts;
        let token_metadata = token_metadata.into_iter().map(|(k, v)| (k, v)).collect();
        let owners = owners.into_iter().map(|(k, v)| (k, v)).collect();
        let supply = supply.into_iter().map(|(k, v)| (k, v)).collect();
        Self {
            name,
            symbol,
            base_uri,
            balances,
            matching_pairs,
            ft_tokens,
            skill_nft_tokens,
            repu_nft_tokens,
            token_metadata,
            owners,
            creator,
            supply,
        }
    }
}
