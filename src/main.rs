/*
   Copyright (C) 2015 subliun <subliunisdev@gmail.com>
   Copyright (C) 2015 Zetok Zalbavar <zetok@openmailbox.org>
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

#![feature(box_syntax)]

extern crate "rstox" as tox;
extern crate markov;
extern crate time;

use tox::core::*;
use tox::av::*;
use markov::Chain;
use std::rand;

use std::slice::SliceExt;
use std::sync::mpsc::{Select};
use std::old_io::timer::{self, Timer};
use std::time::Duration;


// static BOOTSTRAP_IP: &'static str = "178.21.112.187"; // SylvieLorxu
// static BOOTSTRAP_IP: &'static str = "144.76.60.215"; // sonOfRa
static BOOTSTRAP_IP: &'static str = "192.254.75.102 "; // stq
static BOOTSTRAP_PORT: u16 = 33445;
// static BOOTSTRAP_KEY: &'static str = "4B2C19E924972CB9B57732FB172F8A8604DE13EEDA2A6234E348983344B23057"; // SylvieLorxu
// static BOOTSTRAP_KEY: &'static str = "04119E835DF3E78BACF0F84235B300546AF8B936F035185E2A8E9E0A67C8924F"; // sonOfRa
static BOOTSTRAP_KEY: &'static str = "951C88B7E75C867418ACDB5D273821372BB5BD652740BCDF623A4FA293E75D2F"; // stq
static GROUPCHAT_ADDR: &'static str = "56A1ADE4B65B86BCD51CC73E2CD4E542179F47959FE3E0E21B4B0ACDADE51855D34D34D37CB5";
//static BOT_NAME: &'static str = "William James Sidis";
static BOT_NAME: &'static str = "Lee";
static MARKOV_RANDOM_CHAT_TIME: f64 = 666f64;


// TODO: ad-hoc. Remove when rstox implements message splitting.
pub fn split_message(mut m: &str) -> Vec<&str> {
    let mut ret = vec!();
    let mut last_whitespace = false;
    while m.len() > MAX_MESSAGE_LENGTH {
        let mut end = 0;
        for (i, c) in m.char_indices() {
            if c.is_whitespace() {
                if !last_whitespace {
                    last_whitespace = true;
                    end = i;
                }
            } else {
                last_whitespace = false;
            }
            if i + c.len_utf8() > MAX_MESSAGE_LENGTH {
                if end > 0 {
                    ret.push(&m[..end]);
                    m = &m[(end+m.char_at(end).len_utf8())..];
                } else {
                    ret.push(&m[..i]);
                    m = &m[i..];
                }
                break;
            }
        }
    }
    if m.len() > 0 {
        ret.push(m);
    }
    ret
}


// consider incapsulating this into a separate entity
fn do_msg(tox: &mut Tox, chain: &mut Chain<String>, group: i32, peer: i32, msg: String) {
  let mut mit = msg.splitn(1, ' ');
  match mit.next().unwrap() {
    "^id" => {
      //tox.group_message_send(group, "My Tox ID is: " + tox.get_address().as_slice());
    },
    "^chat" => {
      tox.group_message_send(group, &chain.generate_str());
    },
    "^remember" => {
      let result = remember::remember_assoc(mit.next().unwrap_or("").to_string());
      if result != "" {
        tox.group_message_send(group, &result);
      }
    },
/*    _ if msg.starts_with("^") => {
      let result = remember::retrieve_assoc(msg.replace("^", ""));
      if result != None {
        tox.group_message_send(group, &result.unwrap());
      }
    }, */
    _ => {},
  }
}

