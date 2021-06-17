pub mod tables;
pub mod error;

#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Data {
        name: String,
        inner: Vec<String>
    }

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

    #[test]
    fn save_and_restore() {
        let data = Data {
            name: "first".into(),
            inner: vec!("a".into(), "b".into(), "c".into())
        };

        let tmp = tempfile::Builder::new().prefix("test-db").tempdir().unwrap();
        std::fs::create_dir_all(tmp.path()).unwrap();

        let db = DataDb::new(tmp.path());
        assert!(db.is_ok());
        let db = db.unwrap();

        assert!(data.save_rewrite(&db).is_ok());

        let restored = Data::restore("first", &db);
        assert!(restored.is_ok());

        assert_eq!(data, restored.unwrap());
    }
}
