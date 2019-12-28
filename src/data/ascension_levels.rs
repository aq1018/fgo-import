use postgres::{binary_copy::BinaryCopyInWriter, types::Type, Client, Error};
use serde::Deserialize;

use super::types::{Collection, Level, ID};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct AscensionLevel {
    pub(super) ascend_to: Level,
    pub(super) servant_id: ID,
}

pub(super) type AscensionLevels = Collection<AscensionLevel>;

impl AscensionLevels {
    pub(super) fn create_table(client: &mut Client) -> Result<(), Error> {
        client.execute(
            "
            CREATE TABLE ascension_levels (
                id SERIAL PRIMARY KEY,
                servant_id INTEGER REFERENCES servants(id) NOT NULL,
                ascend_to INTEGER NOT NULL,
                UNIQUE (servant_id, ascend_to)
            )
            ",
            &[],
        )?;
        Ok(())
    }

    pub(super) fn drop_table(client: &mut Client) -> Result<(), Error> {
        client.execute("DROP TABLE IF EXISTS ascension_levels", &[])?;
        Ok(())
    }

    pub(super) fn save(self: &Self, client: &mut Client) -> Result<(), Error> {
        let writer = client
            .copy_in("COPY ascension_levels (id, servant_id, ascend_to) FROM stdin WITH BINARY")?;
        let mut writer = BinaryCopyInWriter::new(writer, &[Type::INT4, Type::INT4, Type::INT4]);

        for (id, v) in self.iter() {
            writer.write(&[&id as &i32, &v.servant_id as &i32, &v.ascend_to as &i32])?
        }

        writer.finish()?;

        Ok(())
    }
}
