use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::{net::TcpListener, io::AsyncWriteExt};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx,_rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            let mut username = String::new();

            writer.write_all("Enter your username : ".as_bytes()).await.unwrap();
            reader.read_line(&mut username).await.unwrap();
            username.pop();
            username.pop();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        let msg_to_send = "[".to_owned() + &username + "]: " + &line;

                        tx.send((msg_to_send,addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, client_addr) = result.unwrap();

                        if client_addr != addr { 
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }

                
            }
        });
        
    }
    
}