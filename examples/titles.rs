use std::io::{Read, Result};

use twjsrs::{RawTiddler, Tiddler};

fn stdin_string() -> Result<String> {
    let mut stdin_str: String = String::new();
    std::io::stdin().read_to_string(&mut stdin_str)?;
    Ok(stdin_str)
}

fn main() -> Result<()> {
    let stdin = stdin_string().expect("failed to read stdin");
    let tiddlers: Vec<RawTiddler> = serde_json::from_str(&stdin).unwrap();
    for raw_tiddler in tiddlers {
        let tiddler = Tiddler::from_raw(raw_tiddler).expect("Failed to parse tiddler");
        println!("{}", tiddler.title);
    }
    Ok(())
}
