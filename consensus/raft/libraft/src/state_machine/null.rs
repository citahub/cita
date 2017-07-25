use state_machine::StateMachine;

/// A state machine with no states.
#[derive(Debug)]
pub struct NullStateMachine;

impl StateMachine for NullStateMachine {
    fn apply(&mut self, _command: &[u8]) -> Vec<u8> {
        Vec::new()
    }

    fn query(&self, _query: &[u8]) -> Vec<u8> {
        Vec::new()
    }

    fn snapshot(&self) -> Vec<u8> {
        Vec::new()
    }

    fn restore_snapshot(&mut self, _snapshot: Vec<u8>) {
        ()
    }
}
