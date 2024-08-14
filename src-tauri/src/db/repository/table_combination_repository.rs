use crate::db::{
    db_actions::{DbActions, Table},
    model::table_combination::TableCombination,
    pg_driver::PgDriver,
};

pub struct TableCombinationRepository;

impl TableCombinationRepository {}

impl<R1: Table<M1>, R2: Table<M2>, M1, M2> Table<TableCombination<R1, R2, M1, M2>>
    for TableCombinationRepository
{
    fn get_name() -> String {
        format!("{}_{}", R1::get_name(), R2::get_name())
    }

    fn get_fmt_cols() -> String {
        format!("{}, {}", R1::get_fk_uuid_name(), R2::get_fk_uuid_name())
    }

    fn get_fk_uuid_name() -> String {
        format!("")
    }

    fn get_fmt_cols_no_id() -> String {
        format!("")
    }

    fn get_fmt_vals(model: &TableCombination<R1, R2, M1, M2>) -> String {
        format!("'{}', '{}'", model.uuid1, model.uuid2)
    }

    fn get_fmt_vals_no_id(model: &TableCombination<R1, R2, M1, M2>) -> String {
        format!("")
    }
}

impl<R1: Table<M1>, R2: Table<M2>, M1, M2> DbActions<TableCombination<R1, R2, M1, M2>, Self>
    for TableCombinationRepository
{
    fn store(driver: &mut PgDriver, model: TableCombination<R1, R2, M1, M2>) -> anyhow::Result<()> {
        Self::insert(driver, model)
    }

    fn update(
        _driver: &mut PgDriver,
        model: TableCombination<R1, R2, M1, M2>,
    ) -> anyhow::Result<()> {
        unimplemented!(
            "You can't update a combination entry, as it does only consist of two uuids."
        )
    }

    fn remove(
        driver: &mut PgDriver,
        model: TableCombination<R1, R2, M1, M2>,
    ) -> anyhow::Result<()> {
        Self::delete_spec_col(driver, R1::get_fk_uuid_name(), model.uuid1.to_string())
    }

    fn retrieve(
        driver: &mut PgDriver,
        condition: Option<String>,
        model: TableCombination<R1, R2, M1, M2>,
    ) -> Vec<TableCombination<R1, R2, M1, M2>> {
        let mut res: Vec<TableCombination<R1, R2, M1, M2>> = vec![];

        let rows = Self::read(driver, Self::get_name().as_str(), condition);

        for row in rows {
            let uuid1 = row.get(R1::get_fk_uuid_name().as_str());
            let uuid2 = row.get(R2::get_fk_uuid_name().as_str());
            res.push(TableCombination::new(uuid1, uuid2))
        }

        res
    }
}
