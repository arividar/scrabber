use win_screenshot::capture::*;

fn main() {
    let image = capture_display().unwrap();
    image.save("screenshot.jpg").unwrap();
}
