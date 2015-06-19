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
    Lee's own stuff
*/
// TODO: when other functions will be moved from main.rs, things should be
//       added here
mod bootstrap;
mod for_markov;


// TODO: load it from config file, if not available, then use default one
//         * perhaps it could be made of some random chars generated
//           at runtime?
static BOT_NAME: &'static str = "Lee";
static BOT_NAMES: &'static [&'static str] = &["Lee", "lee"];


/*
    Defend my honour. Needed to compare whether someone is not trying to
    use my nick.
*/
// TODO: need to switch to PK-based impostor detection, since apparently
//       some people can get '\0' as their name :3
const FAKE_NAMES: &'static [&'static str] = &["Zetok", "zetok"];


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
fn on_group_message(tox: &mut Tox, gnum: i32, pnum: i32, msg: String, title: &mut bool, markov: &mut Chain<String>) {
    // feed Lee with message content
    markov.feed_str(&msg);

    /*
        Triggers Lee
    */
    fn trigger_response(tox: &mut Tox, gnum: i32, msg: &String, markov: &mut Chain<String>) {
        for name in BOT_NAMES {
            if msg.contains(name) {
                let message: String = markov.generate_str();
                drop(tox.group_message_send(gnum, &message));
            }
        }
    }

    match tox.group_peername(gnum, pnum) {
        Some(pname) => {
            if FAKE_NAMES.contains(&&*pname) {
                drop(tox.group_message_send(gnum, "↑ an impostor!"));
            }

            if pname == "Zetok\0" {
                if msg == ".trigger" {
                    *title = true;
                } else if msg == ".rmtrigger" {
                    *title = false;
                }
            }

            trigger_response(tox, gnum, &msg, markov);

            println!("Tox event: GroupMessage({}, {}, {:?}), Name: {:?}", gnum, pnum, msg, pname);
        },

        None => {
            trigger_response(tox, gnum, &msg, markov);

            println!("Tox event: GroupMessage({}, {}, {:?}), Name: •not known•",
                gnum, pnum, msg);
        },
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

    let mut chain = Chain::for_strings();
    for_markov::feed_markov(&mut chain);

    let mut tox = Tox::new(ToxOptions::new(), None).unwrap();

    drop(tox.set_name(BOT_NAME));


    /*
        If set to true, groupchat title should be protected.
    */
    let mut set_title: bool = false;

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
                    on_group_message(&mut tox, gnum, pnum, msg, &mut set_title, &mut chain)
                },

                GroupNamelistChange(gnum, pnum, change) => {
                    on_group_name_list_change(&mut tox, gnum, pnum, change);
                },

                ev => { println!("Tox event: {:?}", ev); },
            }
        }
        if set_title {
            drop(tox.group_set_title(0, "#tox-real-ontopic | so what triggers everyone?"));
        }
        tox.wait();
    }
}
