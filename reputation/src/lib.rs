#![no_std]
use gstd::{collections::HashMap, exec, msg, prelude::*, ActorId};
use reputation_io::*;

#[derive(Debug, Default)]
pub struct Mtk {
    pub tokens: MtkData,
    pub creator: ActorId,
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
        MTKAction::MintFtTo { id, amount, to } => mtk_contract.mint_ft_to(id, amount, to),
        MTKAction::MintNftTo { to, metadata } => mtk_contract.mint_nft_to(to, metadata),
        MTKAction::Burn { id, from, amount } => mtk_contract.burn(id, from, amount),
        MTKAction::ChangeBaseUri { new_base_uri } => mtk_contract.change_base_uri(new_base_uri),
        MTKAction::AddFt { token_data } => mtk_contract.add_ft(token_data),
        MTKAction::ChangeFt { id, new_data } => mtk_contract.change_ft(id, new_data),
        MTKAction::AddNft { name } => mtk_contract.add_nft(name),
        MTKAction::RemoveNft { name } => mtk_contract.remove_nft(name),
        MTKAction::VerifyReputation {
            target,
            skill_type,
            token_id,
        } => mtk_contract.verify_repu(target, skill_type, token_id),
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
    fn mint_ft_to(&mut self, id: TokenId, amount: u128, to: ActorId) -> Result<MTKEvent, MTKError> {
        // 1. check if the token id exists
        if self.tokens.skill_fungible_tokens.contains_key(&id) {
            // 2. if the receipient exists, add to the existing balance; else create a new balance
            let owner_balances = self.tokens.balances.get_mut(&id).unwrap();
            if let Some((_owner, balance)) =
                owner_balances.iter_mut().find(|(owner, _)| to.eq(owner))
            {
                *balance += amount;
            } else {
                owner_balances.insert(to.clone(), amount);
                // add the new token to this owner's list of fts
                self.tokens.ft_owners.entry(to.clone()).or_insert_with(Vec::new).push(id.clone());
            }
            return Ok(MTKEvent::SkillTokenMinted { id, amount, to });
        }
        // 3. if the token id does not exist, return an error
        Err(MTKError::TokenDoesNotExists)
    }

    fn mint_nft_to(
        &mut self,
        to: ActorId,
        metadata: SkillNftMetadata,
    ) -> Result<MTKEvent, MTKError> {
        // check if the name in the metadata exists in the available skill names
        if !self
            .tokens
            .available_skill_names
            .check_name(&metadata.title.clone().unwrap())
        {
            return Err(MTKError::SkillNameDoesNotExists);
        }
        // 1. generate a token id for the soon to be minted NFT.
        let id = gen_token_id();
        // 2. check if the token id already exists
        if self.tokens.skill_nft_metadata.contains_key(&id) {
            Err(MTKError::TokenAlreadyExists)
        } else {
            // 3. if the token id does not exist, add it to the list of NFTs
            self.tokens.skill_nft_metadata.insert(id, metadata);
            // 4. add the NFT to the list of NFTs owned by the receipient
            self.tokens
                .nft_owners
                .entry(to)
                .or_insert_with(Vec::new)
                .push(id);
            Ok(MTKEvent::SkillNftMinted { id, to })
        }
    }

    fn burn(&mut self, id: TokenId, from: ActorId, amount: u128) -> Result<MTKEvent, MTKError> {
        // 1. check if the token id exists
        if self.tokens.skill_fungible_tokens.contains_key(&id) {
            // 2. if the owner exists, subtract from the existing balance; else return an error
            let owner_balances = self.tokens.balances.get_mut(&id).unwrap();
            if let Some((_owner, balance)) =
                owner_balances.iter_mut().find(|(owner, _)| from.eq(owner))
            {
                if *balance < amount {
                    return Err(MTKError::InsufficientBalance);
                }
                *balance -= amount;
                return Ok(MTKEvent::SkillTokenBurned { id, amount, from });
            }
            return Err(MTKError::OwnerDoesNotExists);
        }
        // 3. if the token id does not exist, return an error
        Err(MTKError::TokenDoesNotExists)
    }

    fn add_ft(&mut self, token_data: SkillFtData) -> Result<MTKEvent, MTKError> {
        // only contract creator can add a new fungible token
        if msg::source() != self.creator {
            return Err(MTKError::OnlyCreaterCanOperate);
        }

        let id = gen_token_id();
        self.tokens
            .skill_fungible_tokens
            .insert(id.clone(), token_data);
        self.tokens.balances.insert(id.clone(), HashMap::new());
        Ok(MTKEvent::NewFtAdded { id })
    }

