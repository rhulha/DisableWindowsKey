#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tao::{
  event::{Event, StartCause},
  event_loop::{ControlFlow, EventLoopBuilder},
};
use tray_icon::{
  Icon, TrayIconBuilder,
  menu::{Menu, MenuEvent, MenuItem},
};

mod disable_key;

enum UserEvent {
  MenuEvent(MenuEvent),
}

fn build_icon() -> Icon {
  let size = 32u32;
  let mut rgba = Vec::with_capacity((size * size * 4) as usize);
  for y in 0..size {
    for x in 0..size {
      let border = x == 0 || y == 0 || x == size - 1 || y == size - 1;
      if border {
        rgba.extend_from_slice(&[0x20, 0x20, 0x20, 0xff]);
      } else {
        rgba.extend_from_slice(&[0x2d, 0x7d, 0xd6, 0xff]);
      }
    }
  }
  Icon::from_rgba(rgba, size, size).unwrap()
}

fn main() {
  let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

  let proxy = event_loop.create_proxy();
  MenuEvent::set_event_handler(Some(move |event| {
    proxy.send_event(UserEvent::MenuEvent(event)).ok();
  }));

  let tray_menu = Menu::new();
  let quit_i = MenuItem::new("Quit", true, None);
  tray_menu.append(&quit_i).unwrap();

  let mut tray_icon = None;

  disable_key::attach();

  event_loop.run(move |event, _target, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => {
        tray_icon = Some(
          TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu.clone()))
            .with_tooltip("Disable Windows Key")
            .with_icon(build_icon())
            .build()
            .unwrap(),
        );
      },
      Event::LoopDestroyed => {
        disable_key::detach();
      },
      Event::UserEvent(UserEvent::MenuEvent(event)) => {
        if event.id == quit_i.id() {
          disable_key::detach();
          tray_icon.take();
          *control_flow = ControlFlow::Exit;
        }
      },
      _ => {},
    }
  });
}
