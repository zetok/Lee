/*
    Copyright © 2015 Zetok Zalbavar <zetok@openmailbox.org>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/



/*
    For getting stuff from files
*/
use std::io::prelude::*;
use std::fs::File;


/*
    For markov chain
*/
extern crate markov;
use markov::Chain;


/*
    Function to read file and return vector of strings, each of them
    corresponding to a line from a file.

    In a case where there is no file, return early.
*/
fn vec_strings(file: &str) -> Result<Vec<String>, ()> {
    let mut file = match File::open(file) {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening {}: {}", file, e);
            return Err(())
        },
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    Ok(content.lines().map(|l| l.to_string()).collect())
}

/**
    Feed markov chain with strings from a file.

    In a case where file can't be used, an empty chain will be returned.
*/
pub fn feed_markov(chain: &mut Chain<String>) {
    /*
        Get vector of strings from the file `markov.txt`. In a case where
        there is no file supplied, return early without feeding chain.
    */
    let vec_of_strings: Vec<String> = match vec_strings("markov.txt") {
        Ok(v) => v,
        Err(_) => return,
    };

    /*
        Initialize string to feed markov
    */
    let mut string: String = String::new();

    for line in vec_of_strings {
        if line.starts_with("//") || line.starts_with("/*") || line == "" {
            // just skip this line
            continue;
        } else if line.ends_with("\\") {
            // push it to string
            string.push_str(&line);
            // check for length after size increased, and before it gets
            // truncated ↓
            let string_len = string.len();

            // truncate not needed `\\`
            string.truncate(string_len - 2);

            // since \n is not treated as char, and cant be `.push(char)`
            // to string.. make it a &str, and `.push_str()` it
            string.push_str("\n");
        } else {
            // ↓ push to string
            string.push_str(&line);

            // feed markov with it
            chain.feed_str(&string);

            // ...aaaand clear string
            string.clear();
        }
    }
}


/**
    Function to make chain - either load it from a file, or, if that will
    fail for some reason, make an empty chain and feed it with contents of
    plaintext file.
*/
pub fn make_chain(file: &str) -> Chain<String> {
    match Chain::load_utf8(file) {
        Ok(data) => data,
        Err(e) => {
            println!("Error loading `{}`: {}", file, e);
            let mut chain = Chain::for_strings();
            // try to feed it from a plaintext file
            feed_markov(&mut chain);
            chain
        },
    }
}


/**
    Function to load save file from `save.tox` file.

    In case where it can't be opened, return an error, so that it could
    be printed, and Tox instance could be initialized without it.
*/
pub fn load_save(f: &str) -> Result<Vec<u8>, String> {
    match File::open(f) {
        Ok(mut file) => {
            let mut res: Vec<u8> = Vec::new();
            drop(file.read_to_end(&mut res));
            Ok(res)
        },

        Err(e) => {
            Err(format!("{}", e))
        },
    }
}


/**
    Function to write save file to storage.

    In case where it can't be written to, return an error, so that it could
    be printed.
*/
pub fn write_save(f: &str, data: Vec<u8>) -> Result<(), String> {
    match File::create(f) {
        Ok(mut file) => {
            drop(file.write(&data));
            Ok(())
        },

        Err(e) => {
            Err(format!("{}", e))
        },
    }
}
