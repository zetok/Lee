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



extern crate rstox;

use rstox::core::*;

mod bootstrap;


// TODO: load it from config file, if not available, then use default one
//         * perhaps it could be made of some random chars generated
//           at runtime?
static BOT_NAME: &'static str = "Layer\0";


/*
    Function that deals with incoming friend requests
*/
fn on_friend_request(tox: &mut Tox, fpk: PublicKey, msg: String) {
    drop(tox.add_friend_norequest(&fpk));
    println!("Friend {} with friend message {:?} was added.", fpk, msg);
}


/*
    Function that deals with incoming invites to groupchats
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


fn main() {
    /*
        Defend my honour. Needed to compare whether someone is not trying to
        use my nick.
        Also defend bot.
    */
    // TODO: need to switch to PK-based impostor detection, since apparently
    //       some people can get '\0' as their name :3
    let fake_names = &["Zetok", "zetok", "Layer"];

    let mut tox = Tox::new(ToxOptions::new(), None).unwrap();

    drop(tox.set_name(BOT_NAME));

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
                    match tox.group_peername(gnum, pnum) {
                        Some(pname) => {
                            if fake_names.contains(&&*pname) {
                                drop(tox.group_message_send(gnum, "↑ an impostor!"));
                            }

                            println!("Tox event: GroupMessage({}, {}, {:?}), Name: {:?}", gnum, pnum, msg, pname);
                        },
                        None => {
                            println!("Tox event: GroupMessage({}, {}, {:?}), Name: •not known•",
                                gnum, pnum, msg);
                        },
                    }
                },

                GroupNamelistChange(gnum, pnum, change) => {
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
                },

                ev => { println!("Tox event: {:?}", ev); },
            }
        }
        tox.wait();
    }
}
