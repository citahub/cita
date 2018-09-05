# Executor microservice core for CITA

Files below are extracted from [Parity](https://github.com/paritytech/parity):

- ./src/cache_manager.rs
- ./src/substate.rs
- ./src/snapshot/mod.rs
- ./src/snapshot/io.rs
- ./src/snapshot/account.rs
- ./src/snapshot/error.rs
- ./src/snapshot/service.rs
- ./src/account_db.rs
- ./src/error.rs
- ./src/db.rs
- ./src/basic_types.rs
- ./src/libexecutor/cache.rs
- ./src/trace/mod.rs
- ./src/trace/executive_tracer.rs
- ./src/trace/error.rs
- ./src/trace/noop_tracer.rs
- ./src/trace/db.rs
- ./src/trace/types/mod.rs
- ./src/trace/types/localized.rs
- ./src/trace/types/flat.rs
- ./src/trace/types/error.rs
- ./src/trace/types/filter.rs
- ./src/trace/types/trace.rs
- ./src/trace/import.rs
- ./src/trace/config.rs
- ./src/state/mod.rs
- ./src/state/backend.rs
- ./src/externalities.rs
- ./src/state_db.rs
- ./src/pod_account.rs
- ./src/factory.rs

with following modifications:

- ./src/builtin.rs
- ./src/executed.rs
- ./src/engines/null_engine.rs
- ./src/executive.rs
- ./src/state/account.rs
- ./src/spec/builtin.rs
