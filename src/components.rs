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
          oninput: move |e|{
            name.set(e.value.clone())
          }
        }
        Link{
          to: Route::Chat{
            name: name.get().clone()
          },
          input{
            r#type:"submit"
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
      }

    }
  }
}
#[inline_props]
fn Message(cx: Scope, message:String ) -> Element{
  render!{
    li{
      "{message}",
      // key:"{id}"
    }
  }
}