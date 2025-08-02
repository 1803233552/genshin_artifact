use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{CharacterName, CharacterStaticData, Character};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Ineffa;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::{StatName, WeaponType};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::{weapon_common_data::WeaponCommonData, Weapon};
use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::enemies::Enemy;

pub struct IneffaDefaultTargetFunction {
    pub overclocking_rate: f64,
}

impl TargetFunctionMetaTrait for IneffaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::IneffaDefault,
        name_locale: locale!(
            zh_cn: "伊涅芙-月光机师",
            en: "Ineffa-Mechanical Moon"
        ),
        description: locale!(
            zh_cn: "普通输出伊涅芙",
            en: "Normal DPS Ineffa"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Ineffa),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "overclocking_rate",
            title: locale!(
                zh_cn: "频率超限回路触发比例",
                en: "Overclocking Circuit Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.8 }
        }
    ]);

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let overclocking_rate = match *config {
            TargetFunctionConfig::IneffaDefault { overclocking_rate } => overclocking_rate,
            _ => 0.8
        };

        Box::new(IneffaDefaultTargetFunction {
            overclocking_rate
        })
    }
}

impl TargetFunction for IneffaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.0,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.3,
            elemental_mastery: 0.5,
            critical: 1.0,
            critical_damage: 1.0,
            healing_bonus: 0.0,
            bonus_electro: 2.0,
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_cryo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::ElementalMastery,
                StatName::Recharge,
            ],
            goblet_main_stats: vec![
                StatName::ElectroBonus,
                StatName::ATKPercentage,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                ArtifactSetName::GildedDreams,
                ArtifactSetName::WanderersTroupe,
                ArtifactSetName::ThunderingFury,
                ArtifactSetName::GladiatorsFinale,
            ]),
            very_critical_set_names: None,
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD,
        }
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <Ineffa as CharacterTrait>::DamageEnumType;
        let config = CharacterSkillConfig::Ineffa { overclocking_active: true };

        let dmg_normal1 = Ineffa::damage_internal::<SimpleDamageBuilder>(&context, S::Normal1 as usize, &config, None);
        let dmg_normal2 = Ineffa::damage_internal::<SimpleDamageBuilder>(&context, S::Normal2 as usize, &config, None);
        let dmg_normal3 = Ineffa::damage_internal::<SimpleDamageBuilder>(&context, S::Normal3 as usize, &config, None);
        let dmg_normal4 = Ineffa::damage_internal::<SimpleDamageBuilder>(&context, S::Normal4 as usize, &config, None);
        let dmg_charged = Ineffa::damage_internal::<SimpleDamageBuilder>(&context, S::Charged as usize, &config, None);
        let dmg_skill = Ineffa::damage_internal::<SimpleDamageBuilder>(&context, S::Skill as usize, &config, None);
        let dmg_burst = Ineffa::damage_internal::<SimpleDamageBuilder>(&context, S::Burst as usize, &config, None);

        let normal_dmg = dmg_normal1.normal.expectation + dmg_normal2.normal.expectation 
                        + dmg_normal3.normal.expectation + dmg_normal4.normal.expectation;
        let skill_dmg = dmg_skill.normal.expectation * (1.0 + self.overclocking_rate); // 考虑频率超限回路
        let burst_dmg = dmg_burst.normal.expectation;

        normal_dmg * 0.4 + skill_dmg * 0.8 + burst_dmg * 0.6 + dmg_charged.normal.expectation * 0.3
    }
}
