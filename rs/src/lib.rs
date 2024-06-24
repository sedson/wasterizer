#![allow(dead_code)]

// Grab some js string utilities.
#[cfg(target_arch="wasm32")]
extern {
    fn js_panic(ptr: *const u8, len: i32);
    fn js_log(ptr: *const u8, len: i32);
}

// String log.
pub fn log (s: String) {
    #[cfg(target_arch="wasm32")] {
        unsafe {
            js_log(s.as_ptr(), s.len().try_into().unwrap());
        }
    }

    #[cfg(not(target_arch="wasm32"))] {
        println!("{}", s);
    }
}


/// Allocate size u8s that JavaScript will be the "owner" of. Via Paul and
/// Glicol.
#[no_mangle]
pub fn alloc_u8 (size: usize) -> *mut u8 {
    let mut buf = vec![0u8; size];
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr as *mut u8
}

/// An example of doing some Rust stuff on a JS owned chunk of data.
#[no_mangle]
pub fn fill_u8 (buf: *mut u8, sz: usize, val: u8) {
    let m: &mut [u8] = unsafe {
        std::slice::from_raw_parts_mut(buf, sz)
    };
    for n in 0..sz {
        m[n] = val;
    }
}


/// Color struct. Handles taking an RGBA tuple over the WASM bridge as an i32.
/// A u32 would look nicer, but is not natively part of the WASM spec so a
/// signed int does just fine.
#[derive(Debug, Clone)]
pub struct Col (pub u8, pub u8, pub u8, pub u8);

// Private utils.
impl Col {
    fn i32_to_rgba (val: i32) -> (u8, u8, u8, u8) {
        let r = ((val >> 24) & 0xff) as u8;
        let g = ((val >> 16) & 0xff) as u8;
        let b = ((val >> 8) & 0xff) as u8;
        let a = (val & 0xff) as u8;
        (r, g, b, a)
    }

    fn rgba_to_i32 (r: u8, g: u8, b: u8, a: u8) -> i32 {
        ((r as i32) << 24) | ((g as i32) << 16) | ((b as i32) << 8) | a as i32
    }
}

impl core::default::Default for Col {
    fn default () -> Self {
        Self(0 ,0 ,0, 255)
    }
}

impl Col {
    pub fn new (r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(r, g, b, a)
    }

    pub fn from_i32(val: i32) -> Self {
        let (r, g, b, a) = Self::i32_to_rgba(val);
        Self(r, g, b, a)
    }

    pub fn to_i32(&mut self) -> i32 {
        Self::rgba_to_i32(self.0, self.1, self.2, self.3)
    }
}

// Vectors...or at least what I need of them.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Vec2<T> {
    pub x : T,
    pub y : T
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Vec3<T> {
    pub x : T,
    pub y : T,
    pub z : T,
}

impl Vec3<f32> {
    fn cross (a: Vec3<f32>, b: Vec3<f32>) -> Vec3<f32> {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }
}

pub fn vec2i (x: i32, y: i32) -> Vec2<i32> {
    Vec2 { x, y }
}

pub fn vec3i (x: i32, y: i32, z: i32) -> Vec3<i32> {
    Vec3 { x, y, z }
}

pub fn vec2f (x: f32, y: f32) -> Vec2<f32> {
    Vec2 { x, y }
}

pub fn vec3f (x: f32, y: f32, z: f32) -> Vec3<f32> {
    Vec3 { x, y, z }
}




// Finally the Renderer.
// Arrange the data in one long vector of [ width * height * 4 (RGBA) ] u8's.
// An array with this shape can be passed directly from WASM linear memory
// to canvas2d and WebGL without any copying.
const CHANNELS: i32 = 4;


pub struct Renderer {
    pub width: i32,
    pub height: i32,
    pub size: usize,
    data: Box<Vec<u8>>,
}

impl Renderer {
    pub fn new (width : i32, height: i32) -> Self {
        let size = (width * height * CHANNELS as i32) as usize;
        log(String::from("Renderer constructed"));
        Self {
            width,
            height,
            size,
            data: Box::new(vec![ 0u8; size ]),
        }
    }

