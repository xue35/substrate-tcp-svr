use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn pong(mut stream: TcpStream) {
    // 定义256长度来接收数据
    let mut buf = [0 as u8; 256];

    // 练习目的，这里就只读一次数据了。内容超过256长度的内容不予考虑哈
    match stream.read(&mut buf) {
        Ok(size) => {
            // 读取成功，打印一行垃圾信息。
            println!("Received: {}", std::str::from_utf8(&buf).expect("valid utf8"));
            // 直接全部回写。
            stream.write(&buf[0..size]).unwrap();
        }
        Err(_) => {
            // 读取数据错误，打印一行信息
            println!("Error happened, stop here.");
            // 关闭TCP链接
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }
}

fn main() -> std::io::Result<()> {
    
    // 打印一行信息
    println!("Listening on port 1080");
    // 创建一个绑定在1080端口的TCP侦听服务
    let listener = TcpListener::bind("127.0.0.1:1080")?;

    // 接收入扣TCP链接，发送给pong函数去处理
    for stream in listener.incoming() {
            pong(stream?);
    }
    
    Ok(())
}
