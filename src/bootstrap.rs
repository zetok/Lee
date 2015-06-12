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

    Given those contsrains, bootstrap nodes were selected due to my own
    personal opinion, which is affected by multitude of factors.

    [1] https://github.com/irungentoo/Tox_Client_Guidelines/blob/master/Required/Bootstrapping.md
*/
// TODO: use those only when there's no list with bootstrap nodes present
static BOOTSTRAP1_NAME: &'static str = "sonOfRa";
static BOOTSTRAP1_IPV4: &'static str = "144.76.60.215";
static BOOTSTRAP1_IPV6: &'static str = "2a01:4f8:191:64d6::1";
static BOOTSTRAP1_PORT: u16 = 33445;
static BOOTSTRAP1_KEY:  &'static str =
    "04119E835DF3E78BACF0F84235B300546AF8B936F035185E2A8E9E0A67C8924F";

static BOOTSTRAP2_NAME: &'static str = "SylvieLorxu";
static BOOTSTRAP2_IPV4: &'static str = "178.21.112.187";
static BOOTSTRAP2_IPV6: &'static str = "2a02:2308::216:3eff:fe82:eaef";
static BOOTSTRAP2_PORT: u16 = 33445;
static BOOTSTRAP2_KEY:  &'static str =
    "4B2C19E924972CB9B57732FB172F8A8604DE13EEDA2A6234E348983344B23057";

static BOOTSTRAP3_NAME: &'static str = "Impyy";
static BOOTSTRAP3_IPV4: &'static str = "178.62.250.138";
static BOOTSTRAP3_IPV6: &'static str = "2a03:b0c0:2:d0::16:1";
static BOOTSTRAP3_PORT: u16 = 33445;
static BOOTSTRAP3_KEY:  &'static str =
    "788236D34978D1D5BD822F0A5BEBD2C53C64CC31CD3149350EE27D4D9A2F9B6B";

static BOOTSTRAP4_NAME: &'static str = "ray65536";
static BOOTSTRAP4_IPV4: &'static str = "108.61.165.198";
static BOOTSTRAP4_PORT: u16 = 33445;
static BOOTSTRAP4_KEY:  &'static str =
    "8E7D0B859922EF569298B4D261A8CCB5FEA14FB91ED412A7603A585A25698832";


/*
    Function should be skipped when there is provided file with bootstrap
    nodes. Should be called after Tox instance will be initialized.
*/
// TODO: ↓ check whether bootstrap nodes were loaded from a file, and if that
// was the case, then skip this function (move to exec)
pub fn bootstrap_hardcoded(tox: &mut Tox) {
    /*
        Booststrap 1, both IPv4 and IPv6
    */
    let bootstrap1_key = BOOTSTRAP1_KEY.parse().unwrap();
    tox.bootstrap(BOOTSTRAP1_IPV4, BOOTSTRAP1_PORT, bootstrap1_key).unwrap();

    println!("Bootstrapping from {}, {}, {}, {}",
        BOOTSTRAP1_NAME, BOOTSTRAP1_IPV4, BOOTSTRAP1_PORT, BOOTSTRAP1_KEY);

    tox.bootstrap(BOOTSTRAP1_IPV6, BOOTSTRAP1_PORT, bootstrap1_key).unwrap();
    println!("Bootstrapping from {}, {}, {}, {}",
        BOOTSTRAP1_NAME, BOOTSTRAP1_IPV6, BOOTSTRAP1_PORT, BOOTSTRAP1_KEY);


    /*
        Bootstrap 2, both IPv4 and IPv6
    */
    let bootstrap2_key = BOOTSTRAP2_KEY.parse().unwrap();
    tox.bootstrap(BOOTSTRAP2_IPV4, BOOTSTRAP2_PORT, bootstrap2_key).unwrap();
    println!("Bootstrapping from {}, {}, {}, {}",
        BOOTSTRAP2_NAME, BOOTSTRAP2_IPV4, BOOTSTRAP2_PORT, BOOTSTRAP2_KEY);

    tox.bootstrap(BOOTSTRAP2_IPV6, BOOTSTRAP2_PORT, bootstrap2_key).unwrap();
    println!("Bootstrapping from {}, {}, {}, {}",
        BOOTSTRAP2_NAME, BOOTSTRAP2_IPV6, BOOTSTRAP2_PORT, BOOTSTRAP2_KEY);


    /*
        Bootstrap 3, both IPv4 and IPv6
    */
    let bootstrap3_key = BOOTSTRAP3_KEY.parse().unwrap();
    tox.bootstrap(BOOTSTRAP3_IPV4, BOOTSTRAP3_PORT, bootstrap3_key).unwrap();
    println!("Bootstrapping from {}, {}, {}, {}",
        BOOTSTRAP3_NAME, BOOTSTRAP3_IPV4, BOOTSTRAP3_PORT, BOOTSTRAP3_KEY);

    tox.bootstrap(BOOTSTRAP3_IPV6, BOOTSTRAP3_PORT, bootstrap3_key).unwrap();
    println!("Bootstrapping from {}, {}, {}, {}",
        BOOTSTRAP3_NAME, BOOTSTRAP3_IPV6, BOOTSTRAP3_PORT, BOOTSTRAP3_KEY);


    /*
        Bootstrap 4, only IPv4
    */
    let bootstrap3_key = BOOTSTRAP3_KEY.parse().unwrap();
    tox.bootstrap(BOOTSTRAP4_IPV4, BOOTSTRAP4_PORT, bootstrap3_key).unwrap();
    println!("Bootstrapping from {}, {}, {}, {}",
        BOOTSTRAP4_NAME, BOOTSTRAP4_IPV4, BOOTSTRAP4_PORT, BOOTSTRAP4_KEY);
}
