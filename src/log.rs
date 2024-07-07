#[cfg(feature = "color")]
pub use color::*;

#[cfg(not(feature = "color"))]
pub use no_color::*;

macro_rules! log {
    ($($arg:tt)*) => {
        print!($($arg)*);
    };
    () => {

    };
}

#[cfg(feature = "color")]
mod color {
    use ansi_term::Color::*;
    pub fn important(msg: &str) {
        print!("{}", Red.bold().paint(msg));
    }
    pub fn highlight(prefix: &str, highlight: &str, suffix: &str) {
        print(prefix);
        info(highlight);
        println(suffix);
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
    pub fn end_line() {
        println!();
    }
}

#[cfg(not(feature = "color"))]
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
    pub fn end_line() {
        println!();
    }
}
