#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct MyComponent {
  link: ComponentLink<Self>,
  active: bool,
}

enum Message {
  ButtonClick,
}

impl Component for MyComponent {
  type Message = Message;
  type Properties = ();
  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      link,
      active: false,
    }
  }

  fn update(&mut self, message: Self::Message) -> ShouldRender {
    match message {
      Message::ButtonClick => self.active = !self.active,
    };

    true
  }

  fn view(&self) -> Html {
    let text = if self.active {
      "on -> off"
    } else {
      "off -> on"
    };

    html! {
      <button onclick=self.link.callback(|_| Message::ButtonClick)>{text}</button>
    }
  }
}
#[wasm_bindgen]
pub fn render_hello() -> Result<(), JsValue> {
  yew::initialize();
  App::<MyComponent>::new().mount_to_body();
  Ok(())
}
