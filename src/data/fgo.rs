use postgres::{Client, Error as DbError};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use super::ascension_costs::AscensionCosts;
use super::ascension_levels::AscensionLevels;
use super::classes::Classes;
use super::materials::Materials;
use super::servants::Servants;
use super::skill_costs::SkillCosts;
use super::skill_levels::SkillLevels;
use super::types::ID;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Fgo {
    servants: Servants,
    classes: Classes,
    materials: Materials,
    ascension_levels: AscensionLevels,
    ascension_costs: AscensionCosts,
    skill_levels: SkillLevels,
    skill_costs: SkillCosts,
}

impl Fgo {
    pub(crate) fn from_file<P: AsRef<Path>>(path: P) -> Result<Fgo, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut result: Fgo = serde_json::from_reader(reader)?;

        result.cleanup();

        Ok(result)
    }

    pub(crate) fn reset_schema(client: &mut Client) -> Result<(), DbError> {
        Self::drop_schema(client)?;
        Self::create_schema(client)?;

        Ok(())
    }

    pub(crate) fn save(self: &Self, client: &mut Client) -> Result<(), DbError> {
        self.materials.save(client)?;
        self.classes.save(client)?;
        self.servants.save(client)?;
        self.skill_levels.save(client)?;
        self.ascension_levels.save(client)?;
        self.skill_costs.save(client)?;
        self.ascension_costs.save(client)?;

        Ok(())
    }

    pub(crate) fn create_schema(client: &mut Client) -> Result<(), DbError> {
        Materials::create_table(client)?;
        Classes::create_table(client)?;
        Servants::create_table(client)?;
        SkillLevels::create_table(client)?;
        AscensionLevels::create_table(client)?;
        SkillCosts::create_table(client)?;
        AscensionCosts::create_table(client)?;

        Ok(())
    }

    pub(crate) fn drop_schema(client: &mut Client) -> Result<(), DbError> {
        AscensionCosts::drop_table(client)?;
        SkillCosts::drop_table(client)?;
        AscensionLevels::drop_table(client)?;
        SkillLevels::drop_table(client)?;
        Servants::drop_table(client)?;
        Classes::drop_table(client)?;
        Materials::drop_table(client)?;

        Ok(())
    }

    fn cleanup(self: &mut Self) {
        let invalid_skill_level_ids: Vec<ID> = self
            .skill_levels
            .iter()
            .filter(|(_, v)| !self.servants.contains_key(&v.servant_id))
            .map(|(k, _)| *k)
            .collect();

        for id in invalid_skill_level_ids.iter() {
            (&mut self.skill_levels).remove(id);
        }

        let invalid_ascension_level_ids: Vec<ID> = self
            .ascension_levels
            .iter()
            .filter(|(_, v)| !self.servants.contains_key(&v.servant_id))
            .map(|(k, _)| *k)
            .collect();

        for id in invalid_ascension_level_ids.iter() {
            (&mut self.skill_levels).remove(id);
        }
    }
}
