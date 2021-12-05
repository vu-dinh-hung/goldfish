use ssh2::Session;
use ssh2::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use toml::Value;
use std::path::Path;

// Returns SSH connection to ip_address on port 22 with specified credentials.
fn connect_to_server(ip_address: &str, username: &str, password: &str) -> Option<Session> {
    let full_address = ip_address.to_owned() + &":22".to_string();
    let tcp = TcpStream::connect(full_address).unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    session.userauth_password(username, password).unwrap();
    if session.authenticated() {
        return Some(session);
    } else {
        return None;
    }
    
}

// Download file at filepath from given server session in to current directory
fn download_from_server(session: Session, filepath: &str) -> Result<&str, Error> {
    let downloaded = session.scp_recv(Path::new(filepath));
    match downloaded {
        Ok((mut remote_file, stat)) => {
            let mut contents = Vec::new();
            remote_file.read_to_end(&mut contents).unwrap();
        
            // Close the channel and wait for the whole content to be tranferred
            remote_file.send_eof().unwrap();
            remote_file.wait_eof().unwrap();
            remote_file.close().unwrap();
            remote_file.wait_close().unwrap();
            Ok("File downloaded.")
        },
        Err(e) => Err(e),
    }
}

// fn upload_to_server(session: Session, filepath: &str, file_size: u64) -> Result(&str, &str) {

// }

#[cfg(test)]
mod tests {
    use ssh2::Session;
    use crate::networking::*;
    use std::net::TcpStream;
    use std::io::prelude::*;

    #[test]
    pub fn test_1_connect_ssh_csug() {
       let session = connect_to_server("128.151.69.87", "uamin2", "********").unwrap();
       assert!(session.authenticated());
    }
}

