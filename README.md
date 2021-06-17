# `sled-tables`

Intended to abstract storage of associated sets of data as `Vec` or `HashSet` seamlessly using serde_cbor serialization.

#Usage

For your data struct (should implement `Serialize and Deserialize`):
```rust
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Data {
        name: String,
        inner: Vec<String>
    }

    impl Data {
       	fn save_rewrite(&self, db: &DataDb) -> Result<(), super::error::Error> {
            db.values.put(db.names.designated_key(&self.name), self.inner.clone())
        }
        
        fn restore(name: &str, db: &DataDb) -> Result<Self, super::error::Error> {
            if let Ok(Some(inner)) = db.values.get(db.names.designated_key(&name.into())) {
                Ok(Data {
                    name: name.into(),
                    inner
                })
            } else {
                Err(super::error::Error::NoDataError)
            }
        }
    }
```
Implement struct to link your data with the db:

```rust
    struct DataDb {
        pub names: super::tables::SledEventTree<String>,
        pub values: super::tables::SledEventTreeVec<String>,
    }

    impl DataDb {
        fn new<'p, P>(path: P) -> Result<Self, super::error::Error>
            where P: Into<&'p std::path::Path>{
            let db = sled::open(path.into())?;
            Ok(Self {
                names: super::tables::SledEventTree::new(db.open_tree(b"names")?),
                values: super::tables::SledEventTreeVec::new(db.open_tree(b"values")?),
            })
        }
    }
```
That's it - you're good to go:

```rust
        let data = Data {
            name: "first".into(),
            inner: vec!("a".into(), "b".into(), "c".into())
        };

		// Create database somewhere.
        let db = DataDb::new(tmp.path())?;

		// Store your data.
		data.save_rewrite(&db)?;

		// Read your data.
        let restored = Data::restore("first", &db);
```

Bunch of other methods are present - go read the docs ;)

#License: Apache v2.0
