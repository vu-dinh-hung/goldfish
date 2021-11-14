// Crate for serializing
use toml::Value;

/* Public Struct */
// Data structure to interact with the server
pub struct ServerContext {
    ipAddress: String,
    port: u32,
    data: Value,
}

/* Internal methods */
fn start_server() -> Result<u32, u32> {
    todo!()
}
fn connect_to_server(ipAddress: &String, port: &u32) -> ServerContext {
    todo!()
}
fn write_to_context(data: Value, context: ServerContext) -> Result<u32, u32> {
    todo!()
}
fn push_to_server(context: ServerContext) -> Result<u32, u32> {
    todo!()
}
