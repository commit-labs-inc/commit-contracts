use gstd::ActorId;
use crate::SkillNFT;

pub(crate) fn check_skill_nft(_holder: ActorId, _skill_tags: SkillNFT) -> bool {
    true
}

pub(crate) fn consume_skill_nft(_holder: ActorId, _skill_tags: SkillNFT) {}