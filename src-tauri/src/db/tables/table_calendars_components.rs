use std::marker::PhantomData;
use uuid::Uuid;
use crate::db_actions::{DbActions, Table};
use crate::pg_driver::PgDriver;

pub struct TableCombination<T1: Table, T2: Table> {
    col1: String,
    col2: String,
    phantom1: PhantomData<T1>,
    phantom2: PhantomData<T2>,
}

impl<T1: Table, T2: Table> TableCombination<T1, T2> {
    pub fn new(col1: String, col2: String) -> Self {
        Self {
            col1,
            col2,
            phantom1: Default::default(),
            phantom2: Default::default(),
        }
    }
}

impl<T1: Table, T2: Table> Table for TableCombination<T1, T2> {
    fn get_name() -> String {
        format!("{}_{}", T1::get_name(), T2::get_name())
    }

    fn get_fmt_cols() -> String {
        todo!("We have a slight problem with whats static and what isn't...");
        // format!("{}, {}", col)
    }

    fn get_fmt_cols_no_id() -> String {
        unimplemented!("A combination consists of uuids.")
    }

    fn get_fmt_vals(&self) -> String {
        todo!("We have a slight problem with whats static and what isn't...");
        // format!("'{}', '{}'", self.uuid1, self.uuid2)
    }

    fn get_fmt_vals_no_id(&self) -> String {
        unimplemented!("A combination consists of uuids.")
    }
}

impl<T1: Table, T2: Table> DbActions for TableCombination<T1, T2> {
    type Item = Self;

    fn store(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        Self::insert(driver, self)
    }

    fn update(&self, _driver: &mut PgDriver) -> anyhow::Result<()> {
        unimplemented!("You can't update a combination entry, as it does only consist of two uuids.")
    }

    fn remove(&self, driver: &mut PgDriver) -> anyhow::Result<()> {
        todo!("We have a slight problem with whats static and what isn't...");
        // Self::delete::<&Self>(driver, self.uuid1)
    }

    fn retrieve(driver: &mut PgDriver, mut cols: Vec<String>, condition: Option<String>) -> Vec<Self::Item> {
        // Implement the retrieve logic here
    }
}
