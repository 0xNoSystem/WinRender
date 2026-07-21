// tri_test.rs
//
// Tests two adjacent triangles that form a rectangle.
// Every pixel center inside the rectangle must belong to exactly one triangle:
//
// coverage == 0  -> crack
// coverage == 1  -> correct
// coverage == 2  -> overlapping shared edge
//
use renderer::types::{Triangle, Vec2};

fn coverage_count(p: Vec2, triangles: &[Triangle]) -> usize {
    triangles
        .iter()
        .filter(|triangle| triangle.contains_point(p))
        .count()
}

#[test]
fn adjacent_triangles_have_no_cracks_or_overlap() {
    // Rectangle corners:
    //
    // p0 -------- p1
    // |         /  |
    // |       /    |
    // |     /      |
    // |   /        |
    // p3 -------- p2
    //
    // The triangles share the diagonal p0 -> p2.
    //
    // Integer coordinates are chosen deliberately: several pixel centers lie
    // directly on the diagonal, so this test can expose overlapping edge rules.
    let p0 = Vec2::new(10.0, 10.0);
    let p1 = Vec2::new(30.0, 10.0);
    let p2 = Vec2::new(30.0, 30.0);
    let p3 = Vec2::new(10.0, 30.0);

    let triangles = [Triangle::new(p0, p1, p2), Triangle::new(p0, p2, p3)];

    let mut cracks = Vec::new();
    let mut overlaps = Vec::new();

    // Test pixel centers. The upper bounds are exclusive because the rectangle
    // occupies pixels x = 10..29 and y = 10..29.
    for y in 10..30 {
        for x in 10..30 {
            let pixel_center = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            let coverage = coverage_count(pixel_center, &triangles);

            match coverage {
                0 => cracks.push((x, y)),
                1 => {}
                _ => overlaps.push((x, y)),
            }
        }
    }

    assert!(
        cracks.is_empty(),
        "Found {} uncovered pixels (cracks). First few: {:?}",
        cracks.len(),
        &cracks[..cracks.len().min(10)]
    );

    assert!(
        overlaps.is_empty(),
        "Found {} multiply-covered pixels (shared-edge overlap). First few: {:?}",
        overlaps.len(),
        &overlaps[..overlaps.len().min(10)]
    );
}

#[test]
fn adjacent_triangles_cover_the_expected_number_of_pixels() {
    let p0 = Vec2::new(10.0, 10.0);
    let p1 = Vec2::new(30.0, 10.0);
    let p2 = Vec2::new(30.0, 30.0);
    let p3 = Vec2::new(10.0, 30.0);

    let triangles = [Triangle::new(p0, p1, p2), Triangle::new(p0, p2, p3)];

    let covered_once = (10..30)
        .flat_map(|y| (10..30).map(move |x| (x, y)))
        .filter(|&(x, y)| {
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            coverage_count(p, &triangles) == 1
        })
        .count();

    assert_eq!(
        covered_once,
        20 * 20,
        "The two triangles should cover the entire 20x20 rectangle exactly once"
    );
}
