//! List installed ASIO drivers. Runs on any platform; on non-Windows the
//! list is always empty.

use maolan_asio::drivers;

fn main() {
    let list = match drivers() {
        Ok(list) => list,
        Err(e) => {
            eprintln!("error enumerating ASIO drivers: {e}");
            std::process::exit(1);
        }
    };
    if list.is_empty() {
        println!("No ASIO drivers found.");
        return;
    }
    println!("ASIO drivers ({}):", list.len());
    for d in &list {
        match &d.description {
            Some(desc) => println!("  {}  {{{}}}  ({})", d.name, d.clsid, desc),
            None => println!("  {}  {{{}}}", d.name, d.clsid),
        }
    }
}
