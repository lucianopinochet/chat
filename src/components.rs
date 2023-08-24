// use tokio::sync::mpsc::{self, Sender, Receiver};
use dioxus_router::prelude::*;
use dioxus::prelude::*;
use crate::{Route, tool::coroutine_handle};
// use std::sync::mpsc as sync_mpsc;
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
  
  let tx: &Coroutine<String> = use_coroutine(cx, |rx:UnboundedReceiver<String>| async move {
    coroutine_handle(rx).await;
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
fn SendBar<'a>(cx: Scope, messages: &'a UseRef<Vec<String>>, sender: &'a dioxus::prelude::Coroutine<String>) -> Element{
  let message = use_state(cx, || "".to_string());
	render!{
			form{
				onsubmit: move |_|{
						messages.write().push(message.get().clone());
            let _ws: &Coroutine<()> = use_coroutine(cx, |_rx| async move {
              sender.send(message.get().clone());
            });
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