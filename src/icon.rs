//! Procedurally drawn tray icons.
//!
//! Two states are rendered: a blue Windows flag (feature off, key allowed) and
//! the same flag with a red "no" symbol on top (feature on, key blocked).
//! Everything is drawn at 4x resolution and box-downsampled for anti-aliasing.

use tray_icon::Icon;

const N: usize = 64;
const SS: usize = 4;
const BIG: usize = N * SS;

type Rgba = [u8; 4];

const BLUE: Rgba = [59, 130, 246, 255];
const GREY: Rgba = [150, 156, 163, 255];
const RED: Rgba = [235, 64, 64, 255];

struct Canvas {
  px: Vec<Rgba>,
}

impl Canvas {
  fn new() -> Self {
    Canvas {
      px: vec![[0, 0, 0, 0]; BIG * BIG],
    }
  }

  fn put(&mut self, x: usize, y: usize, c: Rgba) {
    self.px[y * BIG + x] = c;
  }

  /// Box-downsample from BIG to N using premultiplied alpha so transparent
  /// pixels don't darken the edges.
  fn into_rgba(self) -> Vec<u8> {
    let mut out = Vec::with_capacity(N * N * 4);
    for y in 0..N {
      for x in 0..N {
        let (mut r, mut g, mut b, mut a) = (0f32, 0f32, 0f32, 0f32);
        for dy in 0..SS {
          for dx in 0..SS {
            let p = self.px[(y * SS + dy) * BIG + (x * SS + dx)];
            let pa = p[3] as f32;
            r += p[0] as f32 * pa;
            g += p[1] as f32 * pa;
            b += p[2] as f32 * pa;
            a += pa;
          }
        }
        if a > 0.0 {
          out.push((r / a) as u8);
          out.push((g / a) as u8);
          out.push((b / a) as u8);
        } else {
          out.extend_from_slice(&[0, 0, 0]);
        }
        out.push((a / (SS * SS) as f32) as u8);
      }
    }
    out
  }
}

fn f(v: f32) -> f32 {
  v * BIG as f32
}

/// Filled rounded rectangle, coordinates in the 0..1 range.
fn rounded_rect(c: &mut Canvas, x0: f32, y0: f32, x1: f32, y1: f32, r: f32, color: Rgba) {
  let (x0, y0, x1, y1, r) = (f(x0), f(y0), f(x1), f(y1), f(r));
  let (ix0, iy0, ix1, iy1) = (x0 as usize, y0 as usize, x1.ceil() as usize, y1.ceil() as usize);
  for y in iy0..iy1.min(BIG) {
    for x in ix0..ix1.min(BIG) {
      let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
      let qx = (x0 + r - px).max(px - (x1 - r)).max(0.0);
      let qy = (y0 + r - py).max(py - (y1 - r)).max(0.0);
      if qx * qx + qy * qy <= r * r {
        c.put(x, y, color);
      }
    }
  }
}

/// The four-pane Windows flag centered in the canvas.
fn windows_flag(c: &mut Canvas, color: Rgba) {
  let (lo, hi) = (0.26, 0.74);
  let mid = (lo + hi) / 2.0;
  let gap = 0.028;
  let r = 0.02;
  rounded_rect(c, lo, lo, mid - gap, mid - gap, r, color);
  rounded_rect(c, mid + gap, lo, hi, mid - gap, r, color);
  rounded_rect(c, lo, mid + gap, mid - gap, hi, r, color);
  rounded_rect(c, mid + gap, mid + gap, hi, hi, r, color);
}

/// Red circular "no" symbol (ring + diagonal bar) over the whole icon.
fn no_symbol(c: &mut Canvas) {
  let cx = f(0.5);
  let cy = f(0.5);
  let outer = f(0.46);
  let thick = f(0.085);
  let inner = outer - thick;
  // Diagonal bar direction: top-right to bottom-left.
  let (dx, dy) = (-0.70710677f32, 0.70710677f32);
  for y in 0..BIG {
    for x in 0..BIG {
      let (px, py) = (x as f32 + 0.5 - cx, y as f32 + 0.5 - cy);
      let dist = (px * px + py * py).sqrt();
      let in_ring = dist <= outer && dist >= inner;
      let along = px * dx + py * dy;
      let perp = px * -dy + py * dx;
      let in_bar = dist <= outer && perp.abs() <= thick / 2.0 && along.abs() <= outer;
      if in_ring || in_bar {
        c.put(x, y, RED);
      }
    }
  }
}

fn blocked_rgba() -> Vec<u8> {
  let mut c = Canvas::new();
  windows_flag(&mut c, BLUE);
  no_symbol(&mut c);
  c.into_rgba()
}

fn allowed_rgba() -> Vec<u8> {
  let mut c = Canvas::new();
  windows_flag(&mut c, GREY);
  c.into_rgba()
}

/// Icon for when the feature is ON — the Windows key is blocked.
pub fn blocked() -> Icon {
  Icon::from_rgba(blocked_rgba(), N as u32, N as u32).unwrap()
}

/// Icon for when the feature is OFF — the Windows key works normally.
pub fn allowed() -> Icon {
  Icon::from_rgba(allowed_rgba(), N as u32, N as u32).unwrap()
}
