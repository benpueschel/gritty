use std::{env, path::Path};

use clap::CommandFactory;

use gritty_clap::Args;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let man_dir = Path::new(&out_dir).join("man");
    // Generate gritty manpages on build
    std::fs::create_dir_all(&man_dir).expect("Failed to create man dir");
    clap_mangen::generate_to(Args::command(), &man_dir).expect("Failed to generate manpages");
}
