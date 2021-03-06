use std::io::Write;
use std::ffi::OsStr;
use term;

#[allow(dead_code)]
pub fn print_message(msg: &str, color: term::color::Color) {
    let term_stdout = term::stdout();

    if let Some(mut terminal) = term_stdout {
        let _ = terminal.fg(color);
        println!("{}", msg);
        let _ = terminal.reset();
    } else {
        println!("{}", msg);
    }
}

fn red(s: &str) {
    let mut term_stderr = term::stderr();
    term_stderr.as_mut().map(|t|{
        let _ = t.attr(term::Attr::Bold);
        let _ = t.fg(term::color::RED);
    });
    let _ = write!(::std::io::stderr(), "{}", s);
    let _ = term_stderr.map(|mut t| t.reset());
}


pub fn report_error(e: &super::Error) {
    red("error:");
    let _ = writeln!(::std::io::stderr(), " {}", e);
    for e in e.iter().skip(1) {
        red("  caused by:");
        let _ = writeln!(::std::io::stderr(), " {}", e);
    }
}

/// The default target to pass to cargo, to workaround issue #11.
#[cfg(target_os="macos")]
pub fn default_target() -> &'static OsStr {
    OsStr::new("x86_64-apple-darwin")
}

/// The default target to pass to cargo, to workaround issue #11.
#[cfg(not(target_os="macos"))]
pub fn default_target() -> &'static OsStr {
    OsStr::new("x86_64-unknown-linux-gnu")
}
