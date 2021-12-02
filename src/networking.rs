// Crate for serializing
use ssh2::Session;
use std::net::TcpStream;
use toml::Value;

#[test]
pub fn test_1_inspect_ssh_agent() {
    // Almost all APIs require a `Session` to be available
    let session = Session::new().unwrap();
    let mut agent = session.agent().unwrap();

    // Connect the agent and request a list of identities
    agent.connect().unwrap();
    agent.list_identities().unwrap();
    println!("+++++++++++++++++++++++++");

    for identity in agent.identities().unwrap() {
        println!("{}", identity.comment());
        let pubkey = identity.blob();
    }
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
