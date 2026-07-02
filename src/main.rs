#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tao::{
  event::{Event, StartCause},
  event_loop::{ControlFlow, EventLoopBuilder},
};
use tray_icon::{
  TrayIconBuilder,
  menu::{Menu, MenuEvent, MenuItem},
};

mod disable_key;
mod icon;

enum UserEvent {
  Menu(MenuEvent),
}

const TIP_ON: &str = "Windows key blocked";
const TIP_OFF: &str = "Windows key allowed";

fn main() {
  let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

  let proxy = event_loop.create_proxy();
  MenuEvent::set_event_handler(Some(move |event| {
    proxy.send_event(UserEvent::Menu(event)).ok();
  }));

  let icon_on = icon::blocked();
  let icon_off = icon::allowed();

  let toggle_i = MenuItem::new("Disable", true, None);
  let quit_i = MenuItem::new("Quit", true, None);
  let tray_menu = Menu::new();
  tray_menu.append_items(&[&toggle_i, &quit_i]).unwrap();

  let mut tray_icon = None;
  let mut enabled = true;

  disable_key::attach();

  event_loop.run(move |event, _target, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => {
        tray_icon = Some(
          TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu.clone()))
            .with_tooltip(TIP_ON)
            .with_icon(icon_on.clone())
            .build()
            .unwrap(),
        );
      },
      Event::LoopDestroyed => {
        disable_key::detach();
      },
      Event::UserEvent(UserEvent::Menu(event)) => {
        if event.id == toggle_i.id() {
          enabled = !enabled;
          if enabled {
            disable_key::attach();
          } else {
            disable_key::detach();
          }
          toggle_i.set_text(if enabled { "Disable" } else { "Enable" });
          if let Some(tray) = &tray_icon {
            tray.set_icon(Some(if enabled { icon_on.clone() } else { icon_off.clone() })).ok();
            tray.set_tooltip(Some(if enabled { TIP_ON } else { TIP_OFF })).ok();
          }
        } else if event.id == quit_i.id() {
          disable_key::detach();
          tray_icon.take();
          *control_flow = ControlFlow::Exit;
        }
      },
      _ => {},
    }
  });
}
