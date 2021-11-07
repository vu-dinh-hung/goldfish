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
fn connectToServer(ipAddress: &String, port: &u32) -> ServerContext {}
fn writeToContext(data: Value, context: ServerContext) -> Result<u32, u32> {}
fn pushToServer(context: ServerContext) -> Result<u32, u32> {}
