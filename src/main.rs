#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::collections::HashSet;
use std::collections::HashMap;
use std::io;

use rocket_contrib::json::{Json, JsonValue};

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

mod finite_automaton;
//mod reg_exp;
//mod cfg;
//mod pda;
//mod tm;

#[post("/api", format = "json", data = "<input_automaton>")]
fn api(input_automaton : Json<finite_automaton::FiniteAutomatonJson>) 
		 -> JsonValue {
	let (mut test_dfa, input_string) =
		finite_automaton::FiniteAutomaton::new_from_json(&input_automaton);

	let return_path = test_dfa.validate_string(input_string, true);
	
	json!(return_path)
}

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("client/build/index.html")
}

#[get("/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("client/build/").join(file)).ok()
}

fn main() {
	rocket::ignite()
		.mount("/", routes![api, index, file])
		.launch();
}
