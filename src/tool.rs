// use std::process::exit;
// use tokio::io::{BufReader, AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpStream;
// // use tokio::sync::mpsc::Receiver;
// use dioxus::prelude::{UnboundedReceiver, UseRef};
// use futures_util::stream::StreamExt;
// // use std::sync::mpsc;

// pub async fn coroutine_handle(mut rx: UnboundedReceiver<String>, messages:&UseRef<Vec<String>>){
//   let mut client = TcpStream::connect("localhost:3000").await.unwrap();
//   tokio::spawn(async move{
//     let (reader, mut writer) = client.split();
//     let mut reader = BufReader::new(reader);
//     let mut buffer = [0;64];
    
//     loop{
//       tokio::select! {
//         result = reader.read(&mut buffer) => {
//           messages.write().push(String::from_utf8_lossy(&buffer).to_string());
//           buffer = [0;64];
//           if let Err(_) = result{
//             println!("Connection Broked");
//             exit(0);
//           };
//         }
//         message = rx.next() => {
//           match message {
//             Some(msg) => {
//               writer.write_all(msg.clone().as_bytes()).await.expect("Error writing to stream");
//             },
//             None => {
//               println!("Error");
//             }
//           }
//         }
//       }
//     }
//   }).await.unwrap();
// }
// pub async fn send_message(messages:){

// }