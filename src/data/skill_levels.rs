use postgres::{binary_copy::BinaryCopyInWriter, types::Type, Client, Error};
use serde::Deserialize;

use super::types::{Collection, Level, ID};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct SkillLevel {
    pub(super) level_to: Level,
    pub(super) servant_id: ID,
}

pub(super) type SkillLevels = Collection<SkillLevel>;

impl SkillLevels {
    pub(super) fn create_table(client: &mut Client) -> Result<(), Error> {
        client.execute(
            "
            CREATE TABLE skill_levels (
                id SERIAL PRIMARY KEY,
                servant_id INTEGER REFERENCES servants(id) NOT NULL,
                level_to INTEGER NOT NULL,
                UNIQUE (servant_id, level_to)
            )
            ",
            &[],
        )?;
        Ok(())
    }

    pub(super) fn drop_table(client: &mut Client) -> Result<(), Error> {
        client.execute("DROP TABLE IF EXISTS skill_levels", &[])?;
        Ok(())
    }

    pub(super) fn save(self: &Self, client: &mut Client) -> Result<(), Error> {
        let writer = client
            .copy_in("COPY skill_levels (id, servant_id, level_to) FROM stdin WITH BINARY")?;
        let mut writer = BinaryCopyInWriter::new(writer, &[Type::INT4, Type::INT4, Type::INT4]);

        for (k, v) in self.iter() {
            writer.write(&[&k as &i32, &v.servant_id as &i32, &v.level_to as &i32])?
        }

        writer.finish()?;

        Ok(())
    }
}