    fn change_ft(&mut self, id: TokenId, new_data: SkillFtData) -> Result<MTKEvent, MTKError> {
        // only contract creator can add a new fungible token
        if msg::source() != self.creator {
            return Err(MTKError::OnlyCreaterCanOperate);
        }

        // check if the token id exists
        if self.tokens.skill_fungible_tokens.contains_key(&id) {
            self.tokens.skill_fungible_tokens.insert(id, new_data);
            Ok(MTKEvent::SkillFtChanged { id })
        } else {
            Err(MTKError::TokenDoesNotExists)
        }
    }

    fn change_base_uri(&mut self, new_uri: String) -> Result<MTKEvent, MTKError> {
        self.tokens.base_uri = new_uri.clone();
        Ok(MTKEvent::MtkUriChanged { new_uri })
    }

    fn add_nft(&mut self, name: String) -> Result<MTKEvent, MTKError> {
        // only contract creator can add a new skill nft.
        if msg::source() != self.creator {
            return Err(MTKError::OnlyCreaterCanOperate);
        }

        // add the new name to the available names set.
        self.tokens.available_skill_names.add_name(name.clone());
        Ok(MTKEvent::SkillNftAdded { name })
    }

    fn remove_nft(&mut self, name: String) -> Result<MTKEvent, MTKError> {
        // only contract creator can add a new skill nft.
        if msg::source() != self.creator {
            return Err(MTKError::OnlyCreaterCanOperate);
        }

        // remove the name from the available names set.
        if self.tokens.available_skill_names.remove_name(&name) {
            Ok(MTKEvent::SkillNftRemoved { name })
        } else {
            Err(MTKError::SkillNameDoesNotExists)
        }
    }

    fn verify_repu(
        &mut self,
        target: ActorId,
        skill_type: bool,
        token_id: TokenId,
    ) -> Result<MTKEvent, MTKError> {
        if skill_type {
            // search for the token id in the owner's NFTs
            if self.tokens.nft_owners.contains_key(&target) {
                if self
                    .tokens
                    .nft_owners
                    .get(&target)
                    .unwrap()
                    .contains(&token_id)
                {
                    Ok(MTKEvent::RepuVerified {
                        initiator: msg::source(),
                        target,
                    })
                } else {
                    Ok(MTKEvent::RepuVerificationFail {
                        initiator: msg::source(),
                        target,
                    })
                }
            } else {
                Ok(MTKEvent::RepuVerificationFail {
                    initiator: msg::source(),
                    target,
                })
            }
        } else {
            // search for the token id in the owner's FTs
            if self.tokens.ft_owners.contains_key(&target) {
                if self
                    .tokens
                    .ft_owners
                    .get(&target)
                    .unwrap()
                    .contains(&token_id)
                {
                    Ok(MTKEvent::RepuVerified {
                        initiator: msg::source(),
                        target,
                    })
                } else {
                    Ok(MTKEvent::RepuVerificationFail {
                        initiator: msg::source(),
                        target,
                    })
                }
            } else {
                Ok(MTKEvent::RepuVerificationFail {
                    initiator: msg::source(),
                    target,
                })
            }
        }
    }
}

impl From<Mtk> for State {
    fn from(value: Mtk) -> Self {
        let Mtk {
            tokens,
            creator,
        } = value;

        let MtkData {
            name,
            symbol,
            base_uri,
            available_skill_names,
            balances,
            skill_fungible_tokens,
            ft_owners,
            skill_nft_metadata,
            nft_owners,
        } = tokens;

        let balances = balances
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(|(a, b)| (a, b)).collect()))
            .collect();
        let skill_fungible_tokens = skill_fungible_tokens
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();
        let ft_owners = ft_owners.into_iter().map(|(k, v)| (k, v)).collect();
        let skill_nft_metadata = skill_nft_metadata
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();
        let nft_owners = nft_owners.into_iter().map(|(k, v)| (k, v)).collect();

        Self {
            name,
            symbol,
            base_uri,
            creator,
            available_skill_names,
            balances,
            skill_fungible_tokens,
            ft_owners,
            skill_nft_metadata,
            nft_owners,
        }
    }
}

fn gen_token_id() -> TokenId {
    exec::block_timestamp().into()
}
