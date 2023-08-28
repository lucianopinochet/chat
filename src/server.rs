
use tokio::{
	net::TcpListener, 
	sync::broadcast,
	io::{ BufReader, AsyncWriteExt, AsyncReadExt},
	};
pub async fn main(addr:&str){
	println!("server starting");
  let listener = TcpListener::bind(addr).await.unwrap();
	println!("server started");
	let (tx, _rx) = broadcast::channel(10);
	loop{
		let (mut stream, socket_addr) = listener.accept().await.unwrap();
		let tx = tx.clone();
		let mut rx = tx.subscribe();
		tokio::spawn(async move {
			let mut name = [0;16];
			stream.read(&mut name).await.unwrap();
			let name = String::from_utf8_lossy(&name).to_string();
			println!("{name} Connected.");
			let (reader, mut writer) = stream.split();
			let mut reader = BufReader::new(reader);
			let mut line = [0;64];
			loop {
					tokio::select! {
						result = reader.read(&mut line)=> {
							if let Err(_) = result{
								println!("Connection lost with {name}");
								break;
							}						
							if line.len() == 0{
								break;
							}
							let anw = format!("{}: {}", &name, String::from_utf8_lossy(&line));
							println!("{anw}"); 
							tx.send((anw.clone(), socket_addr)).unwrap();
							line = [0;64];
						}
						result = rx.recv() => {
							let (msg, other_addr) = result.unwrap();
							let msg = msg.trim_end_matches('\n');
							if other_addr == socket_addr {
							} 
							if other_addr != socket_addr {
								let msg = msg.as_bytes();
								writer.write_all(msg).await.unwrap();
							}
						}
					}
			}
		});
	}
}
