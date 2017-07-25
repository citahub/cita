use std::fmt;
use std::net::SocketAddr;
use std::rc::Rc;

use mio::tcp::TcpStream;
use mio::Timeout as TimeoutHandle;
use mio::{EventLoop, EventSet, PollOpt, Token};
use capnp::message::{Builder, HeapAllocator, Reader, ReaderOptions};
use capnp_nonblock::{MessageStream, Segments};

use ClientId;
use Result;
use ServerId;
use backoff::Backoff;
use messages;
use server::{Server, ServerTimeout};
use state_machine::StateMachine;
use persistent_log::Log;

fn poll_opt() -> PollOpt {
    PollOpt::edge() | PollOpt::oneshot()
}

/// The type of a connection.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ConnectionKind {
    /// A peer in the cluster.
    Peer(ServerId),
    /// A client which is asking the Raft cluster to do things.
    Client(ClientId),
    /// Something else.
    Unknown,
}

impl ConnectionKind {
    /// Returns if the `Connection` is a peer type.
    fn is_peer(&self) -> bool {
        match *self {
            ConnectionKind::Peer(..) => true,
            _ => false,
        }
    }
}

pub struct Connection {
    kind: ConnectionKind,
    /// The address to reconnect to - for a connection initiated by the remote,
    /// this is not the remote address.
    addr: SocketAddr,
    stream: Option<MessageStream<TcpStream, HeapAllocator, Rc<Builder<HeapAllocator>>>>,
    backoff: Backoff,
}

impl Connection {
    /// Creates a new `Connection` wrapping the provided socket stream.
    ///
    /// The socket must already be connected.
    ///
    /// Note: the caller must manually set the token field after inserting the
    /// connection into a slab.
    pub fn unknown(socket: TcpStream) -> Result<Connection> {
        let addr = try!(socket.peer_addr());
        Ok(Connection {
            kind: ConnectionKind::Unknown,
            addr: addr,
            stream: Some(MessageStream::new(socket, ReaderOptions::new())),
            backoff: Backoff::with_duration_range(50, 10000),
        })
    }

    /// Creates a new peer connection.
    pub fn peer(id: ServerId, addr: SocketAddr) -> Result<Connection> {
        let stream = try!(TcpStream::connect(&addr));
        Ok(Connection {
            kind: ConnectionKind::Peer(id),
            addr: addr,
            stream: Some(MessageStream::new(stream, ReaderOptions::new())),
            backoff: Backoff::with_duration_range(50, 10000),
        })
    }

    pub fn kind(&self) -> &ConnectionKind {
        &self.kind
    }

    pub fn set_kind(&mut self, kind: ConnectionKind) {
        self.kind = kind;
    }

    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }

    pub fn set_addr(&mut self, addr: SocketAddr) {
        self.addr = addr;
    }

    /// Returns the connection's stream.
    /// Must only be called while the connection is active.
    fn stream(&self) -> &MessageStream<TcpStream, HeapAllocator, Rc<Builder<HeapAllocator>>> {
        match self.stream {
            Some(ref stream) => stream,
            None => panic!(format!("{:?}: not connected", self)),
        }
    }

    /// Returns the connection's mutable stream.
    /// Must only be called while the connection is active.
    fn stream_mut(&mut self)
                  -> &mut MessageStream<TcpStream, HeapAllocator, Rc<Builder<HeapAllocator>>> {
        match self.stream {
            Some(ref mut stream) => stream,
            None => panic!(format!("{:?}: not connected", self)),
        }
    }

    /// Writes queued messages to the socket.
    pub fn writable(&mut self) -> Result<()> {
        scoped_trace!("{:?}: writable", self);
        if let Connection { stream: Some(ref mut stream), ref mut backoff, .. } = *self {
            try!(stream.write());
            backoff.reset();
            Ok(())
        } else {
            panic!("{:?}: writable event while not connected", self);
        }
    }

    /// Reads a message from the connection's stream, or if a full message is
    /// not available, returns `None`.
    ///
    /// Connections are edge-triggered, so the handler must continue calling
    /// until no more messages are returned.
    pub fn readable(&mut self) -> Result<Option<Reader<Segments>>> {
        scoped_trace!("{:?}: readable", self);
        self.stream_mut().read_message().map_err(From::from)
    }

    /// Queues a message to send to the connection. Returns `true` if the connection should be
    /// reregistered with the event loop.
    pub fn send_message(&mut self, message: Rc<Builder<HeapAllocator>>) -> Result<bool> {
        scoped_trace!("{:?}: send_message", self);
        match self.stream {
            Some(ref mut stream) => {
                // Reregister if the connection is not already registered, and
                // there are still messages left to send. MessageStream
                // optimistically sends messages, so it's likely that small
                // messages can be sent without ever registering.
                let unregistered = stream.outbound_queue_len() == 0;
                try!(stream.write_message(message));
                Ok(unregistered && stream.outbound_queue_len() > 0)
            }
            None => Ok(false),
        }
    }

    fn events(&self) -> EventSet {
        let mut events = EventSet::all();
        if self.stream().outbound_queue_len() == 0 {
            events = events - EventSet::writable();
        }
        events
    }

    /// Registers the connection with the event loop.
    pub fn register<L, M>(&mut self,
                          event_loop: &mut EventLoop<Server<L, M>>,
                          token: Token)
                          -> Result<()>
        where L: Log,
              M: StateMachine
    {
        scoped_trace!("{:?}: register", self);
        event_loop.register(self.stream().inner(), token, self.events(), poll_opt())
                  .map_err(|error| {
                      scoped_warn!("{:?}: reregister failed: {}", self, error);
                      From::from(error)
                  })
    }

    /// Reregisters the connection with the event loop.
    pub fn reregister<L, M>(&mut self,
                            event_loop: &mut EventLoop<Server<L, M>>,
                            token: Token)
                            -> Result<()>
        where L: Log,
              M: StateMachine
    {
        scoped_trace!("{:?}: reregister", self);
        event_loop.reregister(self.stream().inner(), token, self.events(), poll_opt())
                  .map_err(|error| {
                      scoped_warn!("{:?}: register failed: {}", self, error);
                      From::from(error)
                  })
    }

    /// Reconnects to the given peer ID and sends the preamble, advertising the
    /// given local address to the peer.
    pub fn reconnect_peer(&mut self, id: ServerId, local_addr: &SocketAddr) -> Result<()> {
        scoped_assert!(self.kind.is_peer());
        scoped_trace!("{:?}: reconnect", self);
        self.stream = Some(MessageStream::new(try!(TcpStream::connect(&self.addr)),
                                              ReaderOptions::new()));
        try!(self.send_message(messages::server_connection_preamble(id, local_addr)));
        Ok(())
    }

    /// Resets a peer connection.
    pub fn reset_peer<L, M>(&mut self,
                            event_loop: &mut EventLoop<Server<L, M>>,
                            token: Token)
                            -> Result<(ServerTimeout, TimeoutHandle)>
        where L: Log,
              M: StateMachine
    {
        scoped_assert!(self.kind.is_peer());
        self.stream = None;
        let duration = self.backoff.next_backoff_ms();
        let timeout = ServerTimeout::Reconnect(token);
        let handle = event_loop.timeout_ms(timeout, duration).unwrap();

        scoped_info!("{:?}: reset, will attempt to reconnect in {}ms",
                     self,
                     duration);
        Ok((timeout, handle))
    }

    pub fn clear_messages(&mut self) {
        if let Some(ref mut stream) = self.stream {
            stream.clear_outbound_queue();
        }
    }
}

impl fmt::Debug for Connection {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ConnectionKind::Peer(id) => write!(fmt, "PeerConnection({})", id),
            ConnectionKind::Client(id) => write!(fmt, "ClientConnection({})", id),
            ConnectionKind::Unknown => write!(fmt, "UnknownConnection({})", &self.addr),
        }
    }
}
