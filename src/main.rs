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

    tox.set_name(BOT_NAME).unwrap();

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

    println!("{}", tox.get_address());

    loop {
        for ev in tox.iter() {
            match ev {
                FriendRequest(cid, _) => {
                    tox.add_friend_norequest(&cid).unwrap();
                },
                GroupInvite(fid, kind, data) => {
                    match kind {
                        GroupchatType::Text => {
                            tox.join_groupchat(fid, &data).unwrap();
                        },
                        _ => {},
                    }
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
