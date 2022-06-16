use std::io::{self, Bufreader, BufWriter};
use std::net::{SocketAddr, TcpListener, TcpStream};

use structopt::StructOpt;

fn main() -> io::Result<()> {
    let args = Args::from_args();

}