mod content;
mod pos;
mod render;
mod test;

fn main() {
    // test::test_rects_texture();
    test::test_cube_inside(
        100,
        vec![
            "/tmp/a/i1.jpg".to_string(),
            "/tmp/a/i2.jpg".to_string(),
            "/tmp/a/i3.jpg".to_string(),
            "/tmp/a/i4.jpg".to_string(),
        ],
    );
}
