// TCP -> Below code explains how we could write an echo server that can handle one client at a time
use std::io::{Read, Result, Write};

fn main() -> Result<()> {
    let listener = std::net::TcpListener::bind("0.0.0.0:12345")?;

    // This single threaded server can handle only one incoming connection at a
    // time.
    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut buffer = [0u8; 4096];
        let count = stream.read(&mut buffer)?;
        stream.write_all(&buffer[0..count])?;
    }
    Ok(())
}

// And this is how you write and read data over a TcpStream as a client
use std::io::{Read, Result, Write};

fn main() -> Result<()> {
    let mut stream = std::net::TcpStream::connect("127.0.0.1:12345")?;

    stream.write_all(&[0, 1, 2, 3])?;

    let mut buffer = [0u8; 4];
    stream.read_exact(&mut buffer)?;
    println!("Received {buffer:?}");

    Ok(())
}

// UDP
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;

    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = [0; 10];
    let (amt, src) = socket.recv_from(&mut buf)?;

    // Redeclare `buf` as slice of the received data and send reverse data back to origin.
    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, &src)?;
    Ok(())
}