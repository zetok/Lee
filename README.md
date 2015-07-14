This is a markov bot for Tox groupchats. It's the best `;)`

More features may come along the way `:)`

# Features

* support for text groupchats
* support for 1-on-1 chats
* markov chain
* saving & loading Tox data file & markov chain
* ..and lots of other features `;)`


# Installation
Installation is fairly simple. This bot will only work on Linux. 

1. Install [Rust](http://www.rust-lang.org/)
2. Make with `cargo build`
3. Run with `./target/debug/./lee`

# Usage

If there is no chain (``markov.json``) available, then strings can be loaded from a file ``markov.txt``. For time being, presence of ``markov.txt`` in working dir is mandatory.

There custom formatting of file is used.<br/>
Lines starting with ``//`` or ``/*`` are ignored.<br/>
Lines ending with ``\\`` are joined with next line.<br/>
Last line in a file should not be empty.<br/>


Currently supported by Lee commands are:

## Groupchat commands

| Command | What it does |
|---------|--------------|
| .about  | Send "About" message         |
| .id     | Lee will say its ID          |
| .talk   | Turns on talking on its own  |
| .stahp  | Turns off talking on its own |

## Friend commands

| Command | What it does |
|---------|--------------|
| invite  | Invites to last groupchat in which someone spoke |


By default Lee talks on its own at random times, few times per hour.

Even after turning off random talk on its own, Lee will respond to `highlighting`, i.e. to someone mentioning its name.


# License

Licensed under GPLv3+, for details see [COPYING](/COPYING).