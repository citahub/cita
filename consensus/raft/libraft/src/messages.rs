//! Utility functions for working with Cap'n Proto Raft messages.
#![allow(dead_code)]

use std::net::SocketAddr;
use std::rc::Rc;

use capnp::message::{Builder, HeapAllocator};

use {ClientId, Term, LogIndex, ServerId};
use messages_capnp::{client_request, client_response, connection_preamble, message};

// ConnectionPreamble

pub fn server_connection_preamble(id: ServerId, addr: &SocketAddr) -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        let mut server = message.init_root::<connection_preamble::Builder>()
                                .init_id()
                                .init_server();
        server.set_addr(&format!("{}", addr));
        server.set_id(id.as_u64());
    }
    Rc::new(message)
}

pub fn client_connection_preamble(id: ClientId) -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        message.init_root::<connection_preamble::Builder>()
               .init_id()
               .set_client(id.as_bytes());
    }
    Rc::new(message)
}

// AppendEntries

pub fn append_entries_request(term: Term,
                              prev_log_index: LogIndex,
                              prev_log_term: Term,
                              entries: &[(Term, &[u8])],
                              leader_commit: LogIndex)
                              -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        let mut request = message.init_root::<message::Builder>()
                                 .init_append_entries_request();
        request.set_term(term.as_u64());
        request.set_prev_log_index(prev_log_index.as_u64());
        request.set_prev_log_term(prev_log_term.as_u64());
        request.set_leader_commit(leader_commit.as_u64());

        let mut entry_list = request.init_entries(entries.len() as u32);
        for (n, entry) in entries.iter().enumerate() {
            let mut slot = entry_list.borrow().get(n as u32);
            slot.set_term(entry.0.into());
            slot.set_data(entry.1);
        }
    }
    Rc::new(message)
}

pub fn append_entries_response_success(term: Term,
                                       log_index: LogIndex)
                                       -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        let mut response = message.init_root::<message::Builder>()
                                  .init_append_entries_response();
        response.set_term(term.as_u64());
        response.set_success(log_index.as_u64());
    }
    Rc::new(message)
}

pub fn append_entries_response_stale_term(term: Term) -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        let mut response = message.init_root::<message::Builder>()
                                  .init_append_entries_response();
        response.set_term(term.as_u64());
        response.set_stale_term(());
    }
    Rc::new(message)
}

pub fn append_entries_response_inconsistent_prev_entry(term: Term,
                                                       index: LogIndex)
                                                       -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        let mut response = message.init_root::<message::Builder>()
                                  .init_append_entries_response();
        response.set_term(term.as_u64());
        response.set_inconsistent_prev_entry(index.into());
    }
    Rc::new(message)
}

pub fn append_entries_response_internal_error(term: Term,
                                              error: &str)
                                              -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        let mut response = message.init_root::<message::Builder>()
                                  .init_append_entries_response();
        response.set_term(term.as_u64());
        response.set_internal_error(error);
    }
    Rc::new(message)
}

// RequestVote

pub fn request_vote_request(term: Term,
                            last_log_index: LogIndex,
                            last_log_term: Term)
                            -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        let mut request = message.init_root::<message::Builder>()
                                 .init_request_vote_request();
        request.set_term(term.as_u64());
        request.set_last_log_index(last_log_index.as_u64());
        request.set_last_log_term(last_log_term.as_u64());
    }
    Rc::new(message)
}

pub fn request_vote_response_granted(term: Term) -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        let mut response = message.init_root::<message::Builder>()
                                  .init_request_vote_response();
        response.set_term(term.as_u64());
        response.set_granted(());
    }
    Rc::new(message)
}

pub fn request_vote_response_stale_term(term: Term) -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        let mut response = message.init_root::<message::Builder>()
                                  .init_request_vote_response();
        response.set_term(term.as_u64());
        response.set_stale_term(());
    }
    Rc::new(message)
}

pub fn request_vote_response_already_voted(term: Term) -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        let mut response = message.init_root::<message::Builder>()
                                  .init_request_vote_response();
        response.set_term(term.as_u64());
        response.set_already_voted(());
    }
    Rc::new(message)
}

pub fn request_vote_response_inconsistent_log(term: Term) -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        let mut response = message.init_root::<message::Builder>()
                                  .init_request_vote_response();
        response.set_term(term.as_u64());
        response.set_inconsistent_log(());
    }
    Rc::new(message)
}

pub fn request_vote_response_internal_error(term: Term, error: &str) -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        let mut response = message.init_root::<message::Builder>()
                                  .init_request_vote_response();
        response.set_term(term.as_u64());
        response.set_internal_error(error);
    }
    Rc::new(message)
}

// Ping

pub fn ping_request() -> Builder<HeapAllocator> {
    let mut message = Builder::new_default();
    {
        message.init_root::<client_request::Builder>()
               .init_ping();
    }
    message
}

// Query

pub fn query_request(entry: &[u8]) -> Builder<HeapAllocator> {
    let mut message = Builder::new_default();
    {
        message.init_root::<client_request::Builder>()
               .init_query()
               .set_query(entry);
    }
    message
}


// Proposal

pub fn proposal_request(entry: &[u8]) -> Builder<HeapAllocator> {
    let mut message = Builder::new_default();
    {
        message.init_root::<client_request::Builder>()
               .init_proposal()
               .set_entry(entry);
    }
    message
}

// Query / Proposal Response

pub fn command_response_success(data: &[u8]) -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        message.init_root::<client_response::Builder>()
               .init_proposal()
               .set_success(data);
    }
    Rc::new(message)
}

pub fn command_response_unknown_leader() -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        message.init_root::<client_response::Builder>()
               .init_proposal()
               .set_unknown_leader(());
    }
    Rc::new(message)
}

pub fn command_response_not_leader(leader_hint: &SocketAddr) -> Rc<Builder<HeapAllocator>> {
    let mut message = Builder::new_default();
    {
        message.init_root::<client_response::Builder>()
               .init_proposal()
               .set_not_leader(&format!("{}", leader_hint));
    }
    Rc::new(message)
}
