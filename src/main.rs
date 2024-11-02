use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("绑定地址失败");

    let port: u16 = listener.local_addr().unwrap().port();
    
    println!("启动地址: http://127.0.0.1:{port}");

    run(listener)?.await

}