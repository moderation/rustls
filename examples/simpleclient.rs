use std::sync::Arc;

use std::net::TcpStream;
use std::io::{Read, Write, stdout};

extern crate rustls;
extern crate webpki_roots;

fn main() {
    let mut config = rustls::ClientConfig::new();
    config.root_store.add_trust_anchors(&webpki_roots::ROOTS);

    let mut sess = rustls::ClientSession::new(&Arc::new(config), "localhost");
    let mut sock = TcpStream::connect("localhost:8443").unwrap();
    let mut tls = rustls::Stream::new(&mut sess, &mut sock);
    tls.write(concat!("GET / HTTP/1.1\r\n",
                      "Host: google.com\r\n",
                      "Connection: close\r\n",
                      "Accept-Encoding: identity\r\n",
                      "\r\n")
              .as_bytes())
        .unwrap();

    let mut plaintext = Vec::new();
    tls.read_to_end(&mut plaintext).unwrap();
    stdout().write_all(&plaintext).unwrap();
}
