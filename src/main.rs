use crate::web_serv::bind_to_ip_addr;

mod args;
mod config;
mod web_serv;

#[tokio::main]
async fn main() {
    bind_to_ip_addr().await;
}
