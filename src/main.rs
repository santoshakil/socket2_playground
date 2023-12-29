use anyhow::Result;
use socket2::{Domain, Protocol, Socket, Type};
use std::io::{Read, Write};
use std::net::SocketAddr;

fn handle_client(mut stream: Socket) -> Result<()> {
    println!("Incoming connection: {:?}", stream.peer_addr());
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let response = "HTTP/2.0 200 OK\r\n\r\nHello, world!";
    stream.write(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}

fn main() -> Result<()> {
    let socket = Socket::new(Domain::IPV4, Type::STREAM, Some(Protocol::TCP))?;
    let address: SocketAddr = "0.0.0.0:2020".parse()?;
    socket.bind(&address.into())?;
    socket.listen(128)?;

    println!("Listening on {}", address);

    loop {
        let (stream, _) = socket.accept()?;
        handle_client(stream)?;
    }
}
