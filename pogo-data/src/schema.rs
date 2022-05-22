use serde::de::IgnoredAny;
use serde::Deserialize;

/** Root structure (not public) */
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub(crate) struct GameMasterTemplate {
    #[allow(dead_code)]
    pub template_id: String,
    pub data: TemplateData,
}

// FIXME this should be an `enum`
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TemplateData {
    #[allow(dead_code)]
    pub template_id: String,
    pub pokemon_settings: Option<PokemonSettings>,
    pub pokemon_family: Option<PokemonFamily>,
    pub form_settings: Option<FormSettings>,
    pub gender_settings: Option<GenderSettings>,
}

/** Pokemon: pokemonSettings */
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PokemonSettings {
    pub pokemon_id: String,
    pub model_scale: Option<f32>,
    pub model_scale_v2: f32,
    #[serde(rename = "type")]
    pub type1: String,
    pub type2: Option<String>,
    pub camera: MonsterCamera,
    pub encounter: MonsterEncounter,
    pub stats: MonsterBaseStats,
    // E.g. Smeargle has no moves defined
    #[serde(default)]
    pub quick_moves: Vec<String>,
    #[serde(default)]
    pub cinematic_moves: Vec<String>,
    #[serde(default)]
    pub animation_time: Vec<f32>,
    #[serde(default)]
    pub evolution_ids: Vec<String>,
    pub evolution_pips: Option<u32>,       // XXX always value 1
    pub parent_pokemon_id: Option<String>, // Pokemon name
    pub candy_to_evolve: Option<u32>,
    #[serde(default)]
    pub evolution_branch: Vec<MonsterEvolution>,
    #[serde(default)]
    pub temp_evo_overrides: Vec<TempEvolution>,
    #[serde(default)]
    pub form_change: Vec<MonsterFormChange>,
    #[serde(default)]
    pub ob_costume_evolution: Vec<String>, // "APRIL_2020_NOEVOLVE"
    pub rarity: Option<String>, // TODO enum
    pub pokedex_height_m: Option<f32>,
    pub pokedex_weight_kg: Option<f32>,
    pub height_std_dev: Option<f32>,
    pub weight_std_dev: Option<f32>,
    pub family_id: String,
    pub km_buddy_distance: f32,
    pub buddy_size: Option<String>, // enum
    pub model_height: f32,
    pub buddy_offset_male: Option<[f32; 3]>,
    pub buddy_offset_female: Option<[f32; 3]>,
    pub buddy_portrait_offset: Option<[f32; 3]>,
    pub combat_shoulder_camera_angle: Option<[f32; 3]>,
    pub combat_default_camera_angle: Option<[f32; 3]>,
    pub combat_opponent_focus_camera_angle: Option<[f32; 3]>,
    pub combat_player_focus_camera_angle: Option<[f32; 3]>,
    pub combat_player_pokemon_position_offset: Option<[f32; 3]>,
    pub raid_boss_distance_offset: Option<f32>,
    pub buddy_scale: Option<f32>,
    pub third_move: MonsterThirdMove,
    #[serde(default)]
    pub is_transferable: bool,
    #[serde(default)]
    pub is_tradable: bool,
    #[serde(default)]
    pub is_deployable: bool,
    pub shadow: Option<MonsterShadow>,
    pub buddy_group_number: Option<u32>,
    #[serde(default)]
    pub elite_quick_move: Vec<String>,
    #[serde(default)]
    pub elite_cinematic_move: Vec<String>,
    pub buddy_walked_mega_energy_award: Option<u32>,
    pub form: Option<String>,
    #[serde(default)]
    pub disable_transfer_to_pokemon_home: bool,
    #[serde(default)]
    pub ob_preview_pokemon_setting: IgnoredAny,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum MonsterBaseStats {
    #[serde(rename_all = "camelCase")]
    Some {
        base_stamina: u32,
        base_attack: u32,
        base_defense: u32,
    },
    /// Monsters that aren't available in PoGo have missing base stats.
    #[serde(rename_all = "camelCase")]
    Missing {},
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MonsterThirdMove {
    // XXX Smeargle cannot learn a 3rd move... But is default=0 correct?
    #[serde(default)]
    pub stardust_to_unlock: u32,
    pub candy_to_unlock: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MonsterShadow {
    pub purification_stardust_needed: u32,
    pub purification_candy_needed: u32,
    pub purified_charge_move: String,
    pub shadow_charge_move: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum MonsterEvolution {
    #[serde(rename_all = "camelCase")]
    Evolution {
        evolution: String,    // Pokemon name
        form: Option<String>, // Form name
        priority: Option<u32>,
        // Evolution requirements
        candy_cost: u32,
        #[serde(default)]
        no_candy_cost_via_trade: bool,
        evolution_item_requirement: Option<String>,
        lure_item_requirement: Option<String>,
        km_buddy_distance_requirement: Option<f32>,
        gender_requirement: Option<String>,
        #[serde(default)]
        must_be_buddy: bool,
        #[serde(default)]
        only_daytime: bool,
        #[serde(default)]
        only_nighttime: bool,
        #[serde(default)]
        only_upside_down: bool,
        #[serde(default)]
        quest_display: Vec<EvolutionQuestDisplay>,
        ob_purification_evolution_candy_cost: Option<u32>,
    },
    // XXX ugly, but stricter than making all fields optional
    #[serde(rename_all = "camelCase")]
    Purified {
        ob_purification_evolution_candy_cost: u32,
    },
    #[serde(rename_all = "camelCase")]
    Temporary {
        temporary_evolution: String, // "TEMP_EVOLUTION_MEGA",
        temporary_evolution_energy_cost: u32,
        temporary_evolution_energy_cost_subsequent: u32,
        ob_purification_evolution_candy_cost: Option<u32>,
    },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EvolutionQuestDisplay {
    pub quest_requirement_template_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TempEvolution {
    // TODO! Stub only
    pub temp_evo_id: String, // "TEMP_EVOLUTION_MEGA"
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MonsterFormChange {
    pub available_form: Vec<String>,
    pub candy_cost: u32,
    pub stardust_cost: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MonsterCamera {
    pub disk_radius_m: Option<f32>,
    pub cylinder_radius_m: Option<f32>,
    pub cylinder_height_m: Option<f32>,
    pub cylinder_ground_m: Option<f32>,
    pub shoulder_mode_scale: Option<f32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MonsterEncounter {
    // TODO! Stub only
}

/** Pokemon family: pokemonFamily */
/*

*/
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PokemonFamily {
    pub family_id: String,
    pub candy_per_xl_candy: u32,
    pub mega_evolvable_pokemon_id: Option<String>,
}

/** Pokemon form: formSettings */
/*
    "pokemon": "CHARMANDER",
    "forms": [
        {
            "form": "CHARMANDER_NORMAL"
        },
        {
            "form": "CHARMANDER_FALL_2019",
            "assetBundleSuffix": "pm0004_00_pgo_fall2019",
            "isCostume": true
        }
    ]
*/
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FormSettings {
    pub pokemon: String,
    #[serde(default)]
    pub forms: Vec<MonsterForm>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MonsterForm {
    pub form: String,
    #[serde(default)]
    pub is_costume: bool,
    pub asset_bundle_value: Option<i32>,
    pub asset_bundle_suffix: Option<String>,
}

/** Pokemon gender: genderSettings */
/*
    "pokemon": "TOGEKISS",
    "gender": {
        "malePercent": 0.875,
        "femalePercent": 0.125
    }
*/
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GenderSettings {
    pub pokemon: String,
    pub gender: MonsterGender,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
pub enum MonsterGender {
    #[serde(rename_all = "camelCase")]
    Genderless { genderless_percent: f32 },
    #[serde(rename_all = "camelCase")]
    Gendered {
        #[serde(default)]
        male_percent: f32,
        #[serde(default)]
        female_percent: f32,
    },
}
