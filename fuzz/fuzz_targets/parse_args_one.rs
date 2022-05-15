#![no_main]
use libfuzzer_sys::fuzz_target;
use clap::Parser;
use std::ffi::OsString;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fuzz_target!(|data: &[u8]| {
    match std::str::from_utf8(data) {
        Ok(s) => {
            let args_iter: Vec<OsString> = s.split_whitespace()
                .filter(|sub| sub.len() > 0)
                .filter_map(|sub| OsString::from_str(sub).ok())
                .collect();
            for string in args_iter.iter() {
                assert!(string.len() > 0, "OsString is empty");
            }
            let _ = Args::try_parse_from(args_iter);
        },
        _ => {},
    }
});
