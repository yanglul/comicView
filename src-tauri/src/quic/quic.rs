
use crate::trans::*;

use anyhow::{anyhow, Result};
use clap::Parser;
use quinn_proto::crypto::rustls::QuicClientConfig;
use rustls::pki_types::CertificateDer;
use tracing::{error, info};
use url::Url;
use std::{
    fs,
    io::{self, Write},
    net::{SocketAddr, ToSocketAddrs},
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use futures_lite::{future};

pub struct QuicMsg{
    msg : String,
    ip : String,
    port : i32,
}


/// HTTP/0.9 over QUIC client
#[derive(Parser, Debug)]
#[clap(name = "client")]
struct Opt {
    /// Perform NSS-compatible TLS key logging to the file specified in `SSLKEYLOGFILE`.
    #[clap(long = "keylog")]
    keylog: bool,

    url: Url,

    /// Override hostname used for certificate verification
    #[clap(long = "host")]
    host: Option<String>,

    /// Custom certificate authority to trust, in DER format
    #[clap(long = "ca")]
    ca: Option<PathBuf>,

    /// Simulate NAT rebinding after connecting
    #[clap(long = "rebind")]
    rebind: bool,

    /// Address to bind on
    #[clap(long = "bind", default_value = "[::]:0")]
    bind: SocketAddr,
}



impl Transport for QuicMsg{
    fn send_msg(&self,msg:&str)->Result<String>{
        future::block_on( async {
            let remote = (self.ip.as_str(), self.port as u16)
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| anyhow!("couldn't resolve to an address"))?;

            let mut roots = rustls::RootCertStore::empty();
            if let Some(ca_path) = options.ca {
                roots.add(CertificateDer::from(fs::read(ca_path)?))?;
            } else {
                let dirs = directories_next::ProjectDirs::from("org", "quinn", "quinn-examples").unwrap();
                match fs::read(dirs.data_local_dir().join("cert.der")) {
                    Ok(cert) => {
                        roots.add(CertificateDer::from(cert))?;
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                        info!("local server certificate not found");
                    }
                    Err(e) => {
                        error!("failed to open local server certificate: {}", e);
                    }
                }
            }
            let mut client_crypto = rustls::ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();

            client_crypto.alpn_protocols = common::ALPN_QUIC_HTTP.iter().map(|&x| x.into()).collect();
            client_crypto.key_log = Arc::new(rustls::KeyLogFile::new());

            let client_config =
                quinn::ClientConfig::new(Arc::new(QuicClientConfig::try_from(client_crypto)?));
            let mut endpoint = quinn::Endpoint::client(options.bind)?;
            endpoint.set_default_client_config(client_config);

            let request = format!("GET {}\r\n", url.path());
            let start = Instant::now();
            let rebind = options.rebind;
            let host = options.host.as_deref().unwrap_or(url_host);

            eprintln!("connecting to {host} at {remote}");
            let conn = endpoint
                .connect(remote, host)?
                .await
                .map_err(|e| anyhow!("failed to connect: {}", e))?;
            eprintln!("connected at {:?}", start.elapsed());
            let (mut send, mut recv) = conn
                .open_bi()
                .await
                .map_err(|e| anyhow!("failed to open stream: {}", e))?;
            if rebind {
                let socket = std::net::UdpSocket::bind("[::]:0").unwrap();
                let addr = socket.local_addr().unwrap();
                eprintln!("rebinding to {addr}");
                endpoint.rebind(socket).expect("rebind failed");
            }

            send.write_all(request.as_bytes())
                .await
                .map_err(|e| anyhow!("failed to send request: {}", e))?;
            send.finish().unwrap();
            let response_start = Instant::now();
            eprintln!("request sent at {:?}", response_start - start);
            let resp = recv
                .read_to_end(usize::MAX)
                .await
                .map_err(|e| anyhow!("failed to read response: {}", e))?;
            let duration = response_start.elapsed();
            eprintln!(
                "response received in {:?} - {} KiB/s",
                duration,
                resp.len() as f32 / (duration_secs(&duration) * 1024.0)
            );
            io::stdout().write_all(&resp).unwrap();
            io::stdout().flush().unwrap();
            conn.close(0u32.into(), b"done");

            // Give the server a fair chance to receive the close packet
            endpoint.wait_idle().await;

           Ok("".to_string())
        })
        
    }
    fn download(&self,msg:&str,path:&str){

    }

}

