//! Apply ordering from static `#[reference]` metadata.

use core_lib::apply::apply_order;
use core_macros::{resource, service};

#[resource(
    sync = crud,
    list = get("/api/v3/tag"),
    create = post("/api/v3/tag"),
    update = put("/api/v3/tag/${self.id}"),
    delete = delete("/api/v3/tag/${self.id}"),
)]
pub struct Tag {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub label: String,
}

/// References `tag` — must be applied *after* tags exist.
#[resource(
    sync = crud,
    list = get("/api/v3/widget"),
    create = post("/api/v3/widget"),
    update = put("/api/v3/widget/${self.id}"),
    delete = delete("/api/v3/widget/${self.id}"),
)]
pub struct Widget {
    #[id]
    pub id: Option<i32>,
    #[key]
    pub name: String,
    #[reference(tag)]
    pub tags: Vec<i32>,
}

// Declared widgets-BEFORE-tags on purpose: a wrong impl that just preserved
// declaration order would fail this.
#[service(name = "lib", auth = none)]
pub struct Lib {
    pub url: String,
    pub widgets: Vec<Widget>,
    pub tags: Vec<Tag>,
}

#[test]
fn referenced_type_comes_first() {
    let order = apply_order::<Lib>().unwrap();
    let tag = order.iter().position(|t| *t == "tag").unwrap();
    let widget = order.iter().position(|t| *t == "widget").unwrap();
    assert!(tag < widget, "tag must precede widget: {order:?}");
}

#[test]
fn order_is_complete() {
    let order = apply_order::<Lib>().unwrap();
    assert_eq!(order.len(), 2);
}
