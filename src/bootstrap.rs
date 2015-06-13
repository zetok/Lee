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

////////////////////////////////////////////////////////////////////////////

use rstox::core::*;

/*
    Bootstrap nodes section, should be used only when there's no available file
    with bootstrap nodes.

    According to Tox client guidelines[1], one should try to bootstrap with at
    least 4 random nodes from list of public ones. Since this section is only
    meant for backup bootstrapping when there is no list provided in separate
    file, it only will have 4 hardcoded nodes, which are known to be working
    at the time. Should contain both IPv4 and IPv6 nodes.

    Given those constraints, bootstrap nodes were selected due to my own
    personal opinion, which is affected by multitude of factors.

    [1] https://github.com/irungentoo/Tox_Client_Guidelines/blob/master/Required/Bootstrapping.md
*/
// TODO: use those only when there's no list with bootstrap nodes present
struct BootNode<'a> {
    name: &'a str,
    ips: &'a [&'a str],
    port: u16,
    key: &'a str,
}

const BOOTSTRAP_NODES: &'static [BootNode<'static>] = &[
    BootNode {
        name: "sonOfRa",
        ips: &["144.76.60.215", "2a01:4f8:191:64d6::1"],
        port: 33445,
        key: "04119E835DF3E78BACF0F84235B300546AF8B936F035185E2A8E9E0A67C8924F",
    },

    BootNode {
        name: "SylvieLorxu",
        ips: &["178.21.112.187", "2a02:2308::216:3eff:fe82:eaef"],
        port: 33445,
        key: "4B2C19E924972CB9B57732FB172F8A8604DE13EEDA2A6234E348983344B23057",
    },

    BootNode {
        name: "Impyy",
        ips: &["178.62.250.138", "2a03:b0c0:2:d0::16:1"],
        port: 33445,
        key: "788236D34978D1D5BD822F0A5BEBD2C53C64CC31CD3149350EE27D4D9A2F9B6B",
    },

    BootNode {
        name: "ray65536",
        ips: &["108.61.165.198"],
        port: 33445,
        key: "8E7D0B859922EF569298B4D261A8CCB5FEA14FB91ED412A7603A585A25698832",
    },
];

/*
    Function should be skipped when there is provided file with bootstrap
    nodes. Should be called after Tox instance will be initialized.
*/
// TODO: ↓ check whether bootstrap nodes were loaded from a file, and if that
// was the case, then skip this function
pub fn bootstrap_hardcoded(tox: &mut Tox) {
    for node in BOOTSTRAP_NODES {
        let key = node.key.parse().unwrap();
        for ip in node.ips {
            println!("Bootstrapping from {}: [{}]:{}, key: {}", node.name, ip, node.port, node.key);
            tox.bootstrap(ip, node.port, key).unwrap();
        }
    }
}
