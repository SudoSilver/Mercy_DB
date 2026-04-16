## Overview 

MercyDB is a database system build to be simple yet effective.
The PRIORITY of the project is being extremely basic, simple and lightweight.
The system is primarely educational it can be used in production but I do not recommend it.

## Use

The crate offers a struct named DB that is:
```
pub struct DB {
    entries: HashMap<String, Vec<String>>,
    entrie_size: usize,
    path: String,
}
```

and has 2 functions that can be called on it
`::from()` and `.push()`

`::from()` takes arguments
1. path (&str) which is the path to the DB file 
2. es (usize) the size of each row (Vec<String>)

`.push()` takes arguments:
1. name (String) the name of the new row
2. row (Vec<String>) the contents of the new row

## Notes from the dev

The system is designed to be simple and effective for small datasets 
for large datasets or sensitive data or production I recommend sql or MongoDB instead.
This is primarely an educational project and meant to be used as such.

Feedback is welcome but be respectful and keep the project purpose in mind.
