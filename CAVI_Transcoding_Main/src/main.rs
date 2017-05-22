
/// rtsp_URL  =   ( "rtsp:" | "rtspu:" ) "//" host [ ":" port ] [ abs_path ]

/// RTSP URL Scheme
/// tcp: rtsp://  | udp: rtspu://

/// RTSP Port
/// Default port 554

/// For example, the RTSP URL:
/// rtsp://media.example.com:554/twister/audiotrack
/// rtspu://media.example.com:554/twister/audiotrack



pub mod status;
pub mod method;
pub mod version;
pub mod header;

pub mod request;
pub mod response;

pub mod error;

pub mod server;
pub mod client;

fn main() {
    /*
    let url;
    url = "rtsp://admin:1qaz2wsx@10.10.10.208:8555/Streaming/Channels/0101";
    let theClient = client::Rtsp::new(url);
    println!("----------------->0 {:?}", theClient); /* connection failed         .request(method::Method::Describe)    */
*/
    let stringu = format!("update");
    let r = &stringu;
    greet(r);
    greet(&r[2..5]);



}


fn greet(name: &str) {
    println!("Rustup RTSP  {}", name);
}


/*
use std::net::{TcpListener, TcpStream};


// use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};

// let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
// assert_eq!(listener.local_addr().unwrap(),
//            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)));


fn main() {

    println!("----------------->0 "); /* connection failed */
    let listener = TcpListener::bind("10.10.10.208:554")// rtsp://admin:1qaz2wsx@       /Streaming/Channels/0101
        .unwrap_or(0);



    fn handle_client(stream: TcpStream) {
        // ...
    }
    println!("----------------->1 {:?}", listener);
    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("----------------->6 ");
        match stream {
            Ok(stream) => {

                handle_client(stream);
            }
            Err(e) => {
                println!("----------------->3connection failed "); /* connection failed */
            }
        }
    }

}
*/
/*
/// trans-code-it ---<--- inNeedMoreThanAllNowMarkedAsDone

// A tiny async echo server with tokio-core
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use futures::{Future, Stream};
use tokio_io::{io, AsyncRead};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn main() {

    // Create the event loop that will drive this server
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    println!("ceva bun 0");

    // Bind the server's socket
    // rtsp://admin:1qaz2wsx@10.10.10.208/Streaming/Channels/0101
    let addr = "10.10.10.208"
        .parse() // rtsp://admin:1qaz2wsx@
        .unwrap(); // :8554
    // /Streaming/Channels/0101
    let tcp = TcpListener::bind(&addr, &handle).unwrap();

    println!("ceva bun 1");

    // Iterate incoming connections
    let server = tcp.incoming()
        .for_each(|(tcp, _)| {
            println!("ceva bun 2");
            // Split up the read and write halves
            let (reader, writer) = tcp.split();

            // Future of the copy
            let bytes_copied = io::copy(reader, writer);

            // ... after which we'll print what happened
            let handle_conn = bytes_copied
                .map(|(n, _, _)| println!("wrote {} bytes", n))
                .map_err(|err| println!("IO error {:?}", err));

            // Spawn the future as a concurrent task
            handle.spawn(handle_conn);

            Ok(())
        });

    // Spin up the server on the event loop
    core.run(server).unwrap();

    /**/
    /*
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

*/
}
*/
