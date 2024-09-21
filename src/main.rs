#[tokio::main(flavor = "current_thread")]
async fn main() {
    let socket = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = socket.local_addr().unwrap();
    std::thread::spawn(move || loop {
        let (stream, peer) = socket.accept().unwrap();
        stream.shutdown(std::net::Shutdown::Both).unwrap();
        eprintln!("closed peer: {:#?}", peer);
    });

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        // .retry_canceled_requests(false)
        .build_http::<String>();

    let resp = client.get(format!("http://{addr}").parse().unwrap()).await;
    eprintln!("{:#?}", resp);
}
