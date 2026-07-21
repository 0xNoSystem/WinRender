// tests/depth_test.rs
//
// Proves that rendering order does not affect the final visible result when
// the depth buffer is working correctly.
//

use renderer::{Color, ScreenBuffer, Triangle3, TriangleFillType, Vec3};

const WIDTH: u32 = 64;
const HEIGHT: u32 = 64;

fn back_triangle() -> Triangle3 {
    Triangle3::new(
        Vec3::new(8.0, 8.0, 8.0),
        Vec3::new(56.0, 12.0, 8.0),
        Vec3::new(30.0, 56.0, 8.0),
    )
}

fn front_triangle() -> Triangle3 {
    Triangle3::new(
        Vec3::new(16.0, 16.0, 2.0),
        Vec3::new(50.0, 20.0, 2.0),
        Vec3::new(30.0, 48.0, 2.0),
    )
}

fn render_scene(front_first: bool) -> ScreenBuffer {
    let mut screen = ScreenBuffer::new(WIDTH, HEIGHT, Some(Color::Black as u32));

    let front = (front_triangle(), TriangleFillType::Solid(Color::Red as u32));

    let back = (back_triangle(), TriangleFillType::Solid(Color::Blue as u32));

    if front_first {
        screen.fill_triangle_3d(front.0, front.1);
        screen.fill_triangle_3d(back.0, back.1);
    } else {
        screen.fill_triangle_3d(back.0, back.1);
        screen.fill_triangle_3d(front.0, front.1);
    }

    screen
}

#[test]
fn draw_order_does_not_change_color_buffer() {
    let front_then_back = render_scene(true);
    let back_then_front = render_scene(false);

    assert_eq!(
        front_then_back.pixels(),
        back_then_front.pixels(),
        "Final color buffer changed when triangle draw order changed"
    );
}

#[test]
fn closer_triangle_wins_in_overlap() {
    let screen = render_scene(false);

    // This point is well inside both triangles.
    let x = 30;
    let y = 28;
    let idx = y as usize * WIDTH as usize + x as usize;

    assert_eq!(
        screen.pixels()[idx],
        Color::Red as u32,
        "The closer red triangle should be visible at the overlapping pixel"
    );
}

#[test]
fn farther_triangle_cannot_overwrite_closer_triangle() {
    let mut screen = ScreenBuffer::new(WIDTH, HEIGHT, Some(Color::Black as u32));

    let front = Triangle3::new(
        Vec3::new(8.0, 8.0, 2.0),
        Vec3::new(56.0, 12.0, 2.0),
        Vec3::new(30.0, 56.0, 2.0),
    );

    let back = Triangle3::new(
        Vec3::new(8.0, 8.0, 8.0),
        Vec3::new(56.0, 12.0, 8.0),
        Vec3::new(30.0, 56.0, 8.0),
    );

    screen.fill_triangle_3d(front, TriangleFillType::Solid(Color::Red as u32));

    let before = screen.pixels().to_vec();

    screen.fill_triangle_3d(back, TriangleFillType::Solid(Color::Blue as u32));

    assert_eq!(
        screen.pixels(),
        before.as_slice(),
        "The farther triangle overwrote the closer triangle"
    );
}
