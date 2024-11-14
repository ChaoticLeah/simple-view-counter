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
    fn search(self, path: String) -> Self;
}

pub fn add_view(path: &str, db_path: String) -> Result<i32, StructsyError> {
    let db = Structsy::open(db_path)?;
    db.define::<PageData>()?;

    let mut iter = get_views_data(&db, path.to_string())?;

    let mut tx = db.begin()?;
    if let Some((id, mut data)) = iter.next() {
        data.views += 1;
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

pub fn get_views_data(db: &Structsy, path: String) -> Result<StructsyIter<'static, (Ref<PageData>, PageData)>, StructsyError> {
    let iter = db.query::<PageData>().search(path.to_string()).into_iter();
    
    Ok(iter)
}

pub fn get_views(path: &str, db_path: String) -> Result<i32, StructsyError> {
    // Open the database once
    let db = Structsy::open(db_path)?;
    let mut iter = get_views_data(&db, path.to_string())?;
    
    if let Some((_id, data)) = iter.next() {
        Ok(data.views)
    } else {
        Ok(0)
    }
}
