use mailbox::Mailbox;
use std::error::Error;

mod mailbox;

fn main() -> Result<Mailbox, Box<dyn Error>> {
    let mb = Mailbox::new();
}
