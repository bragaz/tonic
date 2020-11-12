use super::super::Connected;
#[cfg(feature = "tls")]
use super::TlsStream;
use hyper::server::conn::AddrStream;
use std::net::SocketAddr;
use tokio::net::TcpStream;
#[cfg(feature = "tls")]
use tokio_rustls::rustls::Session;

impl Connected for AddrStream {
    fn remote_addr(&self) -> Option<SocketAddr> {
        Some(self.remote_addr())
    }
}

impl Connected for TcpStream {
    fn remote_addr(&self) -> Option<SocketAddr> {
        self.peer_addr().ok()
    }
}

#[cfg(feature = "tls")]
impl<T: Connected> Connected for TlsStream<T> {
    fn remote_addr(&self) -> Option<SocketAddr> {
        if let Some((inner, _)) = self.get_ref() {
            inner.remote_addr()
        } else {
            None
        }
    }

    fn peer_certs(&self) -> Option<Vec<Certificate>> {
        if let Some((_, session)) = self.get_ref() {
            if let Some(certs) = session.get_peer_certificates() {
                let certs = certs
                    .into_iter()
                    .map(|c| Certificate::from_pem(c.0))
                    .collect();
                Some(certs)
            } else {
                None
            }
        } else {
            None
        }
    }
}
