#[cfg(test)]
use crate::basic_structs::Rect;

#[test]
fn circle_collisions() {
    use crate::basic_structs::{Circle, Pos};

    let c = Circle { radius: 1.0 };
    assert!(c.contains_point(&Pos { x: 0.0, y: 0.0 }));
    assert!(c.contains_point(&Pos { x: 0.0, y: 1.0 }));
    assert!(c.contains_point(&Pos { x: 1.0, y: 0.0 }));
    assert!(c.contains_point(&Pos {
        x: 1.0 / (2.0_f32.sqrt()) - 0.0001,
        y: 1.0 / (2.0_f32.sqrt()) - 0.0001
    }));
    assert!(!c.contains_point(&Pos { x: 1.0, y: 1.0 }))
}
#[test]
fn circle_collisions_rect1() {
    use crate::basic_structs::{Circle, Pos};

    let c = Circle { radius: 1.0 };

    let rc = Pos { x: 0.0, y: 0.0 };
    let r = Rect::new(&rc, 0.5, 0.5, 0.0, 0.0, 0.0);

    assert!(c.contains(&r))
}
#[test]
fn circle_collisions_rect2() {
    use crate::basic_structs::{Circle, Pos};

    let c = Circle { radius: 1.0 };

    let rc = Pos { x: 0.4, y: 0.3 };
    let r = Rect::new(&rc, 0.5, 0.5, 0.0, 0.0, 0.0);

    assert!(c.contains(&r))
}
#[test]
fn circle_collisions_rect3() {
    use crate::basic_structs::{Circle, Pos};

    let c = Circle { radius: 1.0 };

    let rc = Pos { x: 0.4, y: 0.3 };
    let r = Rect::new(&rc, 1.0, 0.5, 0.0, 0.0, 0.0);

    assert!(!c.contains(&r))
}
#[test]
fn circle_collisions_rect4() {
    use crate::basic_structs::{Circle, Pos};

    let c = Circle { radius: 1.0 };

    let rc = Pos { x: 0.0, y: 0.0 };
    let r = Rect::new(&rc, 2.0, 0.5, 0.0, 0.0, 0.0);

    assert!(!c.contains(&r))
}
#[test]
fn rect_collision_test() {
    use crate::basic_structs::Pos;

    let rc = Pos { x: 0.0, y: 0.0 };
    let r = Rect::new(&rc, 2.0, 0.5, 0.0, 0.0, 0.0);

    let r2 = Rect::new(&rc, 2.0, 0.5, 0.0, 0.0, 0.0);

    assert!(r.overlaps(&r2))
}
#[test]
fn rect_collision_test2() {
    use crate::basic_structs::Pos;

    let rc = Pos { x: 0.0, y: 0.0 };
    let r = Rect::new(&rc, 1.0, 1.0, 0.0, 0.0, 0.0);

    let rc2 = Pos { x: 1.0, y: 1.0 };
    let r2 = Rect::new(&rc2, 1.0, 1.0, 0.0, 0.0, 0.0);

    assert!(!r.overlaps(&r2))
}
#[test]
fn rect_collision_test3() {
    use crate::basic_structs::Pos;

    let rc = Pos { x: 0.0, y: 0.0 };
    let r = Rect::new(&rc, 1.0, 1.0, 0.0, 0.0, 0.0);

    let rc2 = Pos { x: 1.1, y: 1.1 };
    let r2 = Rect::new(&rc2, 1.0, 1.0, 0.0, 0.0, 0.0);

    assert!(!r.overlaps(&r2))
}
#[test]
fn rect_collision_test4() {
    use crate::basic_structs::Pos;

    let rc = Pos { x: 0.0, y: 0.0 };
    let r = Rect::new(&rc, 1.0, 1.0, 0.0, 0.0, 0.0);

    let rc2 = Pos { x: 0.7, y: 0.7 };
    let r2 = Rect::new(&rc2, 1.0, 1.0, 0.0, 0.0, 0.0);

    assert!(r.overlaps(&r2))
}
#[test]
fn vlinetest() {
    use crate::basic_structs::Pos;

    let rc = Pos { x: 0.0, y: 0.0 };
    let r = Rect::new(&rc, 1.0, 1.0, 0.0, 0.0, 0.0);

    assert!(r.contains_vert_line(0.5));
    assert!(!r.contains_vert_line(0.6))
}
