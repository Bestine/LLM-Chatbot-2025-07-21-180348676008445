use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 5000;

// Define a proposal struct
#[derive(CandidType, Deserialize)]
struct Proposal {
    description: String,
    approve: u32,
    reject: u32,
    pass: u32,
    is_active: bool,
    voted: Vec<candid::Principal>,
    owner: candid::Principal,
}

// Create proposal struct for create and edit proposal
#[derive(CandidType, Deserialize)]
struct CreateProposal {
    description: String,
    is_active: bool,
}

// Implement traits for Proposal
impl Storable for Proposal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(&self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

//Define limits for Proposal
impl BoundedStorable for Proposal {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}
