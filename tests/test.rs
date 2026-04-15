use mercy_db::*;

fn path(file: &str) -> String {
    format!("{}/tests/{}", env!("CARGO_MANIFEST_DIR"), file)
}

#[test]
fn test_schema() {
    let schema = Schema::from(&path("test.mds"));

    assert_eq!(schema.name, "Test");
    assert_eq!(schema.fields.len(), 3);

    assert_eq!(schema.fields[0].name, "user");
    assert!(matches!(schema.fields[0].ty, Types::StringInstance));

    assert_eq!(schema.fields[1].name, "age");
    assert!(matches!(schema.fields[1].ty, Types::Int));

    assert_eq!(schema.fields[2].name, "admin");
    assert!(matches!(schema.fields[2].ty, Types::Bool));
}

#[test]
fn test_db_from_file() {
    let schema = Schema::from(&path("test.mds"));
    let db = DB::from(schema, &path("test.mdd"));

    assert_eq!(db.schema.name, "Test");
    assert_eq!(db.schema.fields.len(), 3);

    assert_eq!(db.entry_size, 3);
    assert_eq!(db.values.len(), 2);

    // row 1
    match &db.values[0].fields[0] {
        Value::StringInstance(v) => assert_eq!(v, "john"),
        _ => panic!("wrong type"),
    }

    match &db.values[0].fields[1] {
        Value::Int(v) => assert_eq!(*v, 1),
        _ => panic!("wrong type"),
    }

    match &db.values[0].fields[2] {
        Value::Bool(v) => assert_eq!(*v, true),
        _ => panic!("wrong type"),
    }

    // row 2
    match &db.values[1].fields[0] {
        Value::StringInstance(v) => assert_eq!(v, "alice"),
        _ => panic!("wrong type"),
    }

    match &db.values[1].fields[1] {
        Value::Int(v) => assert_eq!(*v, 2),
        _ => panic!("wrong type"),
    }

    match &db.values[1].fields[2] {
        Value::Bool(v) => assert_eq!(*v, false),
        _ => panic!("wrong type"),
    }
}
