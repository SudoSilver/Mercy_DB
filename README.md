## Overview 

MercyDB is a database system build to be simple yet effective.
The PRIORITY of the project is being extremely basic, simple and lightweight.
The system is primarely educational it can be used in production but I do not recommend it.

## Technical Process

1. The Schema system:
After creating a schema file (eg. Schema.msc) call the Schema::from(path) method where path is 
the path to the schema file.

Schema::from is going to tokenize and parse the schema and return a Schema struct to be given 
to the DB::from() method to create the actual database.

2. The Database system
IN DEV

## Example schema

```
Account {
username = string
password = string
age = uint
}
```

## Planned features
Writting to the DB
Not having the DB loaded in ram all the time
make a querry engine

## Notes from the dev

The system is designed to be simple and effective for small datasets 
for large datasets or sensitive data or production I recommend sql or MongoDB instead.
This is primarely an educational project and meant to be used as such.

Feedback is welcome but be respectful and keep the project purpose in mind.
