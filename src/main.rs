#[macro_use]
extern crate serde;

mod code_flow;

#[tokio::main]
async fn main() {
    code_flow::start_server_main().await;
}
