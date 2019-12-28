use postgres::{binary_copy::BinaryCopyInWriter, types::Type, Client, Error};
use serde::Deserialize;

use super::types::{Collection, Position};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct Material {
    pub(super) name: String,
    pub(super) icon: String,
    pub(super) order: Position,
}

pub(super) type Materials = Collection<Material>;

impl Materials {
    pub(super) fn create_table(client: &mut Client) -> Result<(), Error> {
        client.execute(
            "
            CREATE TABLE materials (
                id SERIAL PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                icon TEXT NOT NULL UNIQUE,
                position INTEGER UNIQUE
            )
            ",
            &[],
        )?;
        Ok(())
    }

    pub(super) fn drop_table(client: &mut Client) -> Result<(), Error> {
        client.execute("DROP TABLE IF EXISTS materials", &[])?;
        Ok(())
    }

    pub(super) fn save(self: &Self, client: &mut Client) -> Result<(), Error> {
        let writer =
            client.copy_in("COPY materials (id, name, icon, position) FROM stdin WITH BINARY")?;
        let mut writer =
            BinaryCopyInWriter::new(writer, &[Type::INT4, Type::TEXT, Type::TEXT, Type::INT4]);

        for (k, v) in self.iter() {
            writer.write(&[&k as &i32, &v.name, &v.icon, &v.order as &i32])?
        }

        writer.finish()?;

        Ok(())
    }
}
