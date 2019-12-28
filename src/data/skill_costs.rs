use postgres::{binary_copy::BinaryCopyInWriter, types::Type, Client, Error};
use serde::Deserialize;

use super::types::{Amount, Collection, ID};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct SkillCost {
    pub(super) skill_level_id: ID,
    pub(super) material_id: ID,
    pub(super) amount: Amount,
}

pub(super) type SkillCosts = Collection<SkillCost>;

impl SkillCosts {
    pub(super) fn create_table(client: &mut Client) -> Result<(), Error> {
        client.execute(
            "
            CREATE TABLE skill_costs (
                id SERIAL PRIMARY KEY,
                skill_level_id INTEGER REFERENCES skill_levels(id) NOT NULL,
                material_id INTEGER REFERENCES materials(id) NOT NULL,
                amount INTEGER NOT NULL,
                UNIQUE (skill_level_id, material_id)
            )
            ",
            &[],
        )?;
        Ok(())
    }

    pub(super) fn drop_table(client: &mut Client) -> Result<(), Error> {
        client.execute("DROP TABLE IF EXISTS skill_costs", &[])?;
        Ok(())
    }

    pub(super) fn save(self: &Self, client: &mut Client) -> Result<(), Error> {
        let writer = client.copy_in(
            "COPY skill_costs (id, skill_level_id, material_id, amount) FROM stdin WITH BINARY",
        )?;
        let mut writer =
            BinaryCopyInWriter::new(writer, &[Type::INT4, Type::INT4, Type::INT4, Type::INT4]);

        for (k, v) in self.iter() {
            writer.write(&[
                &k as &i32,
                &v.skill_level_id as &i32,
                &v.material_id as &i32,
                &v.amount as &i32,
            ])?
        }

        writer.finish()?;

        Ok(())
    }
}
