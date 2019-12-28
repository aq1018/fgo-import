use postgres::{binary_copy::BinaryCopyInWriter, types::Type, Client, Error};
use serde::Deserialize;

use super::types::Collection;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct Class {
    name: String,
    icon: String,
}

pub(super) type Classes = Collection<Class>;

impl Classes {
    pub(super) fn create_table(client: &mut Client) -> Result<(), Error> {
        client.execute(
            "
            CREATE TABLE classes (
                id SERIAL PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                icon TEXT NOT NULL UNIQUE
            )
            ",
            &[],
        )?;
        Ok(())
    }

    pub(super) fn drop_table(client: &mut Client) -> Result<(), Error> {
        client.execute("DROP TABLE IF EXISTS classes", &[])?;
        Ok(())
    }

    pub(super) fn save(self: &Self, client: &mut Client) -> Result<(), Error> {
        let writer = client.copy_in("COPY classes (id, name, icon) FROM stdin WITH BINARY")?;
        let mut writer = BinaryCopyInWriter::new(writer, &[Type::INT4, Type::TEXT, Type::TEXT]);

        for (id, v) in self.iter() {
            writer.write(&[&id as &i32, &v.name, &v.icon])?
        }

        writer.finish()?;

        Ok(())
    }
}
