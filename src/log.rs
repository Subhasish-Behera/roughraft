

use crate::node::nodeId;
use crate::node::Term;
//use it in future.
pub type LIndex = usize;



pub struct LogEntry<T> {
    pub term: u64,
    pub index:usize,
    pub data: T,
}


pub struct Log<T>{
    pub node_id: nodeId,
    pub entries: Vec<LogEntry<T>>,
    commited_entries: usize,
    applied_entries: usize,
    pub committed_len: LIndex,
}

impl<T> Log<T> {
    pub fn new(node_Id: nodeId) -> Self {
        Log{
            node_id:node_Id,
            committed_len: 0,
            entries: Vec::new(),
            commited_entries: 0,
            applied_entries: 0,

        }
    }
    pub fn last_term(&self) -> Term {
        self.entries.last().map(|x| x.term).unwrap_or(0)
    }
    pub fn last_idx(&self) -> LIndex {
        if !self.entries.is_empty() {
            self.entries.len() - 1
        } else {
            0
        }
    }
    pub fn append_entries(
        &mut self,
        prefix_idx: LIndex,
        leader_commit_len: LIndex,
        mut entries: Vec<LogEntry<T>>,
    ) {
        if !entries.is_empty() && self.entries.len() > prefix_idx {
            let rollback_to = std::cmp::min(self.entries.len(), prefix_idx + entries.len()) - 1;
            let our_last_term = self.entries[rollback_to].term;
            let leader_last_term = entries[rollback_to - prefix_idx].term;

            if our_last_term != leader_last_term {
                self.entries.truncate(prefix_idx);
            }
        }

        // Add new entries if we don't already have them
        if prefix_idx + entries.len() > self.entries.len() {
            let start = self.entries.len() - prefix_idx;
            self.entries.extend(entries.drain(start..));
        }

        // Update committed length
        if leader_commit_len > self.committed_len {
            self.committed_len = leader_commit_len;
        }
        }
    }
}