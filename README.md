This is a bot for Tox groupchats. It currently does very little, more functionality to come `:)`


# Features

* support for text groupchats
* support for 1-on-1 chats
* markov chain
* ..and lots of other features `;)`


# Installation
Installation is fairly simple. This bot will only work on Linux. 

1. Install [Rust](http://www.rust-lang.org/)
2. Make with `cargo build`
3. Run with `./target/debug/./lee`

# Usage

Currently Lee requires file `markov.txt` to be available in working directory. Preferably with some content.

Currently supported by Lee commands are:

| Command | What it does |
|---------|--------------|
| .stahp  | Turns off talking on its own |
| .talk   | Turns on talking on its own  |
| .id     | Lee will say its ID          |


By default Lee talks on its own at random times, few times per hour.

Even after turning off random talk on its own, Lee will respond to `highlighting`, i.e. to someone mentioning its name.


# License

Licensed under GPLv3+, for details see [COPYING](/COPYING).