use postgres::{binary_copy::BinaryCopyInWriter, types::Type, Client, Error};
use serde::Deserialize;

use super::types::{Collection, Rarity, ID};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct Servant {
    pub(super) name: String,
    pub(super) rarity: Rarity,
    pub(super) icon: String,
    pub(super) class_id: ID,
}

pub(super) type Servants = Collection<Servant>;

impl Servants {
    pub(super) fn create_table(client: &mut Client) -> Result<(), Error> {
        client.execute(
            "
            CREATE TABLE servants (
                id INTEGER PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                icon TEXT NOT NULL UNIQUE,
                      rarity INTEGER NOT NULL,
                class_id INTEGER REFERENCES classes(id) NOT NULL,
                      UNIQUE (class_id, rarity, name)
              )
            ",
            &[],
        )?;
        Ok(())
    }

    pub(super) fn drop_table(client: &mut Client) -> Result<(), Error> {
        client.execute("DROP TABLE IF EXISTS servants", &[])?;
        Ok(())
    }

    pub(super) fn save(self: &Self, client: &mut Client) -> Result<(), Error> {
        let writer = client
            .copy_in("COPY servants (id, name, rarity, icon, class_id) FROM stdin WITH BINARY")?;
        let mut writer = BinaryCopyInWriter::new(
            writer,
            &[Type::INT4, Type::TEXT, Type::INT4, Type::TEXT, Type::INT4],
        );

        for (k, v) in self.iter() {
            writer.write(&[
                &k as &i32,
                &v.name,
                &v.rarity as &i32,
                &v.icon,
                &v.class_id as &i32,
            ])?
        }

        writer.finish()?;

        Ok(())
    }
}