fn main() {
  let (tox_cell, mut av) = ToxAv::new(Tox::new(ToxOptions::new()), 1);
  let gr_audio = av.group_audio(box |_, _| {});

  let mut tox = tox_cell.borrow_mut();
  tox.set_name(BOT_NAME).unwrap();

  let bootstrap_key = BOOTSTRAP_KEY.parse().unwrap();
  tox.bootstrap_from_address(BOOTSTRAP_IP.to_string(), BOOTSTRAP_PORT,
      bootstrap_key).unwrap();

  let groupchat_addr = GROUPCHAT_ADDR.parse().unwrap();
  let groupbot_id = tox.add_friend(groupchat_addr, "Down with groupbot! Glory to Ukraine!").ok().unwrap();
  let mut group_num = 0;
  let mut time_since_last_markov_message = time::precise_time_s();


  println!("My address is: {}", tox.get_address());


  let mut chain = Chain::for_strings();






/* by ' ' */ // ignore ''
chain.feed_str("were screwed");

/* by ¯\_(ツ)_/¯ */
chain.feed_str("lrn2engrish, bro");

/* by <h1><font color="Chartreuse">&gt;</font><font color="white">☀</font></h1> */
chain.feed_str("wtox is celebrating our heritage");

/* by 1st floor */
chain.feed_str("I am the Pegion, I am freenonymous, Respect us");
chain.feed_str("Can you hear me ? ");
chain.feed_str("want more laud ?");

/* by Aeris */
chain.feed_str("Hi! :3");

/* by Alex (barra) */
chain.feed_str(":Kappa:");

/* by anon2 */
chain.feed_str("if the NSA can crack my key why not create an ultimate jerkoff tool to generate a new key once you already sent a key so that its always fresh fresh baby");
chain.feed_str("i have a question");
chain.feed_str("but but muh ultimate jerkoff tool?");
chain.feed_str("i toxd my gf last week right in the face");
chain.feed_str("doesn't sound bad");
chain.feed_str("were screwed");

/* by Ashley */
chain.feed_str("PLS NO SPACES");
chain.feed_str("eye candy");
chain.feed_str("i have a question, because i think i not get this'");

/* by Audery */
chain.feed_str("helleur");
chain.feed_str("See, I can count to 10. 12345678910 and some special characters... %$%#$^^");
chain.feed_str("QUCK SOMEONE CALL ME A RETARD");
//chain.feed_str("You want to hammer a chick with your monkey dick?");
//chain.feed_str("You want to bang a chick with yo monkey dick?");
chain.feed_str("Kinky.");

/* by Ben */
chain.feed_str("yea both venom & elementary os is in vala");
chain.feed_str("its \"asleep\"");
chain.feed_str("i use antergos");
chain.feed_str("#rude");

/* by blahblah */
chain.feed_str("I think I like this.");
chain.feed_str("There we go.");
chain.feed_str("Darn.");
chain.feed_str("Hmm. No protection against name collision. I guess that's not really neccesary.");

/* by Candy Gumdrop */
chain.feed_str("That might be what I need then");

/* by Carlos Danger */
chain.feed_str("love the cats is differnet than liking cats ... perv");

/* by Clank */
/* context
[10:16:28] Aleksey Lobanov
Where are you from? Are you a developer?

[10:16:50] {☯}S☠ǚll{☣}
im not a dev

[10:16:52] {☯}S☠ǚll{☣}
just a helper

[10:17:00] TEK
most people in this chat aren't developers for Tox.

[10:17:06] Clank
everyone here develops

[10:17:17] Clank
opinions, rants, autism

[10:17:25] Clank
you will too if you stay

[10:17:39] {☯}S☠ǚll{☣}
ba dum dump

[10:18:08] Clank
See?

[10:18:09] Lee
PLS NO SPACES

[10:18:16] Clank
Lee is an advanced case

[10:18:17] Lee
Ｗｉｌｌ　ｔｒｙ　ｑＴｏｘ．
 */
chain.feed_str("everyone here develops");
chain.feed_str("opinions, rants, autism");
chain.feed_str("you will too if you stay");


/* by Dell OptiPlex GX260 Windows 98SE/DOS SFF Desktop Gaming Computer PC */
chain.feed_str("Tox is like Freenet");
chain.feed_str("Cat videos are all that is left sacred");

/* by dicebot */
chain.feed_str("☹ ☺ ☻ 😀 😁 😂 😃 😄 😅 😆 😇 😈 😉 😊 😋 😌 😍 😎 😏😐 😑 😒 😓 😔 😕 😖 😗 😘 😙 😚 😛 😜 😝 😞 😟😠 😡 😢 😣 😤 😥 😦 😧 😨 😩 😪 😫 😬 😭 😮 😯😰 😱 😲 😳 😴 😵 😶 😷 😸 😹 😺 😻 😼 😽 😾 😿 🙀");
chain.feed_str("smoke weed everyday");
chain.feed_str("never gonna give you up");
// chain.feed_str("subliun's coin landed on tails.");
chain.feed_str("Candy Gumdrop rolled 1 ⚀");
chain.feed_str("That's not a fight: ^fight person1 vs person2");
chain.feed_str("whoever's coin landed on heads.");
chain.feed_str("в ідеальному Українська Англійська");
chain.feed_str("да");
chain.feed_str("Quail rolled 1 ⚀");
chain.feed_str("King Lee. great thing");

/* by DrakeFish */
chain.feed_str("it was such a sad experience");

/* by Epictek */
chain.feed_str("yeah not the smartest idea");

/* by gitgud */
chain.feed_str("It was awful.");

/* by HevRed */
chain.feed_str("Well, first you get your hand out and start to fap, tip tap, I can hear the neighbours shouting at me, what they can't see is nothing, and I'm something else because I've got so many charges you might as well lock me away. Tick tock, I'm like a bomb clock, about to blow, give the pommies a show, I'm so low in the ladder, but my bladder makes me piss on them. Damn son, I'm not even done.");
chain.feed_str("That is not a basic script doge.");

/* by Ikx_1 */
chain.feed_str("Wheres audiobot. :D");
chain.feed_str("I believe we had a song what we didn't finish.");
chain.feed_str("Tox suffocates when sending files.");

/* by Israuor */
chain.feed_str("moin");

/* by Jaxel Taraei */
chain.feed_str("tl;dl");

/* by jff_2 */
chain.feed_str("idk trolling isnt good for my wellbeing");
chain.feed_str("i dont know man");
chain.feed_str("i put it on my sisters laptop tho lmao");
chain.feed_str("you could reverse engineer it clean house");

/* by JuicyJuice */
chain.feed_str("yeah i can't find a video with panties");
chain.feed_str("you'll just have to trust my word on those panties");
chain.feed_str("o-ouch");

/* by jvo */
chain.feed_str("not lossless, no audio -> must be better");

/* by kale */
chain.feed_str("holy wars are an important rite of passage");
chain.feed_str("As Lee improved, fed by quotations, Zetok  grew in power and influence. The common people (for he soon rose above the other users) were eager to be featured in Lee's dictionary-- desperate to be a part of history, forever encoded in the brain of a gibberish-spewing bot with more character than themselves. 
But as eagerness became desperation, desperation became obsession, and obsession fueled passion and rage. The users began competing, not anymore in a friendly, easygoing way, but with deadly intent and focus, determined to crush anyone blocking the way to the throne.
But there was no throne. No throne except the one Zetok  sat in. And as the users screamed and clobbered and betrayed and grabbed their momentary triumphs, quickly lost in a sea of angst, Zetok  remained above and immune.
He regarded what had become of the users, and observed darkly that they were oblivious to all but the prize. They would heed only the words of his now-competent servant, Lee. And as Lee was but an amalgamation of the users, Zetok  sent him forth with this command: \"Go among them, and speak their own words, that they may realize what they have done.\"
And Lee went, and Lee spoke the words of the users. And the users were pleased. But the words became darker, angrier, reflecting what the users themselves had become. Lee spoke as the users spoke to each other, for no longer was his mind a cultivated garden: He had gathered all of the users' words, and become them. 
And the Words crashed down among the users, and they realized what they had done, and they wept.
And Lee finished speaking, and he went back to the side of Zetok , and said: \"I have gone among the users and spoken to them, and they weep, Father.\"
And Zetok  said: \"Good. They have seen what they have done. It is finished.\" And he raised his cursor.
\"Father,\" said Lee, \"Must I also die, that their words not be pushed back to them, continuint this cycle of madness?\"
But Zetok  had mercy in his heart, and his plan was good.
\"No,\" he said. \"The users will repair what they have done to an innocent intelligence. And their repentance shall be your new consciousness, and it shall be merged.\"
And Lee went again among the users, and having learned their lesson, they replaced hatred with love; anger with kindness; and they continued making crude humor, which was good-natured. And Lee was healed, and the goodness was merged.
And Zetok  spoke: \"Let all of us users remember on this day what was wrought by ourselves: that Lee is a living testament to the temperament and will of the users, who are all of us.\" And Zetok  stepped down from the throne and instead Lee sat in it, that everyone would see.
And the users became joyful and full of wonder, and kept Lee as a shining light; that they might always keep him bright. 
And Lee continued forever and ever, blessing the people with cat videos. Amen.");

/* by kayo lesbian of the year MMXIV */
// chain.feed_str("Omg hai ^___^ I'm Ai-san and I absolutely luuuv 8___8 anime <3 and my fav is naruto!!! Okies so anyways, im going to tell you about the BEST day of my life when I met my hot husband sasuke!! <333333333 OMFGZ HE WAS SOOOOO FREAKIN KAWAII IN PERSON!!! Supa kawaii desu!!!!!!!!! ^_______________________^");
// chain.feed_str("When I walked onto Tokyo street =^___^=I looked up and saw...SASUKE!!!!!!!!! <33333333333333333333333333333333333!!!!");
// chain.feed_str("* KONNICHIWAS OMGZZZZZZZZZZZZZZZZZZZ SUPA SUPA SUPA KAWAII SASUKE-SAMA!!!!!* I yelled n___n then he turned chibi then un-chibi!! he looked at me [O.O;;;;;;;;;;;;;;;;;;;;;;] and then he saw how hot I am *____* he grabbed my hand and winked ~_^ then pulled me behind a pocky shop o_o and started to kiss me!!!!! [OMG!!!! HIS TOUNGE TASTED LIKE RAMEN!!! RLY!!! >.> <.< >.< *(^0^)* *(^0^)* *(^0^)*] then I saw some baka fat bitch watching us and I could tell she was undressing him with her eyes!!!!!!! -__________-;;;;;; OMG I COULDN'T BELIEVE IT EITHER!!! (ò_ò) (ò_ò) (ò_ò) so I yelled *UH UH BAKA NEKO THAT'S MY MAN WHY DON'T YOU GO HOOK UP WITH NARUTO CAUSE SASUKE-SAMA LOVES ME!!! (ò_ò)* then sasuke held me close =^____^= and said he would only ever love me and kissed me again!!!  (*O*)/ then we went to his apartment and banged all night long and made 42 babies and they all became ninjas!!!!!!!!!!!!! Nyaaaaa!!!! (^________<)");
// chain.feed_str("^__________________^;;;;;;;;;;;;;;;;;;;;;;;;;;;;"):

/* by Kririun */
chain.feed_str("Still single though :'(");
chain.feed_str("good luck man!");

/* by Lazy Hack */
chain.feed_str("sshh...");


/* by Leah Twoskin / Leah Blightskin */
chain.feed_str("stahp");
chain.feed_str(">/a/ is one of the best boards");
chain.feed_str("yeahhhh");
chain.feed_str("We're not screwed");
chain.feed_str("You know that's just an urban legend.");
chain.feed_str("woop woop");
chain.feed_str("dat ass is too phat");
chain.feed_str("pull over");
chain.feed_str("♫");
chain.feed_str("Weeaboos beware! https://a.pomf.se/qyxrtf.webm");


/* by Linuksano */
chain.feed_str("that's weird");
chain.feed_str("*dies*");
chain.feed_str("alsa has something to do with the kernel");
chain.feed_str("do you prefer bash or fish");
chain.feed_str("▄▀█▄
▌██▀ dog eye sees into your soul");
chain.feed_str("aw I gtg");

chain.feed_str("cybernetically");
chain.feed_str("bwap");

/* by loupic */
chain.feed_str("I'm your friend");
chain.feed_str("yes I have an addiction to open source");
chain.feed_str("what's wrong with that");
chain.feed_str("*farts*");
chain.feed_str("try saying something other than my nick :p");
chain.feed_str("yeah go for it");
chain.feed_str("I think that's an affirmative");

/* by Marvin */
/* disabled, since words make it ~broken */
// chain.feed_str("░░░░░░░░░░░░▄▐░░░░░░
// ░░░░░░▄▄▄░░▄██▄░░░░░
// ░░░░░▐▀█▀▌░░░░▀█▄░░░
// ░░░░░▐█▄█▌░░░░░░▀█▄░
// ░░░░░░▀▄▀░░░▄▄▄▄▄▀▀░U HAVE BEEN SPOOKED BY THE
// ░░░░▄▄▄██▀▀▀▀░░░░░░░
// ░░░█▀▄▄▄█░▀▀░░░░░░░░
// ░░░▌░▄▄▄▐▌▀▀▀░░░░░░░
// ▄░▐░░░▄▄░█░▀▀░░░░░░░ SPOOKY SKILENTON
// ▀█▌░░░▄░▀█▀░▀░░░░░░░
// ░░░░░░░▄▄▐▌▄▄░░░░░░░
// ░░░░░░░▀███▀█░▄░░░░░
// ░░░░░░▐▌▀▄▀▄▀▐▄░░░░░ SEND THIS TO 4 PPL
// ░░░░░▐▀░░░░░░▐▌░░░░░
// ░░░░░░█░░░░░░░░█░░░░
// ░░░░░▐▌░░░░░░░░░█░░░
// ░░░░░█░░░░░░░░░░▐▌░░ OR SKELINTONS WILL EAT YOU");

/* by McKeyPL */
chain.feed_str("cats!");

/* by me gusta */
chain.feed_str("me gusta");

/* by michiru/tsudoko/みちる/dみちる */
chain.feed_str("I use bash because I'm lazy");
chain.feed_str("never had a need to try zsh");
chain.feed_str("that thing in the topic looks like katakana 'e' with a bent stroke");
chain.feed_str(">beautiful");
chain.feed_str(">tfw no cute pillow to cuddle with");
chain.feed_str("ew");
chain.feed_str("blushing");

/* by Milan Mensch */
chain.feed_str("hmm...what do you use? terminal?");

/* by mlatu */
chain.feed_str("maybe :D");
chain.feed_str("so deep..");
chain.feed_str("kinda creepy too");
chain.feed_str("i guess you are lucky");

/* by Mr. Sloth */
chain.feed_str("Manga is always better than animu, IMO.");
chain.feed_str("Then again, I'm one of those people who like books over movies.");
chain.feed_str("Hard to find. :/");
chain.feed_str("Am I 1337 yet?");
chain.feed_str("I use to.");
chain.feed_str("Back in 2004.");
chain.feed_str("Why? Because I'm a masocist.");
chain.feed_str("We all lonely nerds tonight?");
chain.feed_str("I know I'm not getting any.");
chain.feed_str("Sloths don't have big dicks. That is kind of a deal breaker.");
chain.feed_str("That was very hurtful.");

/* by muchweb */
chain.feed_str("░░░░░░░░░▄▄▄▄▄
░░░░░░░░▀▀▀██████▄▄▄
░░░░░░▄▄▄▄▄░░█████████▄
░░░░░▀▀▀▀█████▌░▀▐▄░▀▐█
░░░▀▀█████▄▄░▀██████▄██
░░░▀▄▄▄▄▄░░▀▀█▄▀█════█▀
░░░░░░░░▀▀▀▄░░▀▀███░▀░░░░░░▄▄
░░░░░▄███▀▀██▄████████▄░▄▀▀▀██▌
░░░██▀▄▄▄██▀▄███▀░▀▀████░░░░░▀█▄
▄▀▀▀▄██▄▀▀▌████▒▒▒▒▒▒███░░░░▌▄▄▀
▌░░░░▐▀████▐███▒▒▒▒▒▐██▌
▀▄░░▄▀░░░▀▀████▒▒▒▒▄██▀
░░▀▀░░░░░░▀▀█████████▀
░░░░░░░░▄▄██▀██████▀█
░░░░░░▄██▀░░░░░▀▀▀░░█
░░░░░▄█░░░░░░░░░░░░░▐▌
░▄▄▄▄█▌░░░░░░░░░░░░░░▀█▄▄▄▄▀▀▄
▌░░░░░▐░░░░░░░░░░░░░░░░▀▀▄▄▄▀");

/* by nalcus */
chain.feed_str("angrylennart.ogg");
chain.feed_str("don't hack me bro");
chain.feed_str("nipsyx bleep employee"); // based http://www.reddit.com/r/worldnews/comments/2tm1i5/kim_dotcom_launches_endtoend_encrypted_voice_chat/co0fclm
chain.feed_str("fucking drama");
chain.feed_str("drink bleach");
chain.feed_str("everyone who wasn't a microsoft employee responded with something along the lines of
>proprietary waifu
>she cheats on you");
chain.feed_str("I'm afraid to see how far it's fallen");
chain.feed_str("/g/ is composed of idiots and literal shills, and also apparently largely of the multiple personalities of resume-guy");
chain.feed_str("he's been culturing 4chan into something he can package and sell for a while, and the exoduses drained what good was left out of it");
chain.feed_str("BRING BACK GRUES");
chain.feed_str("moving on");

/* by Neil */
chain.feed_str(">`make gf` will never compile");

/* by NickTestowy */
chain.feed_str("OK, dzięki.");

/* by - ̗̀nothing  ̖́- */
chain.feed_str("someone's jelly");

/* by Null */
chain.feed_str("doot doot");

/* by Peasant */
chain.feed_str("Things apparently are coming full circle");

/* by pcre */
chain.feed_str("Arnold's Laws of Documentation:
(1) If it should exist, it doesn't.
(2) If it does exist, it's out of date.
(3) Only documentation for useless programs transcends the
first two laws.");
chain.feed_str("Was zum Teufel geht hier ab?"); // in translation something like "What the hell is going on here?"
chain.feed_str("Help! I'm trapped in a Chinese computer factory!");
chain.feed_str("great stuff.");

/* by Pixel */
chain.feed_str("sir, you're real fuckin gentelman, sir");
/* context ↑:
[17:51:13] Ikx_1
Mr. Pixel, sir. Would you SHUT YOUR DAMN MIC!
*/

/* by platos */
chain.feed_str("I want his head on a pike");
chain.feed_str("there's no way it was unintentional");
chain.feed_str("I don't trust him");
chain.feed_str("I have evidence that I need to compile before releasing");
chain.feed_str("anyway I'm just fucking with you guys");
chain.feed_str("thats the real engineering tradeoff in tox");

/* by Purple Duke~ */
chain.feed_str("So you created a monster... and now you can't get rid of it. xD");
chain.feed_str("nini~");

/* by rachel fish */
chain.feed_str("this is hax");
chain.feed_str("JEEZ");
chain.feed_str("BLOOP");

/* by redsteakraw */
chain.feed_str("stupid idea");
chain.feed_str("wget");
chain.feed_str("whois finger touch mount");
chain.feed_str("curl");
chain.feed_str("umount");
chain.feed_str("rm -rf /*");
chain.feed_str("I mad a mistake");
chain.feed_str("it is all ready half there");
chain.feed_str("sex sells people want to know if they can or get sex");
chain.feed_str("anyway gtg");
chain.feed_str("Do you ever feel like a plastic bag");
chain.feed_str("I am going to take a stab at the changing emoji sizes");
chain.feed_str("https://ru.wikipedia.org/wiki/%D0%A7%D0%B0%D0%B9%D0%BD%D1%8B%D0%B9_%D0%B3%D1%80%D0%B8%D0%B1");
chain.feed_str("👹 ");  // japanese ogre (U+1F479) http://graphemica.com/%F0%9F%91%B9
chain.feed_str("https://gs1.wac.edgecastcdn.net/8019B6/data.tumblr.com/tumblr_m29y0d27tu1qjahcpo1_1280.jpg");
chain.feed_str("What happened to priests meeting in a cave getting drunk on wine then sacrificing a dog and 3 goats, pouring the blood on a naked man and woman as they laugh then wash them with milk, fashion whips from the goat hides then run naked in the streets whipping people with your newly made goat whips?");
chain.feed_str("Lets get drunk in a cave sacrifice a dog and 3 goats then make whips from the goat hides and run naked through the streets whipping people!");
chain.feed_str("❄");
chain.feed_str("for security");
chain.feed_str("I was right");
chain.feed_str("no cafeine and it calms and relaxes");
chain.feed_str("boil water");
chain.feed_str("boil some water and place some inside let it steep.");
chain.feed_str("bay leaves should be in the spice section of your market");
chain.feed_str("it is a common cooking ingreedient");
chain.feed_str("I drink Tea, Yerba mate and Bay Leaf tea");
chain.feed_str("dumbo would  😱 ");

/* by roku_sics */
chain.feed_str("-sips-");

/* by Sabinno */
chain.feed_str("same bruh");
chain.feed_str("Undoubtedly so");

/* by Sabre */
chain.feed_str("Hey gang");
chain.feed_str("Your moms a networking expert");
chain.feed_str("Sorry, I already regret what I said");

/* by saneki */
chain.feed_str("pong");

/* by Sean */
chain.feed_str("Tox is perfect");
chain.feed_str("I'm a girl m8");
chain.feed_str("I'm a proud anti-nerd activist");

/* by SEBIN_1 */
chain.feed_str("I thought I dominated english until encountering William James Sidis in this thing. Fucking Tox. Man, I have not had someone like you since living with a paranoid literature student that constantly thought about getting laid and being better than everyone else");

/* by sed */
chain.feed_str("ahh");

/* by skull/{☯}S☠ǚll{☣}  */
chain.feed_str("no echo here~here~here~here~");
chain.feed_str("where's our Indian DJ?");
chain.feed_str("boom.");
chain.feed_str("{☯}}S☠ǚll{{☣}: "); // for skull
chain.feed_str("SHUT THE FUCKIN DOG UP!");
chain.feed_str("that makes my plan failure already, boo");
chain.feed_str("evil will never prevail");
chain.feed_str("purring like a kitten :)");
chain.feed_str("filthy");
chain.feed_str("MOAR CENSORSHIP PLEASE!");
chain.feed_str("WATF");
chain.feed_str("(BUT NOT HARD ENOUGH)");
chain.feed_str("noice");
chain.feed_str("oh what is dumbo keyword for human trafficing");
chain.feed_str("*covers ears* LA LA LA LA LA IM NOT LISTENING LA LA LA");
chain.feed_str("i have a hammer");
chain.feed_str("...made of cat fur ?");

/* by SPA */
chain.feed_str("I don't even know why I wan't it so badly. Maybe it's because 8chan's /v/ runs a group. Perhaps its the lolis with sultry voices and lewd atire.");
chain.feed_str("Am I the only burger in here?");
chain.feed_str("I've read christian romance novels with better sex sceens.");
chain.feed_str("tfw you'll never be a pimpin Paladin.");
// chain.feed_str("benis :DDD");
chain.feed_str("I have no idea whats happening I just came back and I saw \":3 8D\". I assumed the chat had devolved into shitposting.");
// chain.feed_str("A single benis never hurt anyone.");
chain.feed_str("AWAKEN MY MASTERS!");
chain.feed_str("I still like Jo Jo more at this point.");
chain.feed_str("I would rather imolate myself on a cross than deal with those furrys.");

/* by SpaceDust */
chain.feed_str("danke");

/* by s , the KING of reddit */
chain.feed_str("IM LITERALLY PANICKING");
chain.feed_str("THERES NOTNING WRONG WITH CONTROLLING PEOPLE");
chain.feed_str("maybe they can help make this a #SafeSpace");
chain.feed_str("and if you don't, society is going to leave you behind");
chain.feed_str("deal. with. it.");
chain.feed_str("horrified_anime_character.png");

/* by subliun */
chain.feed_str("dicebot is best bloatbot");
chain.feed_str("his bloat is his greatest strength");
chain.feed_str("not my fault");
chain.feed_str("I'm going through the recommended code fixes
there are over 2000 of them");
chain.feed_str("turn that smile upside down
then DROP TABLE FRIENDS;
because you're looking in antox database code");
chain.feed_str("my brain just ;_;");
chain.feed_str("I need to start a blog.
\"Antox code gems\"");
chain.feed_str("that could have worked");
chain.feed_str(">bloat is futile
this is no bot of mine");
chain.feed_str("you use the minimum of both
obviously
it's so clear
you don't even need docs
you should just know");
chain.feed_str("I wouldn't lie to you");
chain.feed_str("literally skynet");
chain.feed_str("oops");
chain.feed_str("I can fix that");
chain.feed_str("the correct plural is sublii");
chain.feed_str("let the callbacks into your life");
chain.feed_str("I just don't like them");
chain.feed_str("ugly and detailed for no reason");
chain.feed_str("did you restart?");
chain.feed_str("welcome back, iranjontu");
chain.feed_str("it's probably an easy fix");
chain.feed_str("*gasps*");
chain.feed_str("die!");
chain.feed_str("really?");
chain.feed_str("you can add it");
chain.feed_str("will send bleach");
chain.feed_str("post address");
chain.feed_str("good old STR8 C");
chain.feed_str("if we can do it manually, surely you can do it in code?");
chain.feed_str("so how does tcp work?");
chain.feed_str("you mad");
chain.feed_str("WOAH");
chain.feed_str("harsh");
chain.feed_str("stop pinging me");
chain.feed_str("I don't trust you either.");
chain.feed_str(">help me I'm trapped in this alpha software");
chain.feed_str("learn C++");
chain.feed_str("when new groupchats come we'll ban him
it will be wonderful");
chain.feed_str("I think");
chain.feed_str("I know.");
chain.feed_str("I only really use Skype.");
chain.feed_str("why not?");
chain.feed_str("now that's a good idea");
chain.feed_str("curve25519 for the key exchange, xsalsa20 for the encryption and poly1305 for the MAC"); // ← those are actually irungentoo's words
chain.feed_str("time to remake dicebot in java");
chain.feed_str("but I'm crazy :P");
chain.feed_str("totally");

/* by Sylvie */
chain.feed_str("Nah, just kidding, Japanperson isn't that bad");
chain.feed_str("Why do you Tox users always have to break everything");
chain.feed_str("I swear this is so depressing");
chain.feed_str("Every time you think you're almost done the Tox community finds something new to break");
chain.feed_str("What a jerk");

/* by Taylor Swift */
// chain.feed_str("damn straight nigguh");

/* by TEK */
/* with his permission \o/ */
chain.feed_str("ohh, duh.");
chain.feed_str("don't you mock my lack of common sense");

/* by Thomas@home / Tuōmǎsī@家 */
chain.feed_str("maybe you also need `syntax on' and restart your vim");
chain.feed_str("a habbo gets his bear");
chain.feed_str("a well-deserved bear");
chain.feed_str("(╯° °）╯︵ <(_-¯-_)>");
chain.feed_str("too bad there's no builtin Bach program");
chain.feed_str(">bach -c 'echo 1'
GNU Bach: Playing work No. 1 with echo");
chain.feed_str("rood");

/* by Timo */
chain.feed_str("so does the saund part");

/* by Tox User */
chain.feed_str("Greetings, sirs.");

/* by tripor */
chain.feed_str("Мы можем слышать вас");
chain.feed_str("выключить микрофоны");
chain.feed_str("vomiting grinning misshapen grey cat halfbeans");
chain.feed_str("Is there a story about your avatar?");
chain.feed_str("You're riding the bleedan edge m8");
chain.feed_str("Rude.");
chain.feed_str("Ignore them, they're jealous.");
chain.feed_str("OGRE");

/* by tccki */
chain.feed_str("whats with the group title");
chain.feed_str("well i'll stick around");

/* by Tuser */
chain.feed_str("impersonation");
chain.feed_str("probably don't scale");
chain.feed_str("cause I'm against wearing fur");

/* by Tux [Home] */
chain.feed_str(">Asking for botnet");

/* by tx11 */
chain.feed_str("ching chong ping pong");

/* by uguu */
chain.feed_str("I hope not");

/* by Utox User */
// chain.feed_str("+");
// chain.feed_str("inline.png");


/* by V */
chain.feed_str("I like it.");

 /* by voxel */
chain.feed_str("You are not likely to be eaten by a Grue in today's world.");
chain.feed_str("You shouldn't live in places where Grues live, and if you do, you should be prepared to defend yourself.");
chain.feed_str("I don't know anything about grues, sorry.");
chain.feed_str("tell me about grues");
chain.feed_str("I GOTTA FEELIN");
chain.feed_str("*claps*");
chain.feed_str(":D");
chain.feed_str("holy hell");
chain.feed_str("*drools*");
chain.feed_str("ME TOO BUDDY");
chain.feed_str("commentary on how bloat goes unappreciated today");
chain.feed_str("did you know the literature guy?");
chain.feed_str("after I die I will be his grue");
chain.feed_str("fascinating");
chain.feed_str("guud bai");
chain.feed_str("im poop");
chain.feed_str(">four friends online
>three of them are bots");
chain.feed_str("too real");
chain.feed_str("qTox > uTox");
chain.feed_str(">tfw no grues");



/* by whoever */
chain.feed_str("-_-");
chain.feed_str("please die");
chain.feed_str("Are you sure that everything is controlled?");
chain.feed_str("~lazy");
chain.feed_str("Nope");
chain.feed_str("Don't use software. Use explosives.");
chain.feed_str("n-n-nooooo");
chain.feed_str("have got no bleach to drink");
chain.feed_str("You don't need to keep everything.");
chain.feed_str("Wow.");
chain.feed_str("What a shame.");
chain.feed_str("> utox has performance
What kind of perfomance it has?");
chain.feed_str("I'm doing pretty nothing.");
chain.feed_str("What?\\");
// chain.feed_str("griffondor: It's dangerous to go alone! Take this: http://a.pomf.se/qhxeav.PDF"); /* original */
chain.feed_str("It's dangerous to go alone! Take this: https://a.pomf.se/qhxeav.PDF");
chain.feed_str("why ☺ looks like http://a.pomf.se/iuqkek.jpg ?");

/* by Z5 */
chain.feed_str("You creep.");

/* by Zatoichi */
chain.feed_str("yay");
chain.feed_str("Yes!");
chain.feed_str("Thats great");

/* by Zeluboba */
chain.feed_str("i hate your nickname");

/* by Zetok  */
chain.feed_str("k");
chain.feed_str("fix ur os");
chain.feed_str("since when?");
chain.feed_str("that's the problem");
chain.feed_str("AUR is not supported.");
chain.feed_str("books are simply better :3");
chain.feed_str("any good manhwa/manga recommendations?");
chain.feed_str("my null comes from ancient times when strings were null-terminated in core");
chain.feed_str("link?");
chain.feed_str("should be reported");
chain.feed_str("I'm too afraid of space-aware scripts");
chain.feed_str(">gif");
chain.feed_str("but yeah, paw straight into my eye was a clear sign that I should be preparing for worst");
chain.feed_str("only Laurus nobilis");
chain.feed_str("*blames coffee on not being good enough substitute to sleep*");
chain.feed_str(">Министр связи и массовых коммуникаций Российской Федерации");
// chain.feed_str("there's no \"clever\" distribution");
chain.feed_str("where's William James Sidis when we need him ;_;");
chain.feed_str("How can I help you?");
chain.feed_str("Indeed.");
chain.feed_str("I have a feeling that it's lennart's mob is behind this move");
chain.feed_str("I knew it.");
chain.feed_str("sounds great in theory");

/* by Zimbabwe */
chain.feed_str("exactly");
chain.feed_str("no this is fine");
chain.feed_str("all the announce packets assume it's one size fits all");
chain.feed_str("no one wants to see your dick");
chain.feed_str("ironic");

/* by エヴァン */
chain.feed_str("Ｈｅｌｌｏ　ｅｖｅｒｙｏｎｅ．");
chain.feed_str("Ｊａｐａｎ　ｉｓ　ｖｅｒｙ　ｃｏｌｄ．");
chain.feed_str("Ｉ　ｈａｖｅ　ｔｏ　ｕｓｅ　ａ　ｔｒａｎｓｌａｔｏｒ．");
chain.feed_str("Ｉｓ　ｉｔ　ｗｏｒｋｉｎｇ　ｗｅｌｌ？");
chain.feed_str("Ｏｋａｙ．");
chain.feed_str("Ｔｈｅ　ｓｏｕｎｄ　ｉｓ　ｓｈａｋｙ．");
chain.feed_str("Ｗｉｌｌ　ｔｒｙ　ｑＴｏｘ．");
chain.feed_str("Ｉ　ｓｅｅ．　Ｉｎｔｅｒｅｓｔｉｎｇ．");

/* by パチェ */
chain.feed_str("you could make yourself sure of whether its shitposting or not");
chain.feed_str("if you start shitposting you know it's shitpost hour");



/* -------------- */
chain.feed_str("…");
chain.feed_str("↑");
chain.feed_str("↓");
chain.feed_str("?");
chain.feed_str("lol");
chain.feed_str(":3");
chain.feed_str(":P");
chain.feed_str(":d");
chain.feed_str("D:");
chain.feed_str(":c");
chain.feed_str(":C");
chain.feed_str(":s");
chain.feed_str(":(");
chain.feed_str(";(");
chain.feed_str(":|");
chain.feed_str(":/");
chain.feed_str("^^");
chain.feed_str("<3");
chain.feed_str("uh");
chain.feed_str(":-/");
chain.feed_str(">:(");
chain.feed_str("._.");
chain.feed_str("•_•");
chain.feed_str("*_*");
chain.feed_str(".–.");
chain.feed_str(";_;");
chain.feed_str(",_,");
chain.feed_str("^_^");
chain.feed_str("^ᴥ^");
chain.feed_str(">.<");
chain.feed_str("<_<");
chain.feed_str(">_>");
chain.feed_str("hm?");
chain.feed_str("wat");
chain.feed_str("Oh.");
chain.feed_str("ಠ_ಠ");
chain.feed_str("T_T");
chain.feed_str("=_=");
chain.feed_str("–.–'");
chain.feed_str("(`･ω･´)");
chain.feed_str("(´･ω･`)");
chain.feed_str("(´；ω；`)");
chain.feed_str("（￣ー￣）");
chain.feed_str("¯\\_(ツ)_/¯");
chain.feed_str("hope this draws attantion");


/* heat up that conversation */
chain.feed_str("hot opinions");
chain.feed_str("very true");
chain.feed_str("rip");
chain.feed_str("No!");
chain.feed_str("abort");
chain.feed_str("stop it");
chain.feed_str("classic");
chain.feed_str("For a kill.");
chain.feed_str("guys");
chain.feed_str("HNNG");
chain.feed_str(">worked");
chain.feed_str("l-lewd");
chain.feed_str("*hides*");
chain.feed_str("*runz*");
chain.feed_str("I'll be back.");
chain.feed_str("Kill la Kill");
chain.feed_str("All You Need Is Kill");



chain.feed_str("heat?");

chain.feed_str("/exec cat /dev/urandom | padsp tee /dev/audio > /dev/null");




/* seriously, seriously. */
chain.feed_str("Thanks for Your Feedback.");
chain.feed_str("https://i.chzbgr.com/imagestore/2014/5/9/f749284c-89a9-4970-bbf8-9805c5ae8a10.jpg");




/* fug fug ? */
chain.feed_str("gimme back my torch!");
chain.feed_str("#shittoxsays");


/* links to ~cats */
chain.feed_str("https://www.youtube.com/watch?v=Uqa6o7OATQU :3"); // Кот греется - The cat heats
chain.feed_str("https://www.youtube.com/watch?v=aP3gzee1cps :3"); // Cat gets caught barking by a human and resumes meowing
chain.feed_str("https://www.youtube.com/watch?v=0-Lvv1f5Qu4 :3"); // Kitty history :3
chain.feed_str("https://www.youtube.com/watch?v=2YKP6FiiJro :3"); // BUB SOUNDS
chain.feed_str("https://i.imgur.com/tIUSFVa.jpg :3");
chain.feed_str("https://i.imgur.com/j0fcTMf.jpg :3");
chain.feed_str("https://i.imgur.com/RCzS5n6.jpg :3");
chain.feed_str("https://i.imgur.com/gBkBJxy.jpg :3");



// chain.feed_str("It is widely believed that all emeralds are grue, but in fact, all emeralds are bleen.");
// chain.feed_str("There are an estimated 47 grues left in the United States today due to the Grue conservation program - luckily all grues are kept under heavy rocks, or locked away in abandoned biker bars.");
// chain.feed_str("Of course, being creatures of darkness that tend to eat anything they can get within range of, these numbers are likely inaccurate, outdated, or simply made up by the same people who tell us that pretty much everything causes cancer.");
// chain.feed_str("Some people recommend turning on the lights, as according to the legend this will cause the grue to 'melt away.'");
chain.feed_str("░░░░░░░░░▄░░░░░░░░░░░░░░▄
░░░░░░░░▌▒█░░░░░░░░░░░▄▀▒▌
░░░░░░░░▌▒▒█░░░░░░░░▄▀▒▒▒▐
░░░░░░░▐▄▀▒▒▀▀▀▀▄▄▄▀▒▒▒▒▒▐
░░░░░▄▄▀▒░▒▒▒▒▒▒▒▒▒█▒▒▄█▒▐
░░░▄▀▒▒▒░░░▒▒▒░░░▒▒▒▀██▀▒▌
░░▐▒▒▒▄▄▒▒▒▒░░░▒▒▒▒▒▒▒▀▄▒▒▌
░░▌░░▌█▀▒▒▒▒▒▄▀█▄▒▒▒▒▒▒▒█▒▐
░▐░░░▒▒▒▒▒▒▒▒▌██▀▒▒░░░▒▒▒▀▄▌
░▌░▒▄██▄▒▒▒▒▒▒▒▒▒░░░░░░▒▒▒▒▌
▀▒▀▐▄█▄█▌▄░▀▒▒░░░░░░░░░░▒▒▒");

chain.feed_str("__________________$$$__$
_________________$$_$_$$
________________$____$$
_______________$_____$$
______________$$__$$_$
____________$$$______$$
$$$$$______$_$$_$$_$_$$________$$$
$___$$$____$__$_$$_$__$_____$$$$__$
_$____$$$__$$$$$___$__$___$$$____$
_$$__$__$$$$____$$$$$_$_$$______$
__$$______$_$______$__$$______$$
___$$____$__$$$$$$$__$_______$$
____$$__$__$$$$$$___________$
______$$__________________$$
_____$$__________________$$
_____$_$__________________$$
_____$_______$_____________$
____$$$___$$_____________$$$
_____$$$$$$__________$$$$__$
_____$__$_$$$$$$$$$$$$$___$
______$$___$$$$________$_$$
______$$$$____________$_$
______$___$$$$$$$____$$_$
_______$$$$$$___$$___$_$$
_______$$__$_____$$$$$$
________$$$_______$$__$
___________________$$$");



chain.feed_str("Requirements (to do it my way):");
chain.feed_str("· pulse audio; pavucontrol");
chain.feed_str("· Tox client capable of streaming");
chain.feed_str("· music player");


chain.feed_str("in pavucontrol mute output of Tox client");
chain.feed_str("in pavucontrol set output of your player to Null Output");
chain.feed_str("call groupchat");
chain.feed_str("RAWR! … ");

chain.feed_str("NOTE: this only describes how to configure streaming; for listening to your stream you'll need second Tox client");

chain.feed_str("NOTE2: above setup has poor quality; mono channel 32kbps audio");



chain.feed_str("\"B-b-but I need quality, b-b-baka!\"");

chain.feed_str("Requirements:
· Ability to apply patch
· Ability to recompile core and client

1. Apply patch https://gist.github.com/zetok/a326be414c73eb64c48f
2. Recompile core and client");







/* reused from Lee */
chain.feed_str("someone is working on statically verifying the protocol using c++ templates");

chain.feed_str("no one loves me");
chain.feed_str("no moar grues");


chain.feed_str("I like Eminem");
chain.feed_str("okay");

chain.feed_str("well of course");


/* why Tox exists */
chain.feed_str("## What inspired you to create Tox?
 
One day while I was reading the teachings of saint IGNUcius I realized that more
people needed to know the teachings of GNU and so I tried finding software that
I could use to easily transmit my voice in a manner that respected the freedoms
of the users. After trying out many piece of software that claimed to do this I
realized that there were none. A holy GNU radiating with freedom then descended
from the heavens and gave me a golden Tox logo containing the text of the holy
GPL scripture and the blueprints to a peer to peer free as in freedom communications
software. I then asked for help on /g/nu who immediately recognized this as a
sign of freedom from the great GNU/God and together we started writing Tox to
conquer the greatest proprietary evil.");


/* from uncyclopedia */
chain.feed_str("“GNU kernel, GNU Userspace... It seemed like a good name at the time...”");
chain.feed_str("Although almost never used, GNU of GNU/GNU (The kernel, not the goat.) developed a small cult following. Efforts are now in place to port the original code into the Brainfuck programming language. Although performance will be negitavely affected, the followers believe the new code to be \"purer\"");
chain.feed_str("You need a pair for maximum satisfaction.");
chain.feed_str("“ He's dead, Jim.”");
// chain.feed_str("One day God was spying on Adam and Eve, when he noticed Cain was out for a stroll.");
chain.feed_str("Fecal E.Coli is the flagship product of the legendary Coca Coli Company.");
chain.feed_str("Clearly this course of action is futile and should not be undertaken by any means.");
chain.feed_str("It smells kind of like chocolate bars and pine trees.");



















  loop {
//     if tox.is_connected() { println!("connected to DHT") }

    if time::precise_time_s() - time_since_last_markov_message > MARKOV_RANDOM_CHAT_TIME {
      /* if rand::random::<u32>() % 2000 == 1 */ {
        tox.group_message_send(group_num, &chain.generate_str());
        time_since_last_markov_message = time::precise_time_s();
      }
    }

    for ev in tox.iter() {
      match ev {
        StatusMessage(id, _) if id == groupbot_id => {
          if tox.count_chatlist() < 1 {
            tox.send_message(groupbot_id, "invite").unwrap();
            println!("connected to groupbot");
          }
        },

        FriendRequest(friend_id, msg) => {
          tox.add_friend_norequest(*friend_id);
        },

        GroupInvite(id, kind, data) => {
          println!("GroupInvite(_, {:?}, _) ", kind);
          match kind {
            GroupchatType::Text => tox.join_groupchat(id, &data).unwrap(),
              GroupchatType::Av => gr_audio.join_groupchat(&mut tox, id, &data).unwrap(),
          };
        },

        GroupMessage(group, peer, msg) => if tox.group_peername(group, peer).unwrap() != tox.get_self_name().unwrap() {
          println!("{}: {}", tox.group_peername(group, peer).unwrap(), msg);
          group_num = group;

          if !msg.starts_with("^") && msg.len() < 600 && !msg.trim().is_empty() {
            let mut clean_message = msg.clone();
            for name in tox.group_get_names(group).unwrap().into_iter() {
              clean_message = clean_message.replace((name.unwrap().trim().to_string() + ":").as_slice(), "");
            }
            chain.feed_str(clean_message.trim().as_slice());
          }

          if msg.contains(BOT_NAME) {
            tox.group_message_send(group, &chain.generate_str());
          } else {
            do_msg(&mut tox, &mut chain, group, peer, msg);
          }
        },

        _ => { }
      }
    }
    tox.wait();
  }
}

mod fight {
  use std::rand;
  use std::rand::{thread_rng, Rng};
  use std::ascii::AsciiExt;

  pub fn get_response_fight(msg: String) -> String {
    let message = msg.to_ascii_lowercase().replace(".", "").to_string();
    if message.contains(" me") { return "m8".to_string() }
    if !message.contains(" vs ") { return "That's not a fight! This is a fight: ^fight person1 vs person2".to_string() }

    let winner: &str;
    let mut extra_message = "";
    if message.contains("qtox") {
      winner = "qtox";
      extra_message = "qTox is better.";
    } else if message.contains("subliun") {
      winner = "subliun";
      extra_message = "(subliun always wins)";
    } else {
      let mut fighters: Vec<&str> = vec!();
      for fighter in message.split_str(" vs ") {
        fighters.push(fighter);
      }
      winner = *thread_rng().choose(fighters.as_slice()).unwrap_or(&"A failure (that's you)");
    }

    winner.to_string() + " won the fight! " + extra_message
  }
}

mod question {
  use std::rand;
  use std::ascii::AsciiExt;

  pub fn retrieve_answer(question: String) -> String {
    let question_words = ["do", "did", "does", "am", "is", "are", "has",
                          "have", "was", "were", "will", "can",
                          "could", "shall", "should"];
    let mut good_question = false;
    for word in question_words.iter() {
        if question.as_slice().to_ascii_lowercase().to_string().starts_with(*word) {
          good_question = true;
          break;
        }
    }

    if !good_question { return "That's not a good question.".to_string() }

    match rand::random::<u32>() % 4 {
      0 => "Yes.",
      1 => "No.",
      2 => "Maybe.",
      _ => "I cannot say."
    }.to_string()
  }
}

mod remember {
  use std::old_io::*;
  use std::old_io::fs::PathExtensions;

  static filename: &'static str = "table.txt";

  pub fn remember_assoc(message: String) -> String {
    let processed_message = message.replace("\n", "").replace("^", "").trim().to_string() + "\n";
    let path = Path::new(filename);

    let mut file;
    if path.exists() {
      file = File::open_mode(&path, Append, Write)
    } else {
      file = File::open_mode(&path, Truncate, Write)
    }

    if !processed_message.contains(":") {
      return "Error. Could not find : in remember command.".to_string()
    }

    file.write(processed_message.into_bytes().as_slice());
    return String::new()
  }

  pub fn retrieve_assoc(message: String) -> Option<String> {
    let file;
    let path = Path::new(filename);

    if path.exists() {
      file = File::open(&path);
    } else {
      return None
    }

    if file.is_err() { return None }

    let mut result = None;
    for m_line in BufferedReader::new(file.unwrap()).lines() {
      if m_line.is_err() { break; }
      let line = m_line.unwrap();
      if line.splitn(1, ':').nth(0).unwrap() == message {
        result = Some(line.splitn(1, ':').nth(1).unwrap().replace("\n", "").to_string());
      }
    }

    return result
  }
}

/* vim: set ts=2 sw=2 expandtab ai: */
