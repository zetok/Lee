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
mod for_markov;



/*
    For bot functionality
*/
//#[derive(Debug)]   // can't be used, since `rand` doesn't want to cooperate
struct Bot {
    markov: Chain<String>,
    title_pin: bool,
    pk: PublicKey,
    last_group: i32,
    last_time: i64,
    speak: bool,
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
    println!("Friend {} with friend message {:?} was added.", fpk, msg);
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
            println!("Accepted invite to text groupchat by {}.", fid);
        },
        GroupchatType::Av => {
            println!("Declined invite to audio groupchat by {}.", fid);
        },
    }
}


/*
    Function to deal with group messages
*/
fn on_group_message(tox: &mut Tox, gnum: i32, pnum: i32, msg: String, bot: &mut Bot) {
    // mark this groupchat as last active one
    bot.last_group = gnum;

    // feed Lee with message content
    bot.markov.feed_str(&msg);

    /*
        Triggers Lee
    */
    fn trigger_response(tox: &mut Tox, gnum: i32, msg: &String, markov: &Chain<String>) {
        // TODO: find out whether there isn't some more efficient way of
        //       doing this
        for name in BOT_NAMES {
            if msg.contains(name) {
                let message: String = markov.generate_str();
                drop(tox.group_message_send(gnum, &message));
                break;
            }
        }
    }

    /*
        Get PK of the peer who sent message

        In case where toxcore doesn't feel like providing it, use own PK,
        to avoid triggering false alarm
    */
    let pubkey = match tox.group_peer_pubkey(gnum, pnum) {
        Some(pkey) => pkey,
        None       => bot.pk,
    };

    match tox.group_peername(gnum, pnum) {
        Some(pname) => {
            if FAKE_NAMES.contains(&&*pname) {
                if pubkey != bot.pk &&
                    pubkey != "29AE62F95C56063D833024B1CB5C2140DC4AEB94A80FF4596CACC460D7BAA062".parse::<PublicKey>().unwrap() {
                    drop(tox.group_message_send(gnum, "↑ an impostor!"));
                }
            }

            if pname == "Zetok\0" {
                if msg == ".trigger" {
                    bot.title_pin = true;
                } else if msg == ".rmtrigger" {
                    bot.title_pin = false;
                }
            }


            trigger_response(tox, gnum, &msg, &bot.markov);

            println!("Tox event: GroupMessage({}, {}, {:?}), Name: {:?}", gnum, pnum, msg, pname);
        },

        None => {
            trigger_response(tox, gnum, &msg, &bot.markov);

            println!("Tox event: GroupMessage({}, {}, {:?}), Name: •not known•",
                gnum, pnum, msg);
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
}


/*
    Function to deal with groupchat name change
*/
fn on_group_name_list_change(tox: &mut Tox, gnum: i32, pnum: i32, change: ChatChange) {
    let msg = match change {
        ChatChange::PeerAdd => format!("Peer {} joined.", pnum),
        ChatChange::PeerDel => format!("Peer {} left.", pnum),
        ChatChange::PeerName => {
            match tox.group_peername(gnum, pnum) {
                Some(pname) => format!("Peer {} is now known as {}", pnum, pname),
                None => format!("Peer {} has unknown name!", pnum),
            }
        },
    };
    drop(tox.group_message_send(gnum, &msg));
}



fn main() {
    let mut tox = Tox::new(ToxOptions::new(), None).unwrap();

    drop(tox.set_name(BOT_NAME));


    /*
        Bot stuff
    */
    let mut bot = Bot {
        markov: Chain::for_strings(),
        title_pin: false,
        pk: tox.get_public_key(),
        last_group: 0,
        last_time: UTC::now().timestamp(),
        speak: true,
        random: rand::thread_rng(),
    };

    for_markov::feed_markov(&mut bot.markov);


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

                GroupInvite(fid, kind, data) => {
                    on_group_invite(&mut tox, fid, kind, data);
                },

                GroupMessage(gnum, pnum, msg) => {
                    on_group_message(&mut tox, gnum, pnum, msg, &mut bot)
                },

                GroupNamelistChange(gnum, pnum, change) => {
                    on_group_name_list_change(&mut tox, gnum, pnum, change);
                },

                ev => { println!("Tox event: {:?}", ev); },
            }
        }
        if bot.title_pin {
            drop(tox.group_set_title(0, "#tox-real-ontopic | so what triggers everyone?"));
        }


        /*
            Let Lee speak every $time_interval, provided that there is given
            permission for it
        */
        if bot.speak {
        let cur_time = UTC::now().timestamp();
            if  (bot.last_time + 9) < cur_time {
                /* Should have only small chance to speak */
                if 0.02 > bot.random.gen::<f64>() {
                    let message = bot.markov.generate_str();
                    drop(tox.group_message_send(bot.last_group, &message));
                }

                bot.last_time = cur_time;
            }
        }


        tox.wait();
    }
}
