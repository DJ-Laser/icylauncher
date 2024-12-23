use iced::widget::{button, column, text, Column};
use iced::Theme;
use iced_layershell::build_pattern::{application, MainSettings};
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::LayerShellSettings;
use iced_layershell::to_layer_message;
use listings::applications::ApplicationProvider;
use listings::Provider;

pub mod listings;

fn main() -> Result<(), iced_layershell::Error> {
  let mut provider = ApplicationProvider::new();
  provider.update_listings();

  application("A cool counter", Counter::update, Counter::view)
    .theme(|_| Theme::Dark)
    .settings(MainSettings {
      layer_settings: LayerShellSettings {
        size: Some((1000, 600)),
        anchor: Anchor::Top,
        margin: (200, 0, 0, 0),
        ..Default::default()
      },
      ..Default::default()
    })
    .run()
}

#[to_layer_message]
#[derive(Debug, Clone, Copy)]
enum Message {
  Increment,
  Decrement,
}

#[derive(Default)]
struct Counter {
  value: i64,
}

impl Counter {
  fn update(&mut self, message: Message) {
    match message {
      Message::Increment => {
        self.value += 1;
      }
      Message::Decrement => {
        self.value -= 1;
      }

      _ => todo!(),
    }
  }

  fn view(&self) -> Column<Message> {
    column![
      button("+").on_press(Message::Increment),
      text(self.value),
      button("-")
        .on_press(Message::Decrement)
        .style(|theme: &Theme, status: button::Status| button::danger(theme, status))
    ]
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_counts_properly() {
    use super::*;

    let mut counter = Counter { value: 0 };

    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);

    assert_eq!(counter.value, 1);
  }
}
