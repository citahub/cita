// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

//! GRPC contracts.
//! You need following steps to use GRPC Contract:
//! 1. Register a contract info through Executor GRPC interface.
//! 2. Enable this GRPC contract by send a transaction (See block.rs file).
//!
//! now you can invoke this GRPC contract from transaction or EVM contract.

pub mod contract;
pub mod contract_state;
pub mod grpc_vm;
pub mod grpc_vm_adapter;
pub mod service_registry;
pub mod storage;
