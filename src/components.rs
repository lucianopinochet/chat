use dioxus::prelude::*;
use dioxus_router::prelude::*;
// use fermi::*;
use crate::Route;
// static MESSAGES: Atom<Vec<String>> = Atom(|_| Vec::<String>::new());

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
        messages: messages
      }
    }
  }
}
#[inline_props]
fn Message(cx: Scope, message:String ) -> Element{
  render!{
    li{
      class:"message",
      "{message}",
    }
  }
}
#[inline_props]
fn SendBar<'a>(cx: Scope, messages: &'a UseRef<Vec<String>>) -> Element{
  let message = use_state(cx, || "".to_string());
	render!{
			form{
				onsubmit: move 	|_|{
						messages.write().push(message.get().clone());
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