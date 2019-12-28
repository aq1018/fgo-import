use postgres::{binary_copy::BinaryCopyInWriter, types::Type, Client, Error};
use serde::Deserialize;

use super::types::{Amount, Collection, ID};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct AscensionCost {
    pub(super) ascension_level_id: ID,
    pub(super) material_id: ID,
    pub(super) amount: Amount,
}

pub(super) type AscensionCosts = Collection<AscensionCost>;

impl AscensionCosts {
    pub(super) fn create_table(client: &mut Client) -> Result<(), Error> {
        client.execute(
            "
            CREATE TABLE ascension_costs (
                id SERIAL PRIMARY KEY,
                ascension_level_id INTEGER REFERENCES ascension_levels(id) NOT NULL,
                material_id INTEGER REFERENCES materials(id) NOT NULL,
                amount INTEGER NOT NULL,
                UNIQUE (ascension_level_id, material_id)
            )
            ",
            &[],
        )?;
        Ok(())
    }

    pub(super) fn drop_table(client: &mut Client) -> Result<(), Error> {
        client.execute("DROP TABLE IF EXISTS ascension_costs", &[])?;
        Ok(())
    }

    pub(super) fn save(self: &Self, client: &mut Client) -> Result<(), Error> {
        let writer = client
            .copy_in("COPY ascension_costs (id, ascension_level_id, material_id, amount) FROM stdin WITH BINARY")?;
        let mut writer =
            BinaryCopyInWriter::new(writer, &[Type::INT4, Type::INT4, Type::INT4, Type::INT4]);

        for (id, v) in self.iter() {
            writer.write(&[
                &id as &i32,
                &v.ascension_level_id as &i32,
                &v.material_id as &i32,
                &v.amount as &i32,
            ])?
        }

        writer.finish()?;

        Ok(())
    }
}
