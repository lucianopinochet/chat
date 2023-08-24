
use tokio::io::{self, AsyncWriteExt, BufReader, AsyncReadExt};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{self, Sender};
use dioxus_router::prelude::*;
use tokio::net::TcpStream;
use dioxus::prelude::*;
use std::process::exit;
use crate::Route;

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
          to: Route::Chat{
            name: name.get().clone()
          },
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
#[inline_props]
pub fn Chat(cx: Scope, name:String) -> Element{
  let messages = use_ref(cx, || Vec::<String>::new());
  
  let (tx, mut rx) = mpsc::channel::<String>(10);
  let _ws: &Coroutine<()> = use_coroutine(cx, |_rx| async move {
    let mut client = TcpStream::connect("localhost:3000").await.unwrap();
    tokio::spawn(async move{
      let (reader, mut writer) = client.split();
      let mut reader = BufReader::new(reader);
      let mut buffer = [0;64];
      
      loop{
        tokio::select! {
          result = reader.read(&mut buffer) => {
              
            // messages.write().push(String::from_utf8_lossy(&buffer).to_string());
            buffer = [0;64];
            
            if let Err(_) = result{
              println!("Connection Broked");
              exit(0);
            };
          }
          message = rx.recv() => {
            writer.write_all(message.clone().unwrap().as_bytes()).await.expect("Error writing to stream");
          }
        }
      }
    }).await.unwrap();
  });
  let messages_lock = messages.read();
  let messages_rendered = messages_lock.iter().map(|message|{
    render!{
      Message{
        message: message.clone(),
      }
    }
  });
  render!{
    div{
      "{name}",
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
fn Message(cx: Scope, message:String ) -> Element{
  render!{
    li{
      class:"message-sender",
      "{message}",
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
            sender.send(message.get().clone());
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