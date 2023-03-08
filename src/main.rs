use std::env;
use chrono::prelude::*;
use clipboard::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let msg = &args[1];

    let dt: DateTime<Utc> = Utc::now(); 
    let fdt: String = format!("{}-{}-{} {}; CWS",
        dt.month(), dt.day(), dt.year(), msg);

    let mut clip_ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    clip_ctx.set_contents(fdt).unwrap();
}
