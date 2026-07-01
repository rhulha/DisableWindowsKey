use std::sync::atomic::{AtomicIsize, Ordering};

use windows::Win32::{
  Foundation::{LPARAM, LRESULT, WPARAM},
  UI::{
    Input::KeyboardAndMouse::VK_LWIN,
    WindowsAndMessaging::{
      CallNextHookEx, HHOOK, KBDLLHOOKSTRUCT, SetWindowsHookExW, UnhookWindowsHookEx,
      WH_KEYBOARD_LL,
    },
  },
};

static HOOK: AtomicIsize = AtomicIsize::new(0);

extern "system" fn keyboard_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
  if code >= 0 {
    let ev = unsafe { *(lparam.0 as *const KBDLLHOOKSTRUCT) };
    if ev.vkCode as u16 == VK_LWIN.0 {
      // Swallow every left Windows key event.
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
