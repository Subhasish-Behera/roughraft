//observatio

use crate::node::nodeId;
use crate::log::LogEntry;
pub enum message<T> {
    AppendRequest(AppendRequestData<T>),
    AppendResponse(AppendResponseData),
    VoteRequest(VoteRequestData),
    VoteResponse(VoteResponseData)
}

pub struct VoteRequestData {
    from_id: nodeId,
    term: usize,
    last_log_index: usize,
    last_log_term: usize,
    commit_index: usize,


}

pub struct   VoteResponseData {
    from_id: nodeId,
    term: usize,
    vote_granted: bool,
}

pub struct AppendRequestData<T>{
    from_id: nodeId,
    term: usize,
    last_log_index: usize,
    last_log_term: usize,
    entries: Vec<LogEntry<T>>

}

pub struct AppendResponseData{
    from_id: nodeId,
    term: usize,
    last_log_index: usize,
    mismatch_index:Option<usize>,//can also do the same stuff by doing a ack index. you there it is the last log entry that was appended .
    // here it is the the index from where leader and current nodes index becomes diffrent
}