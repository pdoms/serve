mod cli;
mod server;

use crate::cli::Cli;
use crate::server::Serf;
fn main() {
    let cli = Cli::default();
    let mut serf = Serf::new(&cli.address, cli.index).unwrap();

    serf.serve();

    



}
