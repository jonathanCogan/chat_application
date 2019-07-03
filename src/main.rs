//! Hello!
//!
//! THis is my crate.

mod server;
mod client;

// This blocking implementation of our messaging app is probably
// more efficient than a non-blocking one, however it comes with
// the downside that the server must create two threads per user
// connected, and we need to clone the tcp socket.
//     On the client side we have two separate threads, one reading
// from stdin and writing to the tcp socket and one reading from
// the tcp socket and writing to stdout. On the server side, the
// main thread spawns off two threads for each connecting client,
// one to read from the socket and write to the broadcast channel,
// and one to read from the broadcast channel and write to the
// socket. The entire server has only one broadcast thread which
// manages the broadcast channel.
//
//           Client:                     Server:
// stdin -----> thread1 -----|  |-----> s_thread1 ------|
//                           v  |                       v
//                          socket                  bc_thread
//                           |  ^                       |
// stdout <---- thread2 <----|  |------ s_thread2 <-----|

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "chat", about = "A chat application")]
struct Opt {
    name: String
}

/// Hello!
///
/// **This is code!**
///
/// ```rust
/// let x = 3;
/// ```
// Goodbye!
pub fn main() {
    let opt = Opt::from_args();
    if opt.name == "server" {
        server::run();
    } else {
        client::run(opt.name);
    }
}

/// ```rust
/// let x = 3;
/// ```
pub fn foo() {

}
