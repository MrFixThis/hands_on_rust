use super::*;

#[test]
#[should_panic]
fn incorrect_object_val_weight_is_warned() {
    _ = Object::new("obj".to_owned(), 1, 0.0, 45.0);
}

#[test]
#[should_panic]
fn incorrect_object_frag_quantity_is_warned() {
    _ = Object::new("obj".to_owned(), 0, 255.0, 45.0);
}

#[tokio::test]
async fn entire_objects_are_saved_correctly() {
    let bp = BackPack::new(100.0);
    let objs = vec![
        Object::new("obj 1".to_owned(), 5, 300.0, 45.0),
        Object::new("obj 2".to_owned(), 2, 530.0, 50.0),
        Object::new("obj 3".to_owned(), 1, 200.0, 15.0),
    ];

    assert_eq!(
        bp.insert_objs(objs).await.objs,
        vec![
            Object::new("obj 3".to_owned(), 1, 200.0, 15.0),
            Object::new("obj 2".to_owned(), 2, 530.0, 50.0)
        ]
    );
}

#[tokio::test]
async fn revenue_from_entire_objects_is_calculated_propertly() {
    let bp = BackPack::new(100.0);
    let objs = vec![
        Object::new("obj 1".to_owned(), 5, 300.0, 45.0),
        Object::new("obj 2".to_owned(), 2, 530.0, 50.0),
        Object::new("obj 3".to_owned(), 1, 200.0, 15.0),
    ];
    let bp = bp.insert_objs(objs).await;

    assert_eq!(bp.cal_revenue(), 730.0);
}

#[tokio::test]
async fn fragmented_objects_are_saved_correctly() {
    let bp = BackPack::new(60.0);
    let objs = vec![
        Object::new("obj 1".to_owned(), 5, 300.0, 45.0),
        Object::new("obj 2".to_owned(), 2, 530.0, 50.0),
        Object::new("obj 3".to_owned(), 1, 200.0, 15.0),
    ];

    assert_eq!(
        bp.insert_objs_frag(objs).await.objs,
        vec![
            Object::new("obj 3".to_owned(), 1, 200.0, 15.0),
            Object::new("obj 2".to_owned(), 1, 265.0, 25.0),
            Object::new("obj 1".to_owned(), 2, 120.0, 18.0)
        ]
    );
}

#[tokio::test]
async fn revenue_from_fragmented_objects_is_calculated_propertly() {
    let bp = BackPack::new(60.0);
    let objs = vec![
        Object::new("obj 1".to_owned(), 5, 300.0, 45.0),
        Object::new("obj 2".to_owned(), 2, 530.0, 50.0),
        Object::new("obj 3".to_owned(), 1, 200.0, 15.0),
    ];
    let bp = bp.insert_objs_frag(objs).await;

    assert_eq!(bp.cal_revenue(), 585.0);
}
