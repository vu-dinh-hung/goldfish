use ssh2::Session;
use ssh2::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use toml::Value;
use std::path::Path;

// Returns SSH connection to ip_address with specified credentials. 
// ip_address must include the port number
fn connect_to_server(ip_address: &str, username: &str, password: &str) -> Option<Session> {
    let tcp = TcpStream::connect(ip_address).unwrap();
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

// Download file at path from given server session in to current directory
fn download_from_server(session: Session, path: &str) -> Result<&str, Error> {
    let downloaded = session.scp_recv(Path::new(path));
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


// fn write_to_context(data: Value, context: ServerContext) -> Result<u32, u32> {
//     todo!()
// }
// fn push_to_server(context: ServerContext) -> Result<u32, u32> {
//     todo!()
// }



// pub fn test_2_write_file(){
// 	assert_eq!(dvcs.write_file(invalidpath, somedata), Err("InvalidPathError"))
// }
// pub fn test_3_move_file(){
// 	assert_eq!(dvcs.move_file(sourcepath, destpath, c=true), true)
// 	assert_eq!(dvcs.read_file(sourcepath), dvcs.read_file(destpath))
// }
// pub fn test_4_remove_file(){
// 	assert_eq!(dvcs.remove_file(path, r=true), true)
// 	pathprime = path + "/filename"
// 	assert_eq!(dvcs.write_file(pathprime), Err("InvalidPathError"))
// }

// /* Internal methods */
// fn start_server() -> Result<u32, u32> {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use ssh2::Session;
    use std::net::TcpStream;
    use std::io::prelude::*;

    #[test]
    pub fn test_1_connect_ssh_csug() {
        // Replace with IP address of CSUG machine
        let tcp = TcpStream::connect("127.0.0.1:22").unwrap();
        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        session.handshake().unwrap();
        // Replace with login credentials
        session.userauth_password("netid", "password").unwrap();
        assert!(session.authenticated());
        let mut channel = session.channel_session().unwrap();
        channel.exec("ls").unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        println!("{}", s);
        channel.wait_close().ok();
        println!("{}", channel.exit_status().unwrap());
        
    }
}

