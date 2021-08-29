// Adopted from https://github.com/rust-clique/man/blob/fc11e3765a3a94e4ba6e1943eb747fd704f69a61/examples/main.rs

extern crate clap;
extern crate clap_md;

use clap::{App, AppSettings, Arg};
use clap_md::app_to_md;

fn main() {
    let a = App::new("testapp")
        .about("Pointless application")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Katharina Fey <kookie@spacekookie.de>")
        // .author("Yosh Wuyts <y@w.s")
        .long_about("Lorem Ipsum bla bla bla")
        .arg(
            Arg::new("debug")
                .short('d')
                .about("Make program output debug messages"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .takes_value(true)
                .about("Output File"),
        )
        .subcommand(App::new("foo").arg(Arg::new("bar").short('b').long("barr")));

    let markdown = app_to_md(&a, 1).unwrap();
    println!("{}", markdown);
}
