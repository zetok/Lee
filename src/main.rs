/*
    Copyright (C) 2015 subliun <subliunisdev@gmail.com>
    Copyright © 2015 Zetok Zalbavar <zetok@openmailbox.org>
    All Rights Reserved.

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
    Binding to toxcore
*/
extern crate rstox;
use rstox::core::*;


/*
    For markov chain
*/
extern crate markov;
use markov::Chain;

/*
    For extra fun with markov chain
*/
extern crate chrono;
use chrono::UTC;

extern crate rand;
use rand::ThreadRng;
use rand::Rng;


/*
    Lee's own stuff
*/
// TODO: when other functions will be moved from main.rs, things should be
//       added here
mod bootstrap;
mod for_files;



/*
    For bot functionality
*/
//#[derive(Debug)]   // can't be used, since `rand` doesn't want to cooperate
struct Bot {
    markov: Chain<String>,

    /**
        Time since last save.
    */
    last_save: i64,

    /**
        Option to decide whether title should be changed, by default `false`.

        Currently this value can be altered by hardcoded bot owner,
        recognised by unique, hardcoded nick. !! This behaviour should be
        altered, to have public-key based owner recognition, in addition to
        loading public key from config file.
    */
    title_pin: bool,

    /**
        Lee's own public key
    */
    pk: PublicKey,

    /**
        Last group from which message of any kind was received.

        This value is being used to decide in which groupchat Lee should
        speak randomly – since out of all groupchats this was the last one
        in which activity was observed, it is most likely that there are
        some people in it able to receive Lee's message.
    */
    last_group: i32,

    /**
        Time since Lee last spoken randomly.
    */
    last_time: i64,

    /**
        Option to allow Lee talk ar $random_interval, it does not affect Lee's
        response when triggered (highlighted).

        Can be altered by users using commands:
         - `.stahp` – will make Lee stop speaking randomly
         - `.talk`  – will make Lee resume speaking randomly

        Defalut value should be `true`.
    */
    speak: bool,

    /**
        `trigger` is used to launch Lee's talk when something will trigger
        it, by mentioning its name. Answer shouldn't be instantaneous, which
        will make Lee more human.

        By default should be `false`, and after countdown was down to 0, it
        should be restored to `false.
    */
    trigger: bool,

    /**
        Time when trigger happened, as UNIX time in i64.

        Seconds should be added to this value, so that time of Lee's response
        for trigger would be more human-like, rather than instantaneous.
    */
    trigger_time: i64,

    /**
        Cached RNG, apparently it helps with RNG's performance when it's used
        a lot.
    */
    random: ThreadRng,
}



// TODO: load it from config file, if not available, then use default one
//         * perhaps it could be made of some random chars generated
//           at runtime?
static BOT_NAME: &'static str = "Lee";
// TODO: make use of some fancy functions to not have that ↓ mess
static BOT_NAMES: &'static [&'static str] = &["Lee", "lee", "LEE",
                                        "LEe", "LeE", "lEE", "leE"];


/*
    Defend my honour. Needed to compare whether someone is not trying to
    use my nick.

    As extended measure, compares public key of peer.

    Also defend bot.
*/
const FAKE_NAMES: &'static [&'static str] = &["Zetok\0", "Zetok", "zetok",
                                          "Lee", "Lee\0"];



/*
    Function to deal with incoming friend requests

    Currently accepts all by default
*/
// TODO: make it configurable to accept all / only selected FRs
fn on_friend_request(tox: &mut Tox, fpk: PublicKey, msg: String) {
    drop(tox.add_friend_norequest(&fpk));
    println!("\nFriend {} with friend message {:?} was added.", fpk, msg);
}


/*
    Function to deal with friend messages

    Lee is supposed to answer all friend messages, in ~similar way to
    how it's done in groupchats.
*/
fn on_friend_message(tox: &mut Tox, fnum: u32, msg: String, bot: &mut Bot) {
    let pubkey = match tox.get_friend_public_key(fnum) {
        Some(pkey) => pkey,
        None       => bot.pk,
    };


    println!("\nEvent: FriendMessage:\nFriend {} sent message: {}", pubkey, &msg);

    /*
        feed Lee with message content, but only if peer PK doesn't match
        Lee's own PK

        Feeding Lee with what it threw up may not be a good idea after all..
    */
    if pubkey != bot.pk {
        bot.markov.feed_str(&msg);
    }

    let message = bot.markov.generate_str();
    println!("Answer: {}", &message);
    drop(tox.send_friend_message(fnum, MessageType::Normal, &message));
}



/*
    Function to deal with incoming invites to groupchats
*/
fn on_group_invite(tox: &mut Tox, fid: i32, kind: GroupchatType, data: Vec<u8>) {
    /*
        Since rstox currently supports only text groupchats, handle only them,
        and drop other invites.
    */
    match kind {
        GroupchatType::Text => {
            drop(tox.join_groupchat(fid, &data));
            println!("\nAccepted invite to text groupchat by {}.", fid);
        },
        GroupchatType::Av => {
            println!("\nDeclined invite to audio groupchat by {}.", fid);
        },
    }
}


/*
    Function to deal with group messages
*/
fn on_group_message(tox: &mut Tox, gnum: i32, pnum: i32, msg: String, bot: &mut Bot) {
    /*
        Get PK of the peer who sent message

        In case where toxcore doesn't feel like providing it, use own PK,
        to avoid triggering false alarm
    */
    let pubkey = match tox.group_peer_pubkey(gnum, pnum) {
        Some(pkey) => pkey,
        None       => bot.pk,
    };


    // mark this groupchat as last active one
    bot.last_group = gnum;


    /*
        Triggers Lee
    */
    fn trigger_response(msg: &String, bot: &mut Bot) {
        // TODO: find out whether there isn't some more efficient way of
        //       doing this
        for name in BOT_NAMES {
            if msg.contains(name) {
                bot.trigger = true;
                /*
                    ↓ waiting time for response should be random, for more
                    human-like feel, and should be at least 2s long – too
                    quick answer isn't too good either.

                    Currently waiting time should be between 1 and 5s.
                */
                let random_wait = 1.0 + 4.0 * bot.random.gen::<f64>();
                bot.trigger_time = random_wait as i64 + UTC::now().timestamp();
                break;
            }
        }
    }

    match tox.group_peername(gnum, pnum) {
        Some(pname) => {
            /*
                feed Lee with message content, but only if peer PK doesn't match
                Lee's own PK

                Feeding Lee with what it threw up may not be a good idea after
                all..
            */
            if pubkey != bot.pk && pname != "Layer" {
                bot.markov.feed_str(&msg);
            }


            if FAKE_NAMES.contains(&&*pname) {
                if pubkey != bot.pk &&
                    pubkey != "29AE62F95C56063D833024B1CB5C2140DC4AEB94A80FF4596CACC460D7BAA062".parse::<PublicKey>().unwrap() {
                    drop(tox.group_message_send(gnum, "↑ an impostor!"));
                }
            }


            trigger_response(&msg, bot);

            println!("\nEvent: GroupMessage({}, {}, {:?}), Name: {:?}, PK: {}",
                gnum, pnum, msg, pname, pubkey);
        },

        None => {
            trigger_response(&msg, bot);

            println!("Tox event: GroupMessage({}, {}, {:?}), Name: •not known•, PK: {}",
                gnum, pnum, msg, pubkey);
        },
    }

    /*
        Allow anyone to turn speaking on / off
    */
    if msg == ".stahp" {
        bot.speak = false;
    } else if msg == ".talk" {
        bot.speak = true;
    }

    /*
        Allow anyone to get Lee's ID
    */
    if msg == ".id" && pubkey != bot.pk {
        let message = format!("My ID: {}", tox.get_address());
        drop(tox.group_message_send(gnum, &message));
    }

}


