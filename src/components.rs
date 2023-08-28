use std::{process::exit, time::Duration};
use dioxus_router::prelude::*;
use dioxus::prelude::*;
use tokio::{net::TcpStream, sync::mpsc::Sender};
use crate::Route;
// use futures_util::stream::StreamExt;
use tokio::io::{BufReader, AsyncReadExt, AsyncWriteExt};
#[inline_props]
pub fn Chat(cx: Scope) -> Element{
  let messages = use_ref(cx, || Vec::<String>::new());
  let messages_cloned = messages.clone();
  let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(1);
  cx.spawn(async move{
    let mut client = TcpStream::connect("localhost:3001").await.unwrap();
    let (reader, mut writer) = client.split();
    let mut reader = BufReader::new(reader);
    let mut buffer = [0;64];
    loop{
      tokio::select! {
        result = reader.read(&mut buffer) => {
          messages_cloned.write().push(String::from_utf8_lossy(&buffer).to_string());
          buffer = [0;64];
          if let Err(_) = result{
            println!("Connection Broked");
            exit(0);
          };
          std::thread::sleep(Duration::from_millis(100));
        }
        message = rx.recv() => {
          println!("1: {message:?}");
          writer.write_all(message.unwrap().as_bytes()).await.unwrap_or_else(|_| println!("message not sended"));
          
        }
      }      
    };
  });
  // let tx: &Coroutine<String> = use_coroutine(cx, |mut rx:UnboundedReceiver<String>| async move {
  //   let mut client = TcpStream::connect("localhost:3001").await.unwrap();
  //   let (reader, mut writer) = client.split();
  //   let mut reader = BufReader::new(reader);
  //   let mut buffer = [0;64];
  //   loop{
  //     tokio::select! {
  //       result = reader.read(&mut buffer) => {
  //         messages_cloned.write().push(String::from_utf8_lossy(&buffer).to_string());
  //         buffer = [0;64];
  //         if let Err(_) = result{
  //           println!("Connection Broked");
  //           exit(0);
  //         };
  //         std::thread::sleep(Duration::from_millis(100));
  //       }
  //       message = rx.next() => {
  //         match message {
  //           Some(msg) => {
  //             println!("1: {msg}");
  //             writer.write_all(msg.to_string().as_bytes()).await.unwrap_or_else(|_| println!("message not sended"));
  //             println!("2: {msg}");
  //           },
  //           None => {
  //             println!("Error");
  //           }
  //         }
  //       }
  //     }      
  //   };

  // });
  let messages_lock = messages.read();
  let messages_rendered = messages_lock.iter().map(|message|{
    render!{
      li{
        class:"message-sender",
        "{message}",
      }
    }
  });
  render!{
    div{
      ul{
        class:"messages",
        messages_rendered,
      },
      SendBar{
        messages: messages,
        sender: tx
      }
    }
  }
}
#[inline_props]
fn SendBar<'a>(cx: Scope, messages: &'a UseRef<Vec<String>>, sender: Sender<String>) -> Element{
  let message = use_state(cx, || "".to_string());
	render!{
			form{
				onsubmit: move |_|{
						messages.write().push(message.get().clone());
            cx.spawn(async move{
              sender.send(message.get().clone()).await;
            });
            println!("send");
						message.set("".to_string())
				},
				prevent_default:"onsubmit",
				input{
					name:"send",
					class:"input-bar",
					value: "{message}",
					oninput: move |e| message.set(e.value.clone())
				}
				input{
					class:"input-submit",
					r#type:"submit",
          "Submit"
				}
			}
		}
}
#[inline_props]
pub fn Login(cx: Scope) -> Element{
  let name = use_state(cx, || "".to_string());
  render!{
    div{
      class:"login",
      form{
        
        prevent_default:"onsubmit",
        input{
          name:"name",
          value:"{name}",
          class:"input-bar",
          oninput: move |e|{
            name.set(e.value.clone())
          }
        }
        Link{
          to: Route::Chat{},
          class:"input-submit",
          input{
            r#type:"submit",
            "Submit"
          }
        }
      }
    }
  }
}
          // #[inline_props]
          // fn Message(cx: Scope, message:String ) -> Element{
          //   render!{
          //     li{
          //       class:"message-sender",
          //       "{message}",
          //     }
          //   }
          // }