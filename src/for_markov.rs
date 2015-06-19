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
*/
fn vec_strings(file: &str) -> Vec<String> {
    /*
        Try to open a file, `panic!` on error
    */
    let mut f: File = File::open(file).unwrap();

    /*
        Initialize String to which initially whole file will be put into
    */
    let mut string = String::new();

    /*
        Read the whole file into a single string, `panic!` on error
    */
    let _ = f.read_to_string(&mut string).unwrap();

    /*
        Initialize vector to be filled with strings and returned
    */
    let mut vec = Vec::new();

    /*
        Use iterator to push each line into vec
    */
    for l in string.lines() {
        vec.push(l.to_string());
    }

    vec
}

/*
    Feed markov with stuff
*/
pub fn feed_markov(chain: &mut Chain<String>) {
    /*
        let iteration begin
    */
    // TODO: currently textfile is hardcoded, should be changed to made it
    //       try to use setting first, and then fallback on hardcoded.
    //       In a case where there would be no config & file, markove should
    //       be fed with an empty string.
    let vec_of_strings: Vec<String> = vec_strings("markov.txt");

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
