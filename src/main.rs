//! **lmake_version_from_date - In cargo.toml writes the version as the date**  
//region: lmake_readme insert "readme.md"

//endregion: lmake_readme insert "readme.md"

//region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    //variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,

)]
#![allow(
    //library from dependencies have this clippy warnings. Not my code.
    //Why is this bad: It will be more difficult for users to discover the purpose of the crate, 
    //and key information related to it.
    clippy::cargo_common_metadata,
    //Why is this bad : This bloats the size of targets, and can lead to confusing error messages when 
    //structs or traits are used interchangeably between different versions of a crate.
    clippy::multiple_crate_versions,
    //Why is this bad : As the edition guide says, it is highly unlikely that you work with any possible 
    //version of your dependency, and wildcard dependencies would cause unnecessary 
    //breakage in the ecosystem.
    clippy::wildcard_dependencies,
    //Rust is more idiomatic without return statement
    //Why is this bad : Actually omitting the return keyword is idiomatic Rust code. 
    //Programmers coming from other languages might prefer the expressiveness of return. 
    //It’s possible to miss the last returning statement because the only difference 
    //is a missing ;. Especially in bigger code with multiple return paths having a 
    //return keyword makes it easier to find the corresponding statements.
    clippy::implicit_return,
    //I have private function inside a function. Self does not work there.
    //Why is this bad: Unnecessary repetition. Mixed use of Self and struct name feels inconsistent.
    clippy::use_self,
    //Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    //because then wasm-pack build --target web returns an error: export run not found 
    //Why is this bad: In general, it is not. Functions can be inlined across crates when that’s profitable 
    //as long as any form of LTO is used. When LTO is disabled, functions that are not #[inline] 
    //cannot be inlined across crates. Certain types of crates might intend for most of the 
    //methods in their public API to be able to be inlined across crates even when LTO is disabled. 
    //For these types of crates, enabling this lint might make sense. It allows the crate to 
    //require all exported methods to be #[inline] by default, and then opt out for specific 
    //methods where this might not make sense.
    clippy::missing_inline_in_public_items,
    //Why is this bad: This is only checked against overflow in debug builds. In some applications one wants explicitly checked, wrapping or saturating arithmetic.
    //clippy::integer_arithmetic,
    //Why is this bad: For some embedded systems or kernel development, it can be useful to rule out floating-point numbers.
    clippy::float_arithmetic,
    //Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
    //Why is this bad : Splitting the implementation of a type makes the code harder to navigate.
    clippy::multiple_inherent_impl,

    clippy::missing_docs_in_private_items,
    clippy::unused_imports,
)]
//endregion
#![allow(unused_imports)]

//region: use statements
use ansi_term::Colour::{Green, Red, Yellow};
use chrono::offset::Utc;
use chrono::prelude::*;
use chrono::DateTime;
use chrono::Timelike;
use chrono::{Datelike, Local};
use unwrap::unwrap;
//use ansi_term::Style;
use clap::App; //Arg
               //use glob::glob;
use filetime::FileTime;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
//endregion

#[allow(clippy::print_stdout, clippy::integer_arithmetic)]
/// The program starts here. No arguments. Linux only.
fn main() {
    //define the CLI input line parameters using the clap library
    let _matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    let current_dir = unwrap!(env::current_dir());

    println!(
        "current_dir: {}",
        Yellow.paint(unwrap!(current_dir.to_str()))
    );
    let snippets_dir = current_dir.join("pkg").join("snippets");

    //the first folder can be None
    let mut opt_first_folder: Option<PathBuf> = None;
    let mut opt_first_mtime: Option<FileTime> = None;

    //find the newer folder and remove the older folder
    for entry in fs::read_dir(snippets_dir).unwrap() {
        let entry = entry.unwrap();
        let second_folder = entry.path();
        //println!("{:?}",second_folder);
        let second_metadata = unwrap!(fs::metadata(&second_folder));
        let second_mtime = FileTime::from_last_modification_time(&second_metadata);
        //println!("{:?}",second_mtime);

        match opt_first_mtime {
            None => {
                opt_first_folder = Some(second_folder.clone());
                opt_first_mtime = Some(second_mtime.clone());
            }
            Some(first_mtime) => {
                if second_mtime > first_mtime {
                    let first_folder = opt_first_folder.unwrap();
                    println!("delete first: {:?}", first_folder);
                    std::fs::remove_dir_all(first_folder).unwrap();

                    opt_first_folder = Some(second_folder.clone());
                    opt_first_mtime = Some(second_mtime.clone());
                } else if first_mtime > second_mtime {
                    println!("delete second: {:?}", second_folder);
                    std::fs::remove_dir_all(second_folder).unwrap();
                } else {
                    println!("Error: folders have the same date?");
                }
            }
        }
    }
}
