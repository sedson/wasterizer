export function rgba_to_i32(r = 0, g = 0, b = 0, a = 0) {
  return ((r & 0xff) << 24) | ((g & 0xff) << 16) | ((b & 0xff) << 8) | (a & 0xff);
}

export function i32_to_rgb(col) {
  const r = (col >> 24) & 0xff;
  const g = (col >> 16) & 0xff;
  const b = (col >> 8) & 0xff;
  const a = (col) & 0xff;
  return [r, g, b, a];
}

export class Color {
  constructor(r, g, b, a = 255) {
    this.r = r;
    this.g = g;
    this.b = b;
    this.a = a;
  }

  static from_i32(val) {
    return new Color(...i32_to_rgb(val))
  }

  to_i32() {
    return rgba_to_i32(this.r, this.g, this.b, this.a);
  }
}