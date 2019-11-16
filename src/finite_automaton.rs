use std::collections::*;

#[derive(Debug)]
pub struct FiniteAutomaton {
	// automata are defined as a 5 tuple of states, alphabet, transition function,
	// final, and start state
	alphabet : HashSet<char>,
	start_state : String,
	// states are defined as a map from strings to bools, which determine if
	// they are accepting states
    states : HashMap<String, bool>,
	// transition function is a hashMap from the current state, to a hashmap
	// representing all of the transitions for the current state
	// the transition of a current state is a letter and a next state
	transition_function : HashMap<(String, Option<char>), String>,
	determinism : bool
}

impl FiniteAutomaton {
	pub fn new(a_alphabet : HashSet<char>,
			   a_start_state : String, a_new_states : HashMap<String, bool>,
			   a_transitions : HashMap<(String, Option<char>), String>)
			   -> FiniteAutomaton {
		FiniteAutomaton { alphabet : a_alphabet, start_state : a_start_state,
						  states : a_new_states, transition_function : a_transitions,
						  determinism : true }
	}

	pub fn insert_char_in_alphabet(&mut self, new_char : char) {
		self.alphabet.insert(new_char);
	}
	
	pub fn insert_empty_state(&mut self, state_name : String, is_final : bool) -> bool {
		if self.states.get(&state_name.to_string()) == None {
			println!("Inserting New State: {:?}: ",
					 self.states.insert(state_name, is_final));
			true
		}
		else {
			println!("State already exists, call delete() before replacing");
			false
		}
	}

	fn insert_transition(&mut self, state_name : &String, new_transition : &(Option<char>, String)) {
		// checks to see if the read symbol is either an epsilon or inside
		// of the alphabet if not in the alphabet rejects that specific
		// insertion

		// if an empty symbol/an epsilon is ever inserted, we immediately
		// lose determinism

		// the first bool shows if the new state should be inserted, the
		// second if it is deterministic.

		// should also check to see that the end of the transition string exists
		// as well
		let should_insert : (bool, bool) = match &new_transition.0 {
			Some(symbol) => (! (self.alphabet.contains(&symbol) ||
								self.states.contains_key(&(new_transition.1).to_string())), true),
			None => (true, false)
		};

		if should_insert.0 {
			self.determinism = self.determinism && should_insert.1;
			self.transition_function.insert(((&state_name).to_string(), new_transition.0),
											(&new_transition.1).to_string());
		}
		else {
			println!("Tried to incorrectly insert {:?}", state_name);
			println!("{:?} is either a copy of an existing value, or delete failed", state_name);
		}		
	}
	
	pub fn insert_new_state(&mut self, state_name : &String, is_final : bool, 
							transitions : HashSet<(Option<char>, String)>) {
		if self.insert_empty_state(state_name.to_string(), is_final) {
			for i in transitions.iter() {
				self.insert_transition(state_name, i);
			}
		}
	}

	/*pub fn delete_state(&mut self, state_name : String) -> () {
		// start by checking to see that the state exists
		
		// delete state from the states HashSet
		
		// find and delete all transitions to and from the state in the
		// transitions HashMap
		
	}

	fn change_start(&mut self) -> () {

	}
	
	fn serialize_xml() -> () { }

	fn deserialize_xml() -> () { }

	fn serialize_json() -> () { }

	fn deserialize_json() -> () { }
	
	fn check_determinism() -> () { }

	fn validate_string() -> () { }

	fn trace_string() -> () { }

	fn generate_tests() -> () { }*/
	
}
