#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate strata;
extern crate strata_rs;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

extern crate rocket;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use strata::{Algebra, ValidExtent};

use rocket::response::content;

lazy_static! {
    static ref SOURCE: String = {
        let mut f = File::open("src/lib.rs").expect("no open file");
        let mut s = String::new();
        f.read_to_string(&mut s).expect("no read");
        s
    };
    static ref DONE: Done = {
        let f = File::open("self.json").expect("no open file");
        serde_json::de::from_reader(f).expect("no parse")
    };
    static ref INDEX: HashMap<String, Vec<ValidExtent>> = {
        let mut index = HashMap::new();
        for ex in &DONE.idents {
            let s = SOURCE[(ex.0 as usize)..(ex.1 as usize)].to_owned();
            index.entry(s).or_insert_with(Vec::new).push(ex.clone());
        }
        index
    };
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Done {
    functions: Vec<strata::ValidExtent>,
    idents: Vec<strata::ValidExtent>, // mismatched type!
}

#[get("/<id>")]
fn index(id: &str) -> content::Plain<String> {
    let key = INDEX.get(id).map_or(&[][..], Vec::as_slice);

    let query = strata::Containing::new(DONE.functions.as_slice(), key);

    let mut s = String::new();
    for x in query.iter_tau() {
        s.push_str(&SOURCE[(x.0 as usize)..(x.1 as usize)]);
    }

    // DONE.functions
    // let ident = DONE.idents[id];
    // let ident = &SOURCE[ident.0..ident.1];



    content::Plain(s)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}