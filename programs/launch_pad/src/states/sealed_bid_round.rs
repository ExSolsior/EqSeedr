use crate::utils::*;
use anchor_lang::prelude::*;

#[account]
pub struct SealedBidRound {
    pub bump: u8,
    pub authority: Pubkey,
    pub session: Pubkey,

    status: Status,

    pub total_sealed_bids: u32,
    pub total_unsealed_bids: u32,
}

impl SealedBidRound {
    pub const LEN: usize = DISCRIMINATOR
        + UNSIGNED_8
        + PUBKEY_BYTES
        + PUBKEY_BYTES
        + Status::LEN
        + UNSIGNED_32
        + UNSIGNED_32;

    pub fn initialize(&mut self, bump: u8, authority: Pubkey, session: Pubkey) {
        self.bump = bump;
        self.authority = authority;
        self.session = session;

        self.status = Status::Enqueue;

        self.total_sealed_bids = 0;
        self.total_unsealed_bids = 0;

        // emit event
    }

    pub fn next_index(&self) -> String {
        return (self.total_sealed_bids + 1).to_string();
    }

    pub fn get_index(&self) -> u32 {
        return self.total_sealed_bids + 1;
    }

    pub fn update_total_sealed_bids(&mut self) {
        self.total_sealed_bids += 1;
    }

    pub fn update_total_unsealed_bids(&mut self) {
        self.total_unsealed_bids += 1;
    }

    // VALIDATIONS:
    pub fn is_valid_stake_amount(&self) -> bool {
        return true;
    }

    pub fn is_valid_sealed_bid_phase(&self) -> bool {
        match self.status {
            Status::SealedBidPhase => !true,
            _ => !false,
        }
    }

    pub fn is_valid_unsealed_bid_phase(&self) -> bool {
        match self.status {
            Status::UnsealBidPhase => !true,
            _ => !false,
        }
    }

    // don't think this is necessary, will combine the commit bid phase with unsealed bid phase
    pub fn is_valid_commit_bid_phase(&self) -> bool {
        return true;
    }

    pub fn is_valid_commit_amount(&self) -> bool {
        // check user balance
        return true;
    }

    pub fn has_not_commit(&self) -> bool {
        return true;
    }

    pub fn is_valid_sealed_bid_round(&self) -> bool {
        // use unix timestamp || status
        return true;
    }

    pub fn is_valid_unsealed_bid(&self) -> bool {
        return self.total_unsealed_bids < self.total_sealed_bids;
    }

    // pub fn is_valid() {}
}

// #[account]
// pub struct SealedBidByIndex {
//     // VALIDATION STATE
//     pub bump: u8,
//     pub index: u32,
//     pub session: Pubkey,
//     pub owner: Pubkey,

//     // STATE
//     pub commit_hash: Pubkey, // technially a hash [u8; 32]
//     pub staked_amount: u64,
//     pub is_unsealed: bool,
// }

// impl SealedBidByIndex {
//     pub fn initialize(
//         &mut self,
//         bump: u8,
//         index: u32,
//         session: Pubkey,
//         owner: Pubkey,
//         amount: u64,
//         commit_hash: Pubkey,
//     ) {
//         self.bump = bump;
//         self.index = index;
//         self.session = session;
//         self.owner = owner;

//         self.commit_hash = commit_hash;
//         self.staked_amount = amount;

//         self.is_unsealed = false;

//         // emit event
//     }

//     // VALIDATIONS:
//     pub fn is_valid_unsealed_bid(&self, amount: u64, secret: String, session: Pubkey) -> bool {
//         let hasher = Hasher::default();
//         hasher.hash(amount.as_ref());
//         hasher.hash(sealed_bid_by_index.index.to_string().as_ref());
//         hasher.hash(session.as_ref());

//         // convert to Pubkey
//         let hash = hasher.result();
//         return hash == self.commit_hash;
//     }

//     pub fn unsealed_bid(&mut self) {
//         self.is_unsealed = true;
//     }
// }

// #[account]
// pub struct CommitLeaderBoard {
//     pub bump: u8,
//     pub session: Pubkey,
//     pub min_target: u64, // cutoff / bottom amount, increaese when commit queue has 10 -> I don't think I need this
//     pub pool: LinkedList<Commit>,
// }

// impl Len for CommitLeaderBoard {
//     const LEN: usize =
//         DISCRIMINATOR + BUMP + PUBKEY_BYTES + UNSIGNED_64 + LinkedList::<Commit>::LEN;
// }

// impl CommitLeaderBoard {
//     pub fn initialize(&mut self, bump: u8, session: Pubkey) {
//         self.bump = bump;
//         self.session = session;
//         self.min_target = 0;

//         self.pool = LinkedList<Commit>::new();
//     }

//     pub fn update(&self, owner: Pubkey, amount: u64) {
//         // add this code later. going to need index info for linked list
//         self.pool;
//     }

//     pub fn is_valid_commit_leader_board(session: Pubkey) -> bool {
//         return self.session == session;
//     }
// }

// #[account]
// pub struct CommitQueue {
//     pub bump: u8,
//     pub session: Pubkey,
//     pointer: u8,
//     queue: Vec<Commit>,
// }

// const MAX_CAPACITY: usize = 10;
// impl CommitQueue {
//     const LEN: usize =
//         DISCRIMINATOR + BUMP + PUBKEY_BYTES + BYTE + (UNSIGNED_128 + (Commit::LEN * MAX_CAPACITY));

//     pub fn initialize(&mut self, bump: u8, session: Pubkey) {
//         self.bump = bump;
//         self.session = session;
//         self.queue = Vec::new();

//         // emit event
//     }

//     pub fn insert(&mut self, commit: Commit) {
//         let mut index = self.queue.len();

//         while index != 0 && commit.amount > self.queue[index - 1].unwrap().amount {
//             index -= 1;
//         }

//         if self.queue.len() != 0 && self.queue.len() == MAX_CAPACITY {
//             self.queue.insert(index, commit).pop();
//         } else if self.queue.len() != 0 && index < MAX_CAPACITY && index < self.queue.len() {
//             self.queue.insert(index, commit);
//         } else {
//             self.queue.push(commit);
//         }

//         // emit event element was added
//     }

//     pub fn dequeue(&mut self) -> Option<Commit> {
//         let index = self.pointer;
//         self.pointer += 1;
//         return self.queue[index];
//     }

//     pub fn is_valid_insert(&self, commit: Commit) -> bool {
//         return self.queue.len() == MAX_CAPACITY
//             && commit.amount > self.queue[self.queue.len() - 1];
//     }

//     pub fn is_valid_dequeue(&self) -> bool {
//         return self.point < MAX_CAPACITY;
//     }

//     pub fn is_valid_session(&self, session: Session) -> bool {
//         return self.session == session.key();
//     }
// }

// pub struct Commit {
//     pub bidder_index: u32,
//     pub amount: u64,
// }

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
enum Status {
    Enqueue,
    SealedBidPhase,
    UnsealBidPhase,
    Canceled,
}

impl Status {
    const LEN: usize = BYTE;
}

// sealed bid system
//  SealedBidRound
//  SealedBidByIndex
//  CommitLeaderBoard
//  CommitQueue
