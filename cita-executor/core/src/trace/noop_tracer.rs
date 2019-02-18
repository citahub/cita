// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// This software is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This software is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Nonoperative tracer.

use cita_types::{Address, U256};
use evm::action_params::ActionParams;
use trace::trace::{Call, Create, VMTrace};
use trace::{FlatTrace, TraceError, Tracer, VMTracer};
use util::Bytes;

/// Nonoperative tracer. Does not trace anything.
pub struct NoopTracer;

impl Tracer for NoopTracer {
    fn prepare_trace_call(&self, _: &ActionParams) -> Option<Call> {
        None
    }

    fn prepare_trace_create(&self, _: &ActionParams) -> Option<Create> {
        None
    }

    fn prepare_trace_output(&self) -> Option<Bytes> {
        None
    }

    fn trace_call(
        &mut self,
        call: Option<Call>,
        _: U256,
        output: Option<Bytes>,
        _: Vec<FlatTrace>,
    ) {
        assert!(
            call.is_none(),
            "self.prepare_trace_call().is_none(): so we can't be tracing: qed"
        );
        assert!(
            output.is_none(),
            "self.prepare_trace_output().is_none(): so we can't be tracing: qed"
        );
    }

    fn trace_create(
        &mut self,
        create: Option<Create>,
        _: U256,
        code: Option<Bytes>,
        _: Address,
        _: Vec<FlatTrace>,
    ) {
        assert!(
            create.is_none(),
            "self.prepare_trace_create().is_none(): so we can't be tracing: qed"
        );
        assert!(
            code.is_none(),
            "self.prepare_trace_output().is_none(): so we can't be tracing: qed"
        );
    }

    fn trace_failed_call(&mut self, call: Option<Call>, _: Vec<FlatTrace>, _: TraceError) {
        assert!(
            call.is_none(),
            "self.prepare_trace_call().is_none(): so we can't be tracing: qed"
        );
    }

    fn trace_failed_create(&mut self, create: Option<Create>, _: Vec<FlatTrace>, _: TraceError) {
        assert!(
            create.is_none(),
            "self.prepare_trace_create().is_none(): so we can't be tracing: qed"
        );
    }

    fn trace_suicide(&mut self, _address: Address, _balance: U256, _refund_address: Address) {}

    fn subtracer(&self) -> Self {
        NoopTracer
    }

    fn traces(self) -> Vec<FlatTrace> {
        vec![]
    }
}

/// Nonoperative VM tracer. Does not trace anything.
pub struct NoopVMTracer;

impl VMTracer for NoopVMTracer {
    /// Trace the preparation to execute a single instruction.
    fn trace_prepare_execute(&mut self, _pc: usize, _instruction: u8, _gas_cost: &U256) -> bool {
        false
    }

    /// Trace the finalised execution of a single instruction.
    fn trace_executed(
        &mut self,
        _gas_used: U256,
        _stack_push: &[U256],
        _mem_diff: Option<(usize, &[u8])>,
        _store_diff: Option<(U256, U256)>,
    ) {
    }

    /// Spawn subtracer which will be used to trace deeper levels of execution.
    fn prepare_subtrace(&self, _code: &[u8]) -> Self {
        NoopVMTracer
    }

    /// Spawn subtracer which will be used to trace deeper levels of execution.
    fn done_subtrace(&mut self, _sub: Self) {}

    /// Consumes self and returns all VM traces.
    fn drain(self) -> Option<VMTrace> {
        None
    }
}
