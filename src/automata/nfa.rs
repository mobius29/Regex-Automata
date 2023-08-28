pub struct NFA {
  accept_states: Vec<usize>,
  initial_state: Option<State>,
}

#[derive(Debug, Clone)]
struct State<T> {
  state_number: usize,
  value: T,
}