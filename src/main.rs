use std::fmt::Error;
use tokio::main;

mod cmd;
mod http;
mod tester;

#[main]
async fn main() -> Result<(), Error> {
    println!("Hello from Trebuchet!");
    Ok(())
}