    fn bounds (&self, x: i32, y: i32) -> bool {
        x > -1 && y > -1 && x < self.width && y < self.height
    }

    fn pixel_index (&self, x: i32, y: i32) -> Option<usize> {
        (self.bounds(x, y)).then_some(((y * self.width + x) * CHANNELS) as usize)
    }

    pub fn get_pixel (&self, x: i32, y: i32) -> Option<Col> {
        self.pixel_index(x, y).map(|i| {
            Col::new(self.data[i], self.data[i + 1], self.data[i + 2], self.data[i + 3])
        })
    }

    pub fn set_pixel (&mut self, x : i32, y: i32, color: &Col) {
        self.pixel_index(x, y).map(|i| {
            self.data[i] = color.0;
            self.data[i + 1] = color.1;
            self.data[i + 2] = color.2;
            self.data[i + 3] = color.3;
        });
    }

    pub fn fill_channel (&mut self, channel: i32, val: u8) {
        if channel < 0 || channel > CHANNELS {
            return;
        }
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.pixel_index(x, y).unwrap();
                self.data[i + channel as usize] = val;
            }
        }
    }

    pub fn fill(&mut self, color: &Col) {
         for y in 0..self.height {
            for x in 0..self.width {
                let i = self.pixel_index(x, y).unwrap();
                self.data[i]     = color.0;
                self.data[i + 1] = color.1;
                self.data[i + 2] = color.2;
                self.data[i + 3] = color.3;
            }
        }
    }

    pub fn line(&mut self, pt1: &Vec2<i32> , pt2: &Vec2<i32>, color: &Col) {
        let steep: bool = (pt2.x - pt1.x).abs() < (pt2.y - pt1.y).abs();

        // if steep transpose x and y.
        let (x1, y1, x2, y2) = if steep {
            ( pt1.y, pt1.x, pt2.y, pt2.x)
        } else {
            (pt1.x, pt1.y, pt2.x, pt2.y)
        };

        // if backwards swap point 1 for point 2.
        let (x1, y1, x2, y2) = if x1 > x2 {
            (x2, y2, x1, y1)
        } else {
            (x1, y1, x2, y2)
        };

        let (dx, dy) = (x2 - x1, y2 - y1);
        let derror2 = (dy * 2).abs();
        let mut error2 = 0;
        let mut y = y1;

        for x in x1..=x2 {
            if steep {
                self.set_pixel(y, x, color);
            } else {
                self.set_pixel(x, y, color);
            }

            error2 += derror2;
            if error2 > dx {
                y += if y2 > y1 { 1 } else { -1 };
                error2 -= dx * 2;
            }
        }
    }

    fn barycentric (a: &Vec2<i32>, b: &Vec2<i32> , c: &Vec2<i32>, p: &Vec2<i32>) -> Vec3<f32> {
        let x = vec3f((c.x - a.x) as f32, (b.x - a.x) as f32, (a.x - p.x) as f32);
        let y = vec3f((c.y - a.y) as f32, (b.y - a.y) as f32, (a.y - p.y) as f32);
        let u = Vec3::cross(x, y);
        if u.z.abs() < 1.0 {
            vec3f(-1.0, 0.0, 0.0)
        } else {
            vec3f(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
        }
    }

    pub fn triangle_barycentric(&mut self, pt1: &Vec2<i32>, pt2: &Vec2<i32>, pt3: &Vec2<i32>, color: &Col) {
        let mut max: Vec2<i32> = Vec2{x: 0, y: 0};
        let mut min: Vec2<i32> = Vec2{x: self.width, y: self.height};

        for pt in [pt1, pt2, pt3] {
            if pt.x < min.x { min.x = pt.x }
            if pt.y < min.y { min.y = pt.y }
            if pt.x > max.x { max.x = pt.x }
            if pt.y > max.y { max.y = pt.y }
        }

        if max.x > self.width {max.x = self.width}
        if max.y > self.height {max.y = self.height}
        if min.x < 0 {min.x = 0}
        if min.y < 0 {min.y = 0}

        for y in min.y..max.y {
            for x in min.x..max.x {
                let p = Vec2{x, y};
                let bc = Self::barycentric(pt1, pt2, pt3, &p);
                if bc.x >= 0.0 && bc.y >= 0.0 && bc.z >= 0.0 {
                    self.set_pixel(x, y, color);
                }
            }
        }
    }

    pub fn triangle_wireframe(&mut self, pt1: &Vec2<i32>, pt2: &Vec2<i32>, pt3: &Vec2<i32>, color: &Col) {
        self.line(pt1, pt2, color);
        self.line(pt2, pt3, color);
        self.line(pt3, pt1, color);
    }

    pub fn rect_fill (&mut self, pt1: &Vec2<i32>, pt2: &Vec2<i32>, color: &Col) {
        let (x_min, x_max) = if pt1.x < pt2.x { (pt1.x, pt2.x) } else { (pt2.x, pt1.x) };
        let (y_min, y_max) = if pt1.y < pt2.y { (pt1.y, pt2.y) } else { (pt2.y, pt1.y) };

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                self.set_pixel(x, y, color)
            }
        }
    }

    pub fn rect_wireframe (&mut self, pt1: &Vec2<i32>, pt2: &Vec2<i32>, color: &Col) {
        self.line(&vec2i(pt1.x, pt1.y), &vec2i(pt2.x, pt1.y), color);
        self.line(&vec2i(pt2.x, pt1.y), &vec2i(pt2.x, pt2.y), color);
        self.line(&vec2i(pt2.x, pt2.y), &vec2i(pt1.x, pt2.y), color);
        self.line(&vec2i(pt1.x, pt2.y), &vec2i(pt1.x, pt1.y), color);
    }
}


