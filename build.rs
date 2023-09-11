use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::Write;
use std::path::Path;

use heck::ToSnakeCase;
use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use serde::Deserialize;
use serde::Serialize;

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
    println!("cargo:rerun-if-changed=data/bg_entities_all.json");

    let out_dir = std::env::var_os("OUT_DIR").unwrap();

    let file = File::open("data/bg_entities_all.json")?;
    let reader = BufReader::new(file);

    let entities: Entities = serde_json::from_reader(reader)?;

    //TODO: create files for all minion variants, then create another file for all variants with their definition, which gets included by the first file.
    //TODO: remove take(5)

    let mut modules = Vec::new();
    let mut variants = Vec::new();

    for minion in entities.data.minions.iter().take(5) {
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
        match &mut res {
            Err(err) => {
                if err.kind() != ErrorKind::AlreadyExists {
                    res?;
                }
            }
            Ok(file) => {
                let variant_code = quote! {
                    use crate::events::EventHandler;

                    pub fn event_handler() -> EventHandler {
                        EventHandler::default()
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
            abilities,
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

        let data_code = quote! {
            use super::super::MinionVariantData;
            use crate::minions::AbilitiesInit;

            pub fn data() -> MinionVariantData {
                MinionVariantData {
                    name: String::from(#name),
                    health: #health,
                    attack: #attack,
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
                }
            }

        };

        let data_file: syn::File = syn::parse2(data_code)?;
        let data_file_string = prettyplease::unparse(&data_file);
        let mut file = File::create(&format!("src/minions/variants/data/{}.rs", module_name))?;
        file.write_all(data_file_string.as_bytes())?;
    }

    //let mut mod_code = quote! {#(pub mod #modules;)*};
    //let data_mod_file: syn::File = syn::parse2(mod_code.clone())?;
    //let data_mod_file_string = prettyplease::unparse(&data_mod_file);
    //let mut file = File::create("src/minions/variants/data/mod.rs")?;
    //file.write_all(data_mod_file_string.as_bytes())?;

    let mod_code = quote! {
        #[derive(Clone, Copy)]
        pub enum MinionVariant {
            Invalid,
            #(#variants),*
        }

        impl MinionVariant {
            pub fn data(self) -> MinionVariantData {
                match self {
                    MinionVariant::Invalid => panic!("Invalid MinionVariant"),
                    #(MinionVariant::#variants => data::#modules::data()),*
                }
            }

            pub fn event_handler(self) -> EventHandler {
                match self {
                    MinionVariant::Invalid => panic!("Invalid MinionVariant"),
                    #(MinionVariant::#variants => #modules::event_handler()),*
                }
            }

            pub fn random() -> Self {
                *[#(MinionVariant::#variants),*].choose(&mut rand::thread_rng()).unwrap()
            }
        }
    };
    let variants_mod_file: syn::File = syn::parse2(mod_code.clone())?;
    let variants_mod_file_string = prettyplease::unparse(&variants_mod_file);
    let dest_path = Path::new(&out_dir).join("variants.rs");
    let mut file = File::create(dest_path)?;
    file.write_all(variants_mod_file_string.as_bytes())?;

    Ok(())
}