fn main() {
    /*
        Try to load data file, if not possible, print an error and generate
        new Tox instance.
    */
    let data = match for_files::load_save("lee.tox") {
        Ok(d) => Some(d),
        Err(e) => {
            println!("\nError loading save: {}", e);
            None
        },
    };
    let mut tox = Tox::new(ToxOptions::new(), data.as_ref()
                                            .map(|x| &**x)).unwrap();

    drop(tox.set_name(BOT_NAME));

    /*
        Bot stuff
    */
    let mut bot = Bot {
        /*
            Try to load chain from a file, if not possible, then try to add
            strings to a chain from `markov.txt`. If even that is not
            available,, initialize an empty chain.
        */
        markov: {
            match Chain::load_utf8("markov.json") {
                Ok(data) => data,
                Err(e) => {
                    println!("Error loading `markov.json`: {}", e);
                    let mut chain = Chain::for_strings();
                    for_files::feed_markov(&mut chain);
                    chain
                },
            }
        },

        last_save: UTC::now().timestamp(),
        title_pin: false,
        pk: tox.get_public_key(),
        last_group: 0,
        last_time: UTC::now().timestamp(),
        speak: true,
        trigger: false,
        trigger_time: UTC::now().timestamp(),
        random: rand::thread_rng(),
    };


    /*
        Boostrapping process
        During bootstrapping one should query random bootstrap nodes from a
        supplied list; in case where there is no list, rely back on hardcoded
        bootstrap nodes.
        // TODO: actually make it possible to use supplied list; location of a
        //       list should be determined by value supplied in config file;
        //       in case of absence of config file, working dir should be
        //       tried for presence of file named `bootstrap.txt`, only if it
        //       is missing fall back on hardcoded nodes
    */
    bootstrap::bootstrap_hardcoded(&mut tox);

    println!("\nMy ID: {}", tox.get_address());
    println!("My name: {:?}", tox.get_name());

    loop {
        for ev in tox.iter() {
            match ev {
                FriendRequest(fpk, msg) => {
                    on_friend_request(&mut tox, fpk, msg);
                },

                FriendMessage(fnum, _, msg) => {
                    on_friend_message(&mut tox, fnum, msg, &mut bot);
                },

                GroupInvite(fid, kind, data) => {
                    on_group_invite(&mut tox, fid, kind, data);
                },

                GroupMessage(gnum, pnum, msg) => {
                    on_group_message(&mut tox, gnum, pnum, msg, &mut bot)
                },

                ev => { println!("\nTox event: {:?}", ev); },
            }
        }
        if bot.title_pin {
            drop(tox.group_set_title(0, "#tox-real-ontopic | so what triggers everyone?"));
        }


        /*
            Let Lee speak when triggered, provided that it will wait required
            amount of time.
        */
        if bot.trigger {
            let cur_time = UTC::now().timestamp();
            if cur_time >= bot.trigger_time {
                let message = bot.markov.generate_str();
                drop(tox.group_message_send(bot.last_group, &message));
                bot.trigger = false;
            }
        }


        /*
            Let Lee speak every $time_interval, provided that there is given
            permission for it
        */
        if bot.speak {
            let cur_time = UTC::now().timestamp();
            if  (bot.last_time + 10) < cur_time {
                /* Should have only small chance to speak */
                if 0.0161 > bot.random.gen::<f64>() {
                    let message = bot.markov.generate_str();
                    drop(tox.group_message_send(bot.last_group, &message));
                }

                bot.last_time = cur_time;
            }
        }


        /*
            Write save data every 64s.

            After a write, be it successful or not, set clock again to tick,
            for the next time when it'll need to be saved.
            TODO: save data every $relevant_event, rather than on timer.
        */
        let cur_time = UTC::now().timestamp();
        if bot.last_save + 64 < cur_time {
            match for_files::write_save("lee.tox", tox.save()) {
                Ok(_) => println!("\nFile saved."),
                Err(e) => println!("\nFailed to save file: {}", e),
            }
            drop(bot.markov.save_utf8("markov.json"));
            println!("Saved `markov.json`");
            bot.last_save = cur_time;
        }


        tox.wait();
    }
}
