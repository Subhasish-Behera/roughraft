use crate::log::{ Log, LogEntry, LIndex};
pub type nodeId = u64;
pub type Term = u64;
type Ticks = u32;
//pub type ServerId = usize;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
    ops::Div,
    vec,
};
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand_core::SeedableRng;
pub struct Config {
    pub timeout: u32,
    pub heartbeat_interval: u32,
    pub timeout_jitter: u32,
}
enum RaftState {
    Follower,
    Candidate,
    Leader,
}
pub struct FollowerState {
    pub election_time: Ticks,
    pub leader: Option<nodeId>,
}

/// [`Candidate`](RaftLeadershipState::Candidate) specific state
pub struct CandidateState {
    pub election_time: Ticks,
    pub votes_received: BTreeSet<nodeId>,
}
#[derive(Default)]
pub struct NonLeaderState {
    /// Index of next log entry to send to that server.
    /// Initialized to leader's last log index + 1
    pub sent_up_to: LIndex,

    /// Index of highest log entry known to be replicated on server.
    /// Initialized to 0, increases monotonically
    pub acked_up_to: LIndex,
}
/// [`Leader`](RaftLeadershipState::Leader) specific state
pub struct LeaderState {
    pub followers: BTreeMap<nodeId, NonLeaderState>,
    pub heartbeat_timeout: Ticks,
}

pub struct Node<T>{
    id: nodeId,
    peer_ids: BTreeSet<nodeId>,
    current_term: Term,
    current_term_vote:Option<nodeId>,
   // voted_for: Option<nodeId>,//current election vote
    state: RaftState,
    config: Config,
    //leadership_state:LeaderState,
    log: Log<T>,

}
fn rng_jitter(rng: &mut ChaCha8Rng, expected: u32, jitter: u32) -> u32 {
    let low = expected - jitter;
    let hi = expected + jitter;
    rng.gen_range(low..=hi)
}
impl<T> Node<T>
where
    T: Clone + Debug,
{
    pub fn new(
        id: nodeId,
        peers: BTreeSet<nodeId>,
        config: Config,
        seed: Option<u64>,
    ) -> Self {
        let mut rng = seed
            .map(ChaCha8Rng::seed_from_u64)
            .unwrap_or_else(ChaCha8Rng::from_entropy);
        let random_election_time = rng_jitter(&mut rng, config.timeout, config.timeout_jitter);
        Node {
            id: id,
            peer_ids: peers,
            config: config,
            current_term: 0,
            current_term_vote: None,
            log: Log::new(id),
            state: LeaderState::Follower(FollowerState {
                leader: None,
                election_time: random_election_time,
            }),
        }
    }
    pub fn promote_to_leader(
        &mut self,
        followers: BTreeMap<nodeId, NonLeaderState>,
    ) {
        let num_votes = followers.len() + 1;
        let follower_ids: Vec<nodeId> = followers.keys().cloned().collect();

        // Set state to Leader
        self.state = RaftState::Leader;
        self.log.replicate_to_followers(followers, self.config.heartbeat_interval);

        // Log the election win (if you decide to add logging or any custom action here)
        println!("Node {} won the election with {} votes. Followers: {:?}", self.id, num_votes, follower_ids);
    }

    pub fn is_leader(&self) -> bool {
        matches!(self.state, RaftState::Leader)
    }

    /// Whether the current node is a `Candidate`
    pub fn is_candidate(&self) -> bool {
        matches!(self.state, RaftState::Candidate)
    }

    /// Whether the current node is a `Follower`
    pub fn is_follower(&self) -> bool {
        matches!(self.state, RaftState::Follower)
    }

}