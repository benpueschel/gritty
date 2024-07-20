pub fn is_color() -> bool {
    #[cfg(feature = "color")]
    // only colorize if NO_COLOR is not set and stdout is a tty
    return std::env::var("NO_COLOR").is_err() && atty::is(atty::Stream::Stdout);
    #[cfg(not(feature = "color"))]
    return false;
}

pub fn important(msg: &str) {
    if is_color() {
        #[cfg(feature = "color")]
        color::important(msg);
    } else {
        no_color::important(msg);
    }
}
pub fn highlight(prefix: &str, highlight: &str, suffix: &str) {
    print(prefix);
    info(highlight);
    println(suffix);
}
pub fn info(msg: &str) {
    if is_color() {
        #[cfg(feature = "color")]
        color::info(msg);
    } else {
        no_color::info(msg);
    }
}
pub fn alt_info(msg: &str) {
    if is_color() {
        #[cfg(feature = "color")]
        color::alt_info(msg);
    } else {
        no_color::info(msg);
    }
}
pub fn warning(msg: &str) {
    if is_color() {
        #[cfg(feature = "color")]
        color::warning(msg);
    } else {
        no_color::warning(msg);
    }
}
pub fn print(msg: &str) {
    if is_color() {
        #[cfg(feature = "color")]
        color::print(msg);
    } else {
        no_color::print(msg);
    }
}
pub fn println(msg: &str) {
    if is_color() {
        #[cfg(feature = "color")]
        color::println(msg);
    } else {
        no_color::println(msg);
    }
}
pub fn end_line() {
    println!();
}

#[cfg(feature = "color")]
mod color {
    use ansi_term::Color::*;
    pub fn important(msg: &str) {
        print!("{}", Red.bold().paint(msg));
    }
    pub fn info(msg: &str) {
        print!("{}", Cyan.paint(msg));
    }
    pub fn alt_info(msg: &str) {
        print!("{}", Green.paint(msg));
    }
    pub fn warning(msg: &str) {
        print!("{}", Yellow.paint(msg));
    }
    pub fn print(msg: &str) {
        print!("{}", msg);
    }
    pub fn println(msg: &str) {
        println!("{}", msg);
    }
}

mod no_color {
    pub fn important(msg: &str) {
        print!("{}", msg);
    }
    pub fn info(msg: &str) {
        print!("{}", msg);
    }
    pub fn warning(msg: &str) {
        print!("{}", msg);
    }
    pub fn print(msg: &str) {
        print!("{}", msg);
    }
    pub fn println(msg: &str) {
        println!("{}", msg);
    }
}
