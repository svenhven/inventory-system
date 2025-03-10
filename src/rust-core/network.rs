use tokio::net::TcpListener;

pub async fn start_network() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on port 8080");

    loop {
        let (_socket, _) = listener.accept().await.unwrap();
        // Handle the socket connection
    }
}