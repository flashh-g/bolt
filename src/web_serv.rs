use crate::args::Arg;
use crate::config::Config;
use clap::Parser;
use colored::Colorize;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

async fn process_socket_and_fmt_html(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let status_line = if request_line == "GET / HTTP/1.1" {
        "HTTP/1.1 200 OK"
    } else {
        "HTTP?1.1 404 NOT FOUND"
    };

    let arg = Arg::parse(); // parse command line arg

    let content = fs::read_to_string(arg.entry_point).unwrap(); //entry point = index.html or something
    let length = content.len();
    let response = format!("{status_line}\r\n{length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
pub async fn bind_to_ip_addr() {
    let config = Config::set_config(); //accessing config.toml feilds
    let ip = config.ip; // config.toml user set ip value
    let port = config.port.to_string(); // config.toml user set port value
    let result = ip.to_string() + &":".to_string() + &port.as_ref();
    let listener = TcpListener::bind(&result).unwrap();
    let listen_on = "Listening on";

    print!("\n\n");
    println!(
        "{}\n\n{}",
        listen_on.bright_yellow().bold(),
        result.to_string().bright_green()
    );

    loop {
        // event_loop
        let (stream, _) = listener.accept().unwrap();
        tokio::spawn(async move {
            //spawn thread
            process_socket_and_fmt_html(stream).await;
        });
    }
}
