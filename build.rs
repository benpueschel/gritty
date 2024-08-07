use clap::CommandFactory;

static MAN_DIR: &str = "man";
use gritty_clap::Args;

fn main() {
    // Generate gritty manpages on build
    let _ = std::fs::remove_dir_all(MAN_DIR);
    std::fs::create_dir_all(MAN_DIR).expect("Failed to create man dir");
    clap_mangen::generate_to(Args::command(), MAN_DIR).expect("Failed to generate manpages");
}
