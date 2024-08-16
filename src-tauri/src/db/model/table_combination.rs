use std::marker::PhantomData;

use uuid::Uuid;

use crate::db::db_actions::Table;

use super::model::Model;

pub struct TableCombination<R1, R2, M1, M2>
where
    R1: Table<M1>,
    R2: Table<M2>,
    M1: Model,
    M2: Model,
{
    pub(crate) uuid1: Uuid,
    pub(crate) uuid2: Uuid,
    phantom1: PhantomData<R1>,
    phantom2: PhantomData<R2>,
    phantom3: PhantomData<M1>,
    phantom4: PhantomData<M2>,
}

impl<R1, R2, M1, M2> TableCombination<R1, R2, M1, M2>
where
    R1: Table<M1>,
    R2: Table<M2>,
    M1: Model,
    M2: Model,
{
    pub fn new(uuid1: Uuid, uuid2: Uuid) -> Self {
        Self {
            uuid1,
            uuid2,
            phantom1: Default::default(),
            phantom2: Default::default(),
            phantom3: Default::default(),
            phantom4: Default::default(),
        }
    }
}

impl<R1: Table<M1>, R2: Table<M2>, M1: Model, M2: Model> Model
    for TableCombination<R1, R2, M1, M2>
{
}
