use std::sync::atomic::{AtomicIsize, AtomicU8, Ordering};

use windows::Win32::{
  Foundation::{LPARAM, LRESULT, WPARAM},
  UI::{
    Input::KeyboardAndMouse::{VK_LWIN, VK_RWIN},
    WindowsAndMessaging::{
      CallNextHookEx, HHOOK, KBDLLHOOKSTRUCT, SetWindowsHookExW, UnhookWindowsHookEx,
      WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
    },
  },
};

static HOOK: AtomicIsize = AtomicIsize::new(0);
static WIN_KEYS_HELD: AtomicU8 = AtomicU8::new(0);

const LEFT_WIN_HELD: u8 = 0b01;
const RIGHT_WIN_HELD: u8 = 0b10;

fn is_key_down(message: u32) -> bool {
  matches!(message, WM_KEYDOWN | WM_SYSKEYDOWN)
}

fn is_key_up(message: u32) -> bool {
  matches!(message, WM_KEYUP | WM_SYSKEYUP)
}

extern "system" fn keyboard_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
  if code >= 0 {
    let ev = unsafe { *(lparam.0 as *const KBDLLHOOKSTRUCT) };
    let message = wparam.0 as u32;
    let vk_code = ev.vkCode as u16;

    if vk_code == VK_LWIN.0 || vk_code == VK_RWIN.0 {
      let mask = if vk_code == VK_LWIN.0 {
        LEFT_WIN_HELD
      } else {
        RIGHT_WIN_HELD
      };

      if is_key_down(message) {
        WIN_KEYS_HELD.fetch_or(mask, Ordering::SeqCst);
      } else if is_key_up(message) {
        WIN_KEYS_HELD.fetch_and(!mask, Ordering::SeqCst);
      }

      return LRESULT(1);
    }

    if WIN_KEYS_HELD.load(Ordering::SeqCst) != 0 {
      return LRESULT(1);
    }
  }

  unsafe { CallNextHookEx(None, code, wparam, lparam) }
}

pub fn attach() {
  unsafe {
    let hhk = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), None, 0)
      .expect("failed to install keyboard hook");
    HOOK.store(hhk.0 as isize, Ordering::SeqCst);
  }
}

pub fn detach() {
  let ptr = HOOK.swap(0, Ordering::SeqCst);
  if ptr != 0 {
    unsafe {
      let _ = UnhookWindowsHookEx(HHOOK(ptr as *mut _));
    }
  }
}
