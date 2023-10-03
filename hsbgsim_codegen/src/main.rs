use heck::{ToSnakeCase, ToUpperCamelCase};
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use std::io::{ErrorKind, Write};
use std::{error::Error, fs::File};

#[derive(Serialize, Deserialize)]
struct Entities {
    meta: Meta,
    data: Data,
}

#[derive(Serialize, Deserialize)]
struct Meta {
    date: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
struct Data {
    minions: Vec<Minion>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Minion {
    pub name: String,
    pub name_short: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub types: Vec<Option<String>>,
    pub pool: String,
    pub pools: Vec<String>,
    pub tier: u8,
    pub attack: u8,
    pub health: u8,
    pub text: Option<String>,
    pub attack_golden: u8,
    pub health_golden: u8,
    pub text_golden: Option<String>,
    pub is_active: bool,
    pub is_token: bool,
    pub abilities: Abilities,
    pub id: String,
    pub summon_id: Option<String>,
    pub picture: String,
    pub picture_small: String,
    pub artist: Option<String>,
    pub flavor: Option<String>,
    pub websites: Websites,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Abilities {
    pub has_battlecry: bool,
    pub has_deathrattle: bool,
    pub has_taunt: bool,
    pub has_shield: bool,
    pub has_windfury: bool,
    pub has_venomous: bool,
    pub has_reborn: bool,
    pub has_avenge: bool,
    pub has_magnetic: bool,
    pub has_spellcraft: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Websites {
    pub blizzard: Option<String>,
    pub bgknowhow: Option<String>,
    pub fandom: Option<String>,
    pub hearthpwn: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let entities: Entities =
        reqwest::blocking::get("https://bgknowhow.com/bgjson/output/bg_entities_all.json")?
            .json()?;

    let mut modules = Vec::new();
    let mut variants = Vec::new();

    for minion in entities.data.minions.iter() {
        let module_name = minion.name.clone().to_snake_case();
        let module_ident = format_ident!("{}", module_name);
        modules.push(module_ident);
        let minion_name = minion.name.clone().to_upper_camel_case();
        let minion_ident = format_ident!("{}", minion_name);
        variants.push(minion_ident);
        let mut res = File::options()
            .write(true)
            .create_new(true)
            .open(&format!("src/minions/variants/{}.rs", module_name));

        fn remove_keywords(text: &str) -> String {
            text.replace("Taunt", "")
                .replace("Divine Shield", "")
                .replace("Windfury", "")
                .replace("Venomous", "")
                .replace("Reborn", "")
                .replace(|c: char| !c.is_ascii_alphanumeric(), "")
        }

        let event_handlers =
            if remove_keywords(minion.text.as_deref().unwrap_or_default()).is_empty() {
                quote! {
                    EventHandlers {
                        implemented: true,
                        ..Default::default()
                    }
                }
            } else {
                quote! {EventHandlers::default()}
            };
        let golden_event_handlers =
            if remove_keywords(minion.text_golden.as_deref().unwrap_or_default()).is_empty() {
                quote! {
                    EventHandlers {
                        implemented: true,
                        ..Default::default()
                    }
                }
            } else {
                quote! {EventHandlers::default()}
            };
        match &mut res {
            Err(err) => {
                if err.kind() != ErrorKind::AlreadyExists {
                    res?;
                }
            }
            Ok(file) => {
                let variant_code = quote! {
                    use crate::events::EventHandlers;

                    pub fn event_handlers() -> EventHandlers {
                        #event_handlers
                    }

                    pub fn golden_event_handlers() -> EventHandlers {
                        #golden_event_handlers
                    }
                };
                let variant_file: syn::File = syn::parse2(variant_code)?;
                let variant_file_string = prettyplease::unparse(&variant_file);
                file.write_all(variant_file_string.as_bytes())?;
            }
        }
        let Minion {
            name,
            attack,
            health,
            attack_golden,
            health_golden,
            abilities,
            types,
            ..
        } = minion;
        let battlecry = abilities.has_battlecry.then_some(true).into_iter();
        let deathrattle = abilities.has_deathrattle.then_some(true).into_iter();
        let taunt = abilities.has_taunt.then_some(true).into_iter();
        let shield = abilities.has_shield.then_some(true).into_iter();
        let windfury = abilities.has_windfury.then_some(true).into_iter();
        let venomous = abilities.has_venomous.then_some(true).into_iter();
        let reborn = abilities.has_reborn.then_some(true).into_iter();
        let avenge = abilities.has_avenge.then_some(true).into_iter();
        let magnetic = abilities.has_magnetic.then_some(true).into_iter();
        let spellcraft = abilities.has_spellcraft.then_some(true).into_iter();

        let minion_types = types.into_iter().flatten().map(|s| format_ident!("{}", s));

        let data_code = quote! {
            use super::super::MinionVariantData;
            #[allow(unused_imports)]
            use crate::minions::{AbilitiesInit, MinionType};

            use tinyvec::array_vec;

            pub fn data() -> MinionVariantData {
                MinionVariantData {
                    name: String::from(#name),
                    attack: #attack,
                    health: #health,
                    attack_golden: #attack_golden,
                    health_golden: #health_golden,
                    abilities: AbilitiesInit {
                        #(battlecry: #battlecry,)*
                        #(deathrattle: #deathrattle,)*
                        #(taunt: #taunt,)*
                        #(shield: #shield,)*
                        #(windfury: #windfury,)*
                        #(venomous: #venomous,)*
                        #(reborn: #reborn,)*
                        #(avenge: #avenge,)*
                        #(magnetic: #magnetic,)*
                        #(spellcraft: #spellcraft,)*
                        //#(stealth: #stealth)?,
                        ..Default::default()
                    }
                    .init(),
                    minion_types: array_vec![_ => #(MinionType::#minion_types,)*],
                }
            }

        };

        let data_file: syn::File = syn::parse2(data_code)?;
        let data_file_string = prettyplease::unparse(&data_file);
        let mut file = File::create(&format!("src/minions/variants/data/{}.rs", module_name))?;
        file.write_all(data_file_string.as_bytes())?;
    }

    let num_variants = variants.len();

    let mod_code = quote! {
        use super::{Abilities, MinionInstance, MinionType, Position};
        use crate::events::EventHandlers;

        use rand::seq::SliceRandom;
        use strum::EnumString;
        use tinyvec::ArrayVec;

        mod data;
        #(pub mod #modules;)*

        pub struct MinionVariantData {
            pub name: String,
            pub attack: u8,
            pub health: u8,
            pub attack_golden: u8,
            pub health_golden: u8,
            pub abilities: Abilities,
            pub minion_types: ArrayVec<[MinionType; 2]>,
        }

        impl MinionVariant {
            pub fn into_instance(self, golden: bool) -> MinionInstance {
                let data = self.data();

                let (base_health, base_attack) =
                if golden {
                    (data.health_golden as i32, data.attack_golden as i32)
                } else {
                    (data.health as i32, data.attack as i32)
                };

                MinionInstance {
                    variant: self,
                    golden,
                    base_health,
                    base_attack,
                    aura_health: 0,
                    aura_attack: 0,
                    abilities: data.abilities,
                    position: Position::default(),
                    pending_destroy: false,
                    event_handlers: self.event_handlers(),
                }
            }
        }

        #[derive(
            Clone,
            Copy,
            Default,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            EnumString,
            serde::Serialize,
            serde::Deserialize,
        )]
        pub enum MinionVariant {
            #[default]
            Invalid,
            #(#variants),*
        }

        impl MinionVariant {
            pub const ALL: [MinionVariant; #num_variants] = [#(MinionVariant::#variants),*];

            pub fn data(self) -> MinionVariantData {
                match self {
                    MinionVariant::Invalid => panic!("Invalid MinionVariant"),
                    #(MinionVariant::#variants => data::#modules::data()),*
                }
            }

            pub fn event_handlers(self) -> EventHandlers {
                match self {
                    MinionVariant::Invalid => panic!("Invalid MinionVariant"),
                    #(MinionVariant::#variants => #modules::event_handlers()),*
                }
            }

            pub fn golden_event_handlers(self) -> EventHandlers {
                match self {
                    MinionVariant::Invalid => panic!("Invalid MinionVariant"),
                    #(MinionVariant::#variants => #modules::golden_event_handlers()),*
                }
            }

            pub fn random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
                *Self::ALL.choose(rng).unwrap()
            }
        }
    };
    let variants_mod_file: syn::File = syn::parse2(mod_code.clone())?;
    let variants_mod_file_string = prettyplease::unparse(&variants_mod_file);
    let mut file = File::create("src/minions/variants/mod.rs")?;
    file.write_all(variants_mod_file_string.as_bytes())?;

    Ok(())
}
