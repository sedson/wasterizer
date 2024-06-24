use std::fs;
use render_lib::{Renderer, Col};

/// Output the renderer's buffer to a simple image file.
fn dump_ppm (renderer: &Renderer) -> String {
    let mut ppm_str = String::from("P3\n");
    ppm_str.push_str("# PPM export from Rust\n");
    ppm_str.push_str(&format!("{} {}\n", renderer.width, renderer.height));
    ppm_str.push_str("255\n");

    for y in 0..renderer.height {
        for x in 0..renderer.width {

            let col = match renderer.get_pixel(x, y) {
                Some(val) => val,
                None => Col::new(0, 0, 0, 255)
            };
            ppm_str.push_str(&format!("{} {} {}\n", col.0, col.1, col.2));
        }
    }
    ppm_str
}



fn test_1 () {
    println!("Running test_1: plain red square");
    let mut renderer = Renderer::new(128, 128);
    let red = Col::new(255, 0, 0, 255);
    renderer.fill(&red); 
    let _ = fs::write("img/test_1.ppm", dump_ppm(&renderer));
}






fn main () {
    test_1();
}