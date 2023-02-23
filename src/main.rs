#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};


fn remove_ngram_dups(s: &str, n: usize, term: Option<usize>) -> String {
    let max_dups = if n < 3 { 3 } else { 1 };
    let s = s.chars().collect::<Vec<char>>();

    // largest N-gram duplicate possible
    let term = term.unwrap_or_else(|| s.len() / 2);

    // terminal size
    if (n == 0 || n > term) && !s.is_empty() {
        return s.iter().collect();
    }

    let mut result = Vec::new();
    for i in (0..s.len()).step_by(n) {
        if i + (max_dups + 1) * n - 1 >= s.len() {
            result.extend(&s[i..]);
            break;
        }
        if (2..=max_dups + 1).all(|k| &s[i..i + n] == &s[i + (k - 1) * n..i + k * n]) {
            // duplicate found
        } else {
            result.extend(&s[i..i + n]);
        }
    }

    if s != result {
        // backtrack to n/2 gram if pattern is found
        let new_n = n / 2;
        let new_term = Some(term);
        remove_ngram_dups(&result.iter().collect::<String>(), new_n, new_term)
    } else {
        let new_n = n + 1;
        let new_term = Some(term);
        remove_ngram_dups(&result.iter().collect::<String>(), new_n, new_term)
    }
}

fn remove_dups(s: &str) -> String {
    let s1 = remove_ngram_dups(&format!(" {}", s), 1, None);
    let s2 = remove_ngram_dups(&format!("{} ", s), 1, None);
    if s1.len() < s2.len() {
        return s1;
    }
    s2
}

#[derive(Serialize, Deserialize)]
pub struct Output{
    pub result: String
}

#[get("/remove-dups?<q>")]
fn remove_dups_handler(q: &str) -> Json<Output> {
    let result = remove_dups(q);

    Json(Output{result})
}


#[rocket::main]
async fn main() {
   if let Err(err) = rocket::build()
      .mount("/", rocket::routes![remove_dups_handler])
      .launch()
      .await
   {
      println!("Rocket Rust couldn't take off successfully!");
      drop(err); // Drop initiates Rocket-formatted panic
   }
}

/*
use std::io::{self, BufRead};

fn main() {
	println!("Type any string");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            break;
        }
        let result = remove_dups(&line);
        println!("{}", result);
    }
}
*/
