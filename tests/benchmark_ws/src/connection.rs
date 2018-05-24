// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::sync::mpsc;
use std::sync::Arc;
use util::RwLock;
use worker::*;
use ws::{CloseCode, Factory, Handler, Handshake, Message, Result, Sender};

pub struct Connection {
    pub sender: Sender,
    pub tx: mpsc::Sender<Message>,
}

impl Handler for Connection {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        if let Some(addr) = shake.remote_addr()? {
            println!("Connection with {} now open", addr);
        }
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        let _ = self.tx.send(msg);
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Connection closing due to ({:?}) {}", code, reason);
    }
}

#[derive(Clone)]
pub struct FactoryConnection {
    ws_senders: Arc<RwLock<Vec<Sender>>>,
    tx: mpsc::Sender<Message>,
}

impl FactoryConnection {
    pub fn new(
        ws_senders: Arc<RwLock<Vec<Sender>>>,
        tx: mpsc::Sender<Message>,
    ) -> FactoryConnection {
        FactoryConnection {
            ws_senders: ws_senders,
            tx: tx,
        }
    }
}

impl Factory for FactoryConnection {
    type Handler = Connection;

    fn connection_made(&mut self, out: Sender) -> Connection {
        self.ws_senders.write().push(out.clone());
        Connection {
            sender: out,
            tx: self.tx.clone(),
        }
    }
}
