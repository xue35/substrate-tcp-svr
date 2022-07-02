use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn pong(mut stream: TcpStream) {
    let mut buf = [0 as u8; 256];

    match stream.read(&mut buf) {
        Ok(size) => {
            println!("Connection received, now processing...");
            stream.write(&buf[0..size]).unwrap();
        }
        Err(_) => {
            println!("Error happened, stop here.");
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }
}

fn main() -> std::io::Result<()> {
    
    println!("Listening on port 1080");
    // 创建一个绑定在1080端口的TCP侦听服务
    let listener = TcpListener::bind("127.0.0.1:1080")?;

    // 接收入扣TCP链接，发送给pong函数去处理
    for stream in listener.incoming() {
            pong(stream?);
    }
    
    Ok(())
}
