// Crate for serializing
use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use toml::Value;


#[test]
pub fn test_1_connect_ssh() {
    // Replace with IP address of remote
    let tcp = TcpStream::connect("127.0.0.1:22").unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    // Replace with login credentials
    session.userauth_password("username", "password").unwrap();
    assert!(session.authenticated());
    let mut channel = session.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().ok();
    println!("{}", channel.exit_status().unwrap());
    
}

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
// fn connect_to_server(ip_address: &String, port: &u32) -> ServerContext {
//     todo!()
// }
// fn write_to_context(data: Value, context: ServerContext) -> Result<u32, u32> {
//     todo!()
// }
// fn push_to_server(context: ServerContext) -> Result<u32, u32> {
//     todo!()
// }
