use std::iter;

use structsy::{Structsy, StructsyError, StructsyIter, StructsyTx, Ref};
use structsy_derive::{queries, Persistent};

#[derive(Persistent, Debug, PartialEq)]
pub struct PageData {
    #[index(mode = "cluster")]
    path: String,
    views: i32,
}

#[queries(PageData)]
trait PageDataQuery {
    /// The parameters name have two match the field names and type
    /// like the `address` parameter match the `address` field of the struct.
    fn search(self, path: String) -> Self;
}

pub fn add_view(path: &str) -> Result<i32, StructsyError> {
    // Open the database once
    let db = Structsy::open("data.db")?;
    db.define::<PageData>()?;

    // Pass the database instance to get_views_data
    let mut iter = get_views_data(&db, path)?;

    let mut tx = db.begin()?;
    if let Some((id, mut data)) = iter.next() {
        data.views += 1;
        println!("{:?}", data);
        tx.delete(&id)?;
        tx.insert(&data)?;
        tx.commit()?;

        Ok(data.views)
    } else {
        tx.insert(&PageData { path: path.to_string(), views: 1 })?;
        tx.commit()?;

        Ok(1)
    }
}

pub fn get_views_data(db: &Structsy, path: &str) -> Result<StructsyIter<'static, (Ref<PageData>, PageData)>, StructsyError> {
    let mut iter = db.query::<PageData>().search(path.to_string()).into_iter();
    
    Ok(iter)
}

pub fn get_views(path: &str) -> Result<i32, StructsyError> {
    // Open the database once
    let db = Structsy::open("data.db")?;
    let mut iter = get_views_data(&db, path)?;
    
    if let Some((_id, data)) = iter.next() {
        println!("{:?}", data);
        Ok(data.views)
    } else {
        Ok(0)
    }
}