#[no_mangle]
pub fn new(width: i32, height: i32) -> Box<Renderer> {
    let mut renderer = Renderer::new(width, height);
    renderer.fill_channel(3, 255);
    Box::new(renderer)
}

#[no_mangle]
pub fn data_size(renderer: &mut Renderer) -> i32 {
    renderer.size as i32
}

#[no_mangle]
pub fn data_ptr(renderer: &mut Renderer) -> *const u8 {
    let ptr = renderer.data.as_ptr();
    ptr as *const u8
}

#[no_mangle]
pub fn line(renderer: &mut Renderer, x1: i32, y1: i32, x2: i32, y2: i32, color: i32) {
    renderer.line(&vec2i(x1, y1), &vec2i(x2, y2), &Col::from_i32(color));
}

#[no_mangle]
pub fn fill(renderer: &mut Renderer, color: i32) {
    renderer.fill(&Col::from_i32(color));
}

#[no_mangle]
pub fn peek(renderer: &mut Renderer, x: i32, y: i32) -> i32 {
    renderer.get_pixel(x, y).unwrap_or_default().to_i32()
}

#[no_mangle]
pub fn tri(renderer: &mut Renderer, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: i32) {
    let color = Col::from_i32(color);
    renderer.triangle_barycentric(&vec2i(x1, y1), &vec2i(x2, y2), &vec2i(x3, y3), &color);
}

#[no_mangle]
pub fn tri_wf(renderer: &mut Renderer, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: i32) {
    let color = Col::from_i32(color);
    renderer.triangle_wireframe(&vec2i(x1, y1), &vec2i(x2, y2), &vec2i(x3, y3), &color);
}

#[no_mangle]
pub fn rect(renderer: &mut Renderer, x1: i32, y1: i32, x2: i32, y2: i32, color: i32) {
    let color = Col::from_i32(color);
    renderer.rect_fill(&vec2i(x1, y1), &vec2i(x2, y2), &color);
}

#[no_mangle]
pub fn rect_wf(renderer: &mut Renderer, x1: i32, y1: i32, x2: i32, y2: i32, color: i32) {
    let color = Col::from_i32(color);
    renderer.rect_wireframe(&vec2i(x1, y1), &vec2i(x2, y2), &color);
}