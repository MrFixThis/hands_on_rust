mod actors;

use actors::{BackPack, Object};

#[tokio::main]
async fn main() {
    let bp = BackPack::new(60.0);
    let objs = vec![
        Object::new("obj 1".to_owned(), 7, 600.0, 77.0),
        Object::new("obj 2".to_owned(), 2, 485.0, 11.0),
        Object::new("obj 3".to_owned(), 4, 195.0, 13.0),
    ];
    actors::show_backpack_content(bp.insert_objs_frag(objs).await);
}
