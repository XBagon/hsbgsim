use super::{Abilities, MinionInstance, Position};
use crate::events::EventHandlers;
use rand::seq::SliceRandom;
use strum::EnumString;
pub mod accord_o_tron;
pub mod acolyte_of_c_thun;
pub mod adaptable_barricade;
pub mod agamaggan_the_great_boar;
pub mod aggem_thorncurse;
pub mod alleycat;
pub mod amalgadon;
pub mod amalgam;
pub mod amber_guardian;
pub mod annihilan_battlemaster;
pub mod annoy_o_module;
pub mod annoy_o_spawn;
pub mod annoy_o_tron;
pub mod annoy_o_troupe;
pub mod anub_arak_nerubian_king;
pub mod archdruid_hamuul;
pub mod argent_braggart;
pub mod arm_of_the_empire;
pub mod atramedes;
pub mod baby_krush;
pub mod backpiggy_imp;
pub mod backstage_security;
pub mod ball_of_minions;
pub mod baltharak;
pub mod banana_slamma;
pub mod bannerboar;
pub mod baron_rivendare;
pub mod bassgill;
pub mod bejeweled_duelist;
pub mod bigfernal;
pub mod bird_buddy;
pub mod blade_collector;
pub mod blazing_skyfin;
pub mod bloodsail_cannoneer;
pub mod bonemare;
pub mod bongo_bopper;
pub mod bonker;
pub mod brann_bronzebeard;
pub mod bream_counter;
pub mod briarback_bookie;
pub mod briny_bootlegger;
pub mod bristlebach;
pub mod bristleback_brute;
pub mod bristleback_knight;
pub mod bristlemane_scrapsmith;
pub mod bronze_warden;
pub mod bubblette;
pub mod budding_greenthumb;
pub mod cap_n_hoggarr;
pub mod captain_flat_tusk;
pub mod captain_sanders;
pub mod carbonic_copy;
pub mod cave_hydra;
pub mod champion_of_sargeras;
pub mod champion_of_the_primus;
pub mod champion_of_y_shaarj;
pub mod charlga;
pub mod choral_mrrrglr;
pub mod chronormu;
pub mod cobalt_scalebane;
pub mod cogwork_copter;
pub mod coldlight_seer;
pub mod colossus_of_the_sun;
pub mod corpse_refiner;
pub mod corrupted_myrmidon;
pub mod crab;
pub mod crackling_cyclone;
pub mod critter_wrangler;
pub mod cubling;
pub mod cyborg_drake;
pub mod daggerspine_thrasher;
pub mod damaged_golem;
pub mod dancing_barnstormer;
pub mod darkgaze_elder;
mod data;
pub mod dazzling_lightspawn;
pub mod deadly_spore;
pub mod deck_swabbie;
pub mod deep_blue_crooner;
pub mod deep_sea_angler;
pub mod defender_of_argus;
pub mod deflect_o_bot;
pub mod devilsaur;
pub mod diablo_lord_of_terror;
pub mod disco_shuffler;
pub mod dozy_whelp;
pub mod dr_boombox;
pub mod drakkari_enchanter;
pub mod drakonid_enforcer;
pub mod dread_admiral_eliza;
pub mod dreadbeard;
pub mod dynamic_duo;
pub mod eelbound_archer;
pub mod electric_synthesizer;
pub mod elemental_of_surprise;
pub mod emergent_flame;
pub mod emperor_cobra;
pub mod eternal_knight;
pub mod eternal_summoner;
pub mod eventide_brute;
pub mod evolving_chromawing;
pub mod faceless_disciple;
pub mod fairy_tale_caroler;
pub mod famished_felbat;
pub mod felemental;
pub mod felfin_navigator;
pub mod felstomper;
pub mod fiery_imp;
pub mod fireworks_fanatic;
pub mod first_mate_pip;
pub mod fish_of_n_zoth;
pub mod fleet_admiral_tethys;
pub mod floating_watcher;
pub mod flourishing_frostling;
pub mod foe_reaper_4000;
pub mod free_flying_feathermane;
pub mod freedealing_gambler;
pub mod friend_of_a_friend;
pub mod gem_smuggler;
pub mod gemsplitter;
pub mod general_drakkisath;
pub mod gentle_djinni;
pub mod ghastcoiler;
pub mod ghoul_of_the_feast;
pub mod glowscale;
pub mod glyph_guardian;
pub mod golden_monkey;
pub mod goldgrubber;
pub mod goldrinn_the_great_wolf;
pub mod granite_guardian;
pub mod grease_bot;
pub mod greta_gold_gun;
pub mod groundshaker;
pub mod gunpowder_courier;
pub mod gusty_trumpeter;
pub mod half_shell;
pub mod handless_forsaken;
pub mod harvest_golem;
pub mod helping_hand;
pub mod holy_mecherel;
pub mod houndmaster;
pub mod humming_bird;
pub mod hungering_abomination;
pub mod hungry_snapjaw;
pub mod hunter_of_gatherers;
pub mod hyena;
pub mod icky_imp;
pub mod imp;
pub mod imp_mama;
pub mod impatient_doomsayer;
pub mod imposing_percussionist;
pub mod imprisoner;
pub mod impulsive_trickster;
pub mod incorporeal_corporal;
pub mod insatiable_ur_zul;
pub mod interrogator_whitemane;
pub mod invent_o_matic;
pub mod iron_groundskeeper;
pub mod iron_sensei;
pub mod jelly_belly;
pub mod kaboom_bot;
pub mod kalecgos_arcane_aspect;
pub mod kangor_s_apprentice;
pub mod kathra_natir;
pub mod keyboard_igniter;
pub mod khadgar;
pub mod king_bagurgle;
pub mod king_varian;
pub mod kooky_chemist;
pub mod lava_lurker;
pub mod leapfrogger;
pub mod leeching_felhound;
pub mod leeroy_the_reckless;
pub mod legion_overseer;
pub mod lich_doctor;
pub mod lieutenant_garr;
pub mod lightfang_enforcer;
pub mod lil_rag;
pub mod living_constellation;
pub mod lovesick_balladist;
pub mod low_flier;
pub mod lullabot;
pub mod maexxna;
pub mod magmaloc;
pub mod magtheridon_prime;
pub mod majordomo_executus;
pub mod malchezaar_prince_of_dance;
pub mod mama_bear;
pub mod manasaber;
pub mod mannoroth;
pub mod mantid_queen;
pub mod master_of_realities;
pub mod mecha_jaraxxus;
pub mod mechanized_gift_horse;
pub mod mechano_egg;
pub mod mechano_tank;
pub mod mechapony;
pub mod mechorse;
pub mod menagerie_jug;
pub mod menagerie_mug;
pub mod metaltooth_leaper;
pub mod micro_mummy;
pub mod microbot;
pub mod mind_muck;
pub mod mini_myrmidon;
pub mod mistake;
pub mod moira_bronzebeard;
pub mod molten_rock;
pub mod monstrous_macaw;
pub mod moon_bacon_jazzer;
pub mod murcules;
pub mod murky;
pub mod murloc_warleader;
pub mod murozond;
pub mod mythrax_the_unraveler;
pub mod nadina_the_red;
pub mod nathrezim_overseer;
pub mod necrolyte;
pub mod nerubian_deathswarmer;
pub mod nether_drake;
pub mod nightmare_amalgam;
pub mod niuzao;
pub mod nomi_kitchen_nightmare;
pub mod nosy_looter;
pub mod obsidian_ravager;
pub mod octosari_wrap_god;
pub mod omega_buster;
pub mod onyxian_whelp;
pub mod oozeling_gladiator;
pub mod operatic_belcher;
pub mod orgozoa_the_tender;
pub mod ozumat_s_tentacle;
pub mod palescale_crocolisk;
pub mod papa_bear;
pub mod party_elemental;
pub mod pashmar_the_vengeful;
pub mod patient_scout;
pub mod peckish_feldrake;
pub mod peggy_brittlebone;
pub mod peggy_sturdybone;
pub mod picky_eater;
pub mod piggyback_imp;
pub mod plagued_tidewalker;
pub mod plant;
pub mod poetic_pen_pal;
pub mod polarizing_beatboxer;
pub mod possessive_banshee;
pub mod prestor_s_pyrospawn;
pub mod prickly_piper;
pub mod primalfin_lookout;
pub mod prized_promo_drake;
pub mod prophet_of_the_boar;
pub mod pufferquil;
pub mod pupbot;
pub mod qiraji_harbinger;
pub mod rabid_saurolisk;
pub mod radio_star;
pub mod rat;
pub mod rat_pack;
pub mod razorfen_geomancer;
pub mod razorgore_the_untamed;
pub mod reanimating_rattler;
pub mod record_smuggler;
pub mod recurring_nightmare;
pub mod recycling_wraith;
pub mod red_whelp;
pub mod reef_explorer;
pub mod reef_riffer;
pub mod refreshing_anomaly;
pub mod relentless_mur_ghoul;
pub mod relentless_sentry;
pub mod rendle_the_mistermind;
pub mod replicating_menace;
pub mod ring_matron;
pub mod ripsnarl_captain;
pub mod risen_rider;
pub mod roadboar;
pub mod robosaur;
pub mod rock_rock;
pub mod rockpool_hunter;
pub mod rot_hide_gnoll;
pub mod rusted_reggie;
pub mod rylak_metalhead;
pub mod saltscale_honcho;
pub mod salty_looter;
pub mod sanctum_rester;
pub mod sanguine_champion;
pub mod savannah_highmane;
pub mod scallywag;
pub mod scarlet_skull;
pub mod scavenging_hyena;
pub mod scourfin;
pub mod scrap_scraper;
pub mod screwjank_clunker;
pub mod sea_witch_zar_jira;
pub mod seaborn_summoner;
pub mod seafood_slinger;
pub mod selfless_hero;
pub mod sellemental;
pub mod sewer_rat;
pub mod shell_collector;
pub mod shifter_zerus;
pub mod shoal_commander;
pub mod shudderling;
pub mod si_sefin;
pub mod silent_swimmer;
pub mod silverback_patriarch;
pub mod sin_dorei_straight_shot;
pub mod sinrunner_blanchy;
pub mod sister_deathwhisper;
pub mod sky_pirate;
pub mod sly_raptor;
pub mod smogger;
pub mod smolderwing;
pub mod snail_cavalry;
pub mod snake;
pub mod sore_loser;
pub mod soul_devourer;
pub mod soul_juggler;
pub mod soul_rewinder;
pub mod soulsplitter;
pub mod southsea_busker;
pub mod southsea_captain;
pub mod southsea_strongarm;
pub mod spark_ling;
pub mod sparring_partner;
pub mod spawn_of_n_zoth;
pub mod stasis_elemental;
pub mod steward_of_time;
pub mod stone_elemental;
pub mod stormbringer;
pub mod stormscale_siren;
pub mod strongshell_scavenger;
pub mod sun_bacon_relaxer;
pub mod surf_n_surf;
pub mod swampstriker;
pub mod swolefin;
pub mod tabbycat;
pub mod tad;
pub mod tarecgosa;
pub mod tavern_tempest;
pub mod tavern_tipper;
pub mod tea_master_theotar;
pub mod tentacle_of_octosari;
pub mod the_boogie_monster;
pub mod the_boommobile;
pub mod the_glad_iator;
pub mod the_walking_fort;
pub mod thorncaller;
pub mod thorncaptain;
pub mod tichondrius;
pub mod tide_oracle_morgl;
pub mod tidemistress_athissa;
pub mod time_saver;
pub mod titus_rivendare;
pub mod tony_two_tusk;
pub mod tortollan_blue_shell;
pub mod tough_tusk;
pub mod toxfin;
pub mod transmuted_bramblewitch;
pub mod treasure_seeker_elise;
pub mod tunnel_blaster;
pub mod twilight_emissary;
pub mod underhanded_dealer;
pub mod unstable_ghoul;
pub mod upbeat_duo;
pub mod upbeat_flutist;
pub mod upbeat_frontdrake;
pub mod upbeat_impressionist;
pub mod upbeat_upstart;
pub mod uther_the_lightbringer;
pub mod utility_drone;
pub mod vanessa_van_cleef;
pub mod vigilant_stoneborn;
pub mod voidlord;
pub mod warden_of_old;
pub mod wargear;
pub mod warpwing;
pub mod water_droplet;
pub mod waverider;
pub mod whelp_smuggler;
pub mod wildfire_elemental;
pub mod witchwing_nestmatron;
pub mod withered_spearhide;
pub mod wrath_weaver;
pub mod xylo_bones;
pub mod yo_ho_ogre;
pub mod young_murk_eye;
pub mod yrel;
pub mod zapp_slywick;
pub mod zesty_shaker;
pub struct MinionVariantData {
    pub name: String,
    pub health: u8,
    pub attack: u8,
    pub abilities: Abilities,
}
impl MinionVariant {
    pub fn into_instance(self) -> MinionInstance {
        let data = self.data();
        MinionInstance {
            variant: self,
            health: data.health as i32,
            attack: data.attack as i32,
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
    Alleycat,
    AnnoyOTron,
    Bubblette,
    DeckSwabbie,
    DozyWhelp,
    EvolvingChromawing,
    IckyImp,
    Imprisoner,
    IncorporealCorporal,
    Manasaber,
    MicroMummy,
    MiniMyrmidon,
    Mistake,
    PickyEater,
    Pupbot,
    RazorfenGeomancer,
    RedWhelp,
    RefreshingAnomaly,
    RisenRider,
    RockpoolHunter,
    RotHideGnoll,
    Scallywag,
    ScavengingHyena,
    Sellemental,
    ShellCollector,
    SilverbackPatriarch,
    SouthseaBusker,
    SunBaconRelaxer,
    SurfNSurf,
    Swampstriker,
    TavernTipper,
    Thorncaptain,
    UpbeatFrontdrake,
    WrathWeaver,
    AcolyteOfCThun,
    BackstageSecurity,
    BejeweledDuelist,
    BlazingSkyfin,
    BriarbackBookie,
    CogworkCopter,
    CorpseRefiner,
    DeepSeaAngler,
    EternalKnight,
    FlourishingFrostling,
    FreedealingGambler,
    GlyphGuardian,
    HarvestGolem,
    HummingBird,
    HungrySnapjaw,
    ImpulsiveTrickster,
    InventOMatic,
    KaboomBot,
    KookyChemist,
    LavaLurker,
    Leapfrogger,
    LowFlier,
    Lullabot,
    MenagerieMug,
    MetaltoothLeaper,
    MindMuck,
    MoltenRock,
    Murcules,
    MurlocWarleader,
    NathrezimOverseer,
    NerubianDeathswarmer,
    OozelingGladiator,
    PatientScout,
    PiggybackImp,
    PoeticPenPal,
    ProphetOfTheBoar,
    RabidSaurolisk,
    ReefRiffer,
    RipsnarlCaptain,
    Roadboar,
    SaltscaleHoncho,
    ScarletSkull,
    SeabornSummoner,
    SelflessHero,
    SewerRat,
    SnailCavalry,
    SoulRewinder,
    SouthseaCaptain,
    SparringPartner,
    SpawnOfNZoth,
    StewardOfTime,
    Tad,
    Thorncaller,
    ToughTusk,
    TwilightEmissary,
    UnstableGhoul,
    UpbeatFlutist,
    WhelpSmuggler,
    YoHoOgre,
    Yrel,
    AccordOTron,
    AmberGuardian,
    ArmOfTheEmpire,
    BirdBuddy,
    BloodsailCannoneer,
    BrinyBootlegger,
    BristlebackBrute,
    BristlemaneScrapsmith,
    BronzeWarden,
    BuddingGreenthumb,
    ColdlightSeer,
    CracklingCyclone,
    DaggerspineThrasher,
    DeflectOBot,
    Dreadbeard,
    EventideBrute,
    FacelessDisciple,
    Felemental,
    FelfinNavigator,
    FirstMatePip,
    FreeFlyingFeathermane,
    Gemsplitter,
    GhoulOfTheFeast,
    GunpowderCourier,
    HandlessForsaken,
    Houndmaster,
    IronGroundskeeper,
    IronSensei,
    JellyBelly,
    KathraNatir,
    KeyboardIgniter,
    Khadgar,
    LeechingFelhound,
    LegionOverseer,
    LichDoctor,
    LivingConstellation,
    MonstrousMacaw,
    MoonBaconJazzer,
    NetherDrake,
    NightmareAmalgam,
    PartyElemental,
    PashmarTheVengeful,
    PricklyPiper,
    Pufferquil,
    RadioStar,
    RatPack,
    RecyclingWraith,
    RelentlessSentry,
    ReplicatingMenace,
    SaltyLooter,
    Scourfin,
    ScrewjankClunker,
    ShifterZerus,
    ShoalCommander,
    Smogger,
    SoreLoser,
    SoulDevourer,
    SoulJuggler,
    SouthseaStrongarm,
    SparkLing,
    StasisElemental,
    Swolefin,
    Tarecgosa,
    TheGladIator,
    TimeSaver,
    WardenOfOld,
    WitheredSpearhide,
    ZestyShaker,
    AnnihilanBattlemaster,
    AnnoyOModule,
    AnubArakNerubianKing,
    Atramedes,
    BallOfMinions,
    BananaSlamma,
    Bannerboar,
    Bassgill,
    Bigfernal,
    BladeCollector,
    Bonker,
    BreamCounter,
    CarbonicCopy,
    CaveHydra,
    ChampionOfYShaarj,
    CobaltScalebane,
    DancingBarnstormer,
    DazzlingLightspawn,
    DeepBlueCrooner,
    DefenderOfArgus,
    DrakonidEnforcer,
    DynamicDuo,
    EelboundArcher,
    ElectricSynthesizer,
    EmergentFlame,
    FairyTaleCaroler,
    FireworksFanatic,
    FloatingWatcher,
    GemSmuggler,
    Goldgrubber,
    Groundshaker,
    ImpatientDoomsayer,
    LovesickBalladist,
    MajordomoExecutus,
    MalchezaarPrinceOfDance,
    MasterOfRealities,
    MechanoEgg,
    MechanoTank,
    MenagerieJug,
    Necrolyte,
    PeckishFeldrake,
    PeggyBrittlebone,
    PeggySturdybone,
    PlaguedTidewalker,
    PossessiveBanshee,
    PrestorSPyrospawn,
    PrimalfinLookout,
    PrizedPromoDrake,
    QirajiHarbinger,
    ReanimatingRattler,
    ReefExplorer,
    RendleTheMistermind,
    RingMatron,
    RylakMetalhead,
    SavannahHighmane,
    ScrapScraper,
    SilentSwimmer,
    SinDoreiStraightShot,
    SlyRaptor,
    Soulsplitter,
    Stormbringer,
    StrongshellScavenger,
    TreasureSeekerElise,
    TunnelBlaster,
    UpbeatUpstart,
    VigilantStoneborn,
    Wargear,
    Waverider,
    WildfireElemental,
    WitchwingNestmatron,
    XyloBones,
    AdaptableBarricade,
    AgamagganTheGreatBoar,
    AggemThorncurse,
    AnnoyOTroupe,
    BabyKrush,
    BaronRivendare,
    Bonemare,
    BongoBopper,
    BrannBronzebeard,
    BristlebackKnight,
    CapNHoggarr,
    ChampionOfThePrimus,
    Chronormu,
    CorruptedMyrmidon,
    CritterWrangler,
    CyborgDrake,
    DeadlySpore,
    DiscoShuffler,
    DrBoombox,
    DrakkariEnchanter,
    FriendOfAFriend,
    GeneralDrakkisath,
    Glowscale,
    GustyTrumpeter,
    HolyMecherel,
    HungeringAbomination,
    HunterOfGatherers,
    ImposingPercussionist,
    InsatiableUrZul,
    InterrogatorWhitemane,
    KangorSApprentice,
    KingBagurgle,
    LeeroyTheReckless,
    LightfangEnforcer,
    LilRag,
    Magmaloc,
    MamaBear,
    Mannoroth,
    MechanizedGiftHorse,
    Murozond,
    MythraxTheUnraveler,
    Niuzao,
    NomiKitchenNightmare,
    OperaticBelcher,
    PalescaleCrocolisk,
    RazorgoreTheUntamed,
    RecordSmuggler,
    SanctumRester,
    SiSefin,
    SinrunnerBlanchy,
    StormscaleSiren,
    TavernTempest,
    Tichondrius,
    TitusRivendare,
    TonyTwoTusk,
    TortollanBlueShell,
    Toxfin,
    TransmutedBramblewitch,
    UnderhandedDealer,
    UpbeatDuo,
    UpbeatImpressionist,
    UtilityDrone,
    VanessaVanCleef,
    Voidlord,
    ArchdruidHamuul,
    Bristlebach,
    CaptainFlatTusk,
    Charlga,
    ChoralMrrrglr,
    ColossusOfTheSun,
    DarkgazeElder,
    DreadAdmiralEliza,
    ElementalOfSurprise,
    EternalSummoner,
    FamishedFelbat,
    Felstomper,
    FleetAdmiralTethys,
    FoeReaper4000,
    GentleDjinni,
    Ghastcoiler,
    GoldrinnTheGreatWolf,
    GreaseBot,
    GretaGoldGun,
    ImpMama,
    KalecgosArcaneAspect,
    LieutenantGarr,
    Maexxna,
    MantidQueen,
    MechaJaraxxus,
    Murky,
    NadinaTheRed,
    NosyLooter,
    OctosariWrapGod,
    OmegaBuster,
    OrgozoaTheTender,
    PolarizingBeatboxer,
    RelentlessMurGhoul,
    RockRock,
    SeafoodSlinger,
    SisterDeathwhisper,
    TeaMasterTheotar,
    TheBoogieMonster,
    TheWalkingFort,
    TidemistressAthissa,
    UtherTheLightbringer,
    Warpwing,
    YoungMurkEye,
    ZappSlywick,
    Amalgadon,
    ArgentBraggart,
    CaptainSanders,
    ChampionOfSargeras,
    GraniteGuardian,
    KingVarian,
    MoiraBronzebeard,
    ObsidianRavager,
    PapaBear,
    RecurringNightmare,
    SanguineChampion,
    SeaWitchZarJira,
    TheBoommobile,
    TideOracleMorgl,
    Amalgam,
    AnnoyOSpawn,
    BackpiggyImp,
    Baltharak,
    Crab,
    Cubling,
    DamagedGolem,
    Devilsaur,
    DiabloLordOfTerror,
    EmperorCobra,
    FieryImp,
    FishOfNZoth,
    GoldenMonkey,
    HalfShell,
    HelpingHand,
    Hyena,
    Imp,
    MagtheridonPrime,
    Mechapony,
    Mechorse,
    Microbot,
    OnyxianWhelp,
    OzumatSTentacle,
    Plant,
    Rat,
    Robosaur,
    RustedReggie,
    Shudderling,
    SkyPirate,
    Smolderwing,
    Snake,
    StoneElemental,
    Tabbycat,
    TentacleOfOctosari,
    WaterDroplet,
}
impl MinionVariant {
    pub const ALL: [MinionVariant; 390usize] = [
        MinionVariant::Alleycat,
        MinionVariant::AnnoyOTron,
        MinionVariant::Bubblette,
        MinionVariant::DeckSwabbie,
        MinionVariant::DozyWhelp,
        MinionVariant::EvolvingChromawing,
        MinionVariant::IckyImp,
        MinionVariant::Imprisoner,
        MinionVariant::IncorporealCorporal,
        MinionVariant::Manasaber,
        MinionVariant::MicroMummy,
        MinionVariant::MiniMyrmidon,
        MinionVariant::Mistake,
        MinionVariant::PickyEater,
        MinionVariant::Pupbot,
        MinionVariant::RazorfenGeomancer,
        MinionVariant::RedWhelp,
        MinionVariant::RefreshingAnomaly,
        MinionVariant::RisenRider,
        MinionVariant::RockpoolHunter,
        MinionVariant::RotHideGnoll,
        MinionVariant::Scallywag,
        MinionVariant::ScavengingHyena,
        MinionVariant::Sellemental,
        MinionVariant::ShellCollector,
        MinionVariant::SilverbackPatriarch,
        MinionVariant::SouthseaBusker,
        MinionVariant::SunBaconRelaxer,
        MinionVariant::SurfNSurf,
        MinionVariant::Swampstriker,
        MinionVariant::TavernTipper,
        MinionVariant::Thorncaptain,
        MinionVariant::UpbeatFrontdrake,
        MinionVariant::WrathWeaver,
        MinionVariant::AcolyteOfCThun,
        MinionVariant::BackstageSecurity,
        MinionVariant::BejeweledDuelist,
        MinionVariant::BlazingSkyfin,
        MinionVariant::BriarbackBookie,
        MinionVariant::CogworkCopter,
        MinionVariant::CorpseRefiner,
        MinionVariant::DeepSeaAngler,
        MinionVariant::EternalKnight,
        MinionVariant::FlourishingFrostling,
        MinionVariant::FreedealingGambler,
        MinionVariant::GlyphGuardian,
        MinionVariant::HarvestGolem,
        MinionVariant::HummingBird,
        MinionVariant::HungrySnapjaw,
        MinionVariant::ImpulsiveTrickster,
        MinionVariant::InventOMatic,
        MinionVariant::KaboomBot,
        MinionVariant::KookyChemist,
        MinionVariant::LavaLurker,
        MinionVariant::Leapfrogger,
        MinionVariant::LowFlier,
        MinionVariant::Lullabot,
        MinionVariant::MenagerieMug,
        MinionVariant::MetaltoothLeaper,
        MinionVariant::MindMuck,
        MinionVariant::MoltenRock,
        MinionVariant::Murcules,
        MinionVariant::MurlocWarleader,
        MinionVariant::NathrezimOverseer,
        MinionVariant::NerubianDeathswarmer,
        MinionVariant::OozelingGladiator,
        MinionVariant::PatientScout,
        MinionVariant::PiggybackImp,
        MinionVariant::PoeticPenPal,
        MinionVariant::ProphetOfTheBoar,
        MinionVariant::RabidSaurolisk,
        MinionVariant::ReefRiffer,
        MinionVariant::RipsnarlCaptain,
        MinionVariant::Roadboar,
        MinionVariant::SaltscaleHoncho,
        MinionVariant::ScarletSkull,
        MinionVariant::SeabornSummoner,
        MinionVariant::SelflessHero,
        MinionVariant::SewerRat,
        MinionVariant::SnailCavalry,
        MinionVariant::SoulRewinder,
        MinionVariant::SouthseaCaptain,
        MinionVariant::SparringPartner,
        MinionVariant::SpawnOfNZoth,
        MinionVariant::StewardOfTime,
        MinionVariant::Tad,
        MinionVariant::Thorncaller,
        MinionVariant::ToughTusk,
        MinionVariant::TwilightEmissary,
        MinionVariant::UnstableGhoul,
        MinionVariant::UpbeatFlutist,
        MinionVariant::WhelpSmuggler,
        MinionVariant::YoHoOgre,
        MinionVariant::Yrel,
        MinionVariant::AccordOTron,
        MinionVariant::AmberGuardian,
        MinionVariant::ArmOfTheEmpire,
        MinionVariant::BirdBuddy,
        MinionVariant::BloodsailCannoneer,
        MinionVariant::BrinyBootlegger,
        MinionVariant::BristlebackBrute,
        MinionVariant::BristlemaneScrapsmith,
        MinionVariant::BronzeWarden,
        MinionVariant::BuddingGreenthumb,
        MinionVariant::ColdlightSeer,
        MinionVariant::CracklingCyclone,
        MinionVariant::DaggerspineThrasher,
        MinionVariant::DeflectOBot,
        MinionVariant::Dreadbeard,
        MinionVariant::EventideBrute,
        MinionVariant::FacelessDisciple,
        MinionVariant::Felemental,
        MinionVariant::FelfinNavigator,
        MinionVariant::FirstMatePip,
        MinionVariant::FreeFlyingFeathermane,
        MinionVariant::Gemsplitter,
        MinionVariant::GhoulOfTheFeast,
        MinionVariant::GunpowderCourier,
        MinionVariant::HandlessForsaken,
        MinionVariant::Houndmaster,
        MinionVariant::IronGroundskeeper,
        MinionVariant::IronSensei,
        MinionVariant::JellyBelly,
        MinionVariant::KathraNatir,
        MinionVariant::KeyboardIgniter,
        MinionVariant::Khadgar,
        MinionVariant::LeechingFelhound,
        MinionVariant::LegionOverseer,
        MinionVariant::LichDoctor,
        MinionVariant::LivingConstellation,
        MinionVariant::MonstrousMacaw,
        MinionVariant::MoonBaconJazzer,
        MinionVariant::NetherDrake,
        MinionVariant::NightmareAmalgam,
        MinionVariant::PartyElemental,
        MinionVariant::PashmarTheVengeful,
        MinionVariant::PricklyPiper,
        MinionVariant::Pufferquil,
        MinionVariant::RadioStar,
        MinionVariant::RatPack,
        MinionVariant::RecyclingWraith,
        MinionVariant::RelentlessSentry,
        MinionVariant::ReplicatingMenace,
        MinionVariant::SaltyLooter,
        MinionVariant::Scourfin,
        MinionVariant::ScrewjankClunker,
        MinionVariant::ShifterZerus,
        MinionVariant::ShoalCommander,
        MinionVariant::Smogger,
        MinionVariant::SoreLoser,
        MinionVariant::SoulDevourer,
        MinionVariant::SoulJuggler,
        MinionVariant::SouthseaStrongarm,
        MinionVariant::SparkLing,
        MinionVariant::StasisElemental,
        MinionVariant::Swolefin,
        MinionVariant::Tarecgosa,
        MinionVariant::TheGladIator,
        MinionVariant::TimeSaver,
        MinionVariant::WardenOfOld,
        MinionVariant::WitheredSpearhide,
        MinionVariant::ZestyShaker,
        MinionVariant::AnnihilanBattlemaster,
        MinionVariant::AnnoyOModule,
        MinionVariant::AnubArakNerubianKing,
        MinionVariant::Atramedes,
        MinionVariant::BallOfMinions,
        MinionVariant::BananaSlamma,
        MinionVariant::Bannerboar,
        MinionVariant::Bassgill,
        MinionVariant::Bigfernal,
        MinionVariant::BladeCollector,
        MinionVariant::Bonker,
        MinionVariant::BreamCounter,
        MinionVariant::CarbonicCopy,
        MinionVariant::CaveHydra,
        MinionVariant::ChampionOfYShaarj,
        MinionVariant::CobaltScalebane,
        MinionVariant::DancingBarnstormer,
        MinionVariant::DazzlingLightspawn,
        MinionVariant::DeepBlueCrooner,
        MinionVariant::DefenderOfArgus,
        MinionVariant::DrakonidEnforcer,
        MinionVariant::DynamicDuo,
        MinionVariant::EelboundArcher,
        MinionVariant::ElectricSynthesizer,
        MinionVariant::EmergentFlame,
        MinionVariant::FairyTaleCaroler,
        MinionVariant::FireworksFanatic,
        MinionVariant::FloatingWatcher,
        MinionVariant::GemSmuggler,
        MinionVariant::Goldgrubber,
        MinionVariant::Groundshaker,
        MinionVariant::ImpatientDoomsayer,
        MinionVariant::LovesickBalladist,
        MinionVariant::MajordomoExecutus,
        MinionVariant::MalchezaarPrinceOfDance,
        MinionVariant::MasterOfRealities,
        MinionVariant::MechanoEgg,
        MinionVariant::MechanoTank,
        MinionVariant::MenagerieJug,
        MinionVariant::Necrolyte,
        MinionVariant::PeckishFeldrake,
        MinionVariant::PeggyBrittlebone,
        MinionVariant::PeggySturdybone,
        MinionVariant::PlaguedTidewalker,
        MinionVariant::PossessiveBanshee,
        MinionVariant::PrestorSPyrospawn,
        MinionVariant::PrimalfinLookout,
        MinionVariant::PrizedPromoDrake,
        MinionVariant::QirajiHarbinger,
        MinionVariant::ReanimatingRattler,
        MinionVariant::ReefExplorer,
        MinionVariant::RendleTheMistermind,
        MinionVariant::RingMatron,
        MinionVariant::RylakMetalhead,
        MinionVariant::SavannahHighmane,
        MinionVariant::ScrapScraper,
        MinionVariant::SilentSwimmer,
        MinionVariant::SinDoreiStraightShot,
        MinionVariant::SlyRaptor,
        MinionVariant::Soulsplitter,
        MinionVariant::Stormbringer,
        MinionVariant::StrongshellScavenger,
        MinionVariant::TreasureSeekerElise,
        MinionVariant::TunnelBlaster,
        MinionVariant::UpbeatUpstart,
        MinionVariant::VigilantStoneborn,
        MinionVariant::Wargear,
        MinionVariant::Waverider,
        MinionVariant::WildfireElemental,
        MinionVariant::WitchwingNestmatron,
        MinionVariant::XyloBones,
        MinionVariant::AdaptableBarricade,
        MinionVariant::AgamagganTheGreatBoar,
        MinionVariant::AggemThorncurse,
        MinionVariant::AnnoyOTroupe,
        MinionVariant::BabyKrush,
        MinionVariant::BaronRivendare,
        MinionVariant::Bonemare,
        MinionVariant::BongoBopper,
        MinionVariant::BrannBronzebeard,
        MinionVariant::BristlebackKnight,
        MinionVariant::CapNHoggarr,
        MinionVariant::ChampionOfThePrimus,
        MinionVariant::Chronormu,
        MinionVariant::CorruptedMyrmidon,
        MinionVariant::CritterWrangler,
        MinionVariant::CyborgDrake,
        MinionVariant::DeadlySpore,
        MinionVariant::DiscoShuffler,
        MinionVariant::DrBoombox,
        MinionVariant::DrakkariEnchanter,
        MinionVariant::FriendOfAFriend,
        MinionVariant::GeneralDrakkisath,
        MinionVariant::Glowscale,
        MinionVariant::GustyTrumpeter,
        MinionVariant::HolyMecherel,
        MinionVariant::HungeringAbomination,
        MinionVariant::HunterOfGatherers,
        MinionVariant::ImposingPercussionist,
        MinionVariant::InsatiableUrZul,
        MinionVariant::InterrogatorWhitemane,
        MinionVariant::KangorSApprentice,
        MinionVariant::KingBagurgle,
        MinionVariant::LeeroyTheReckless,
        MinionVariant::LightfangEnforcer,
        MinionVariant::LilRag,
        MinionVariant::Magmaloc,
        MinionVariant::MamaBear,
        MinionVariant::Mannoroth,
        MinionVariant::MechanizedGiftHorse,
        MinionVariant::Murozond,
        MinionVariant::MythraxTheUnraveler,
        MinionVariant::Niuzao,
        MinionVariant::NomiKitchenNightmare,
        MinionVariant::OperaticBelcher,
        MinionVariant::PalescaleCrocolisk,
        MinionVariant::RazorgoreTheUntamed,
        MinionVariant::RecordSmuggler,
        MinionVariant::SanctumRester,
        MinionVariant::SiSefin,
        MinionVariant::SinrunnerBlanchy,
        MinionVariant::StormscaleSiren,
        MinionVariant::TavernTempest,
        MinionVariant::Tichondrius,
        MinionVariant::TitusRivendare,
        MinionVariant::TonyTwoTusk,
        MinionVariant::TortollanBlueShell,
        MinionVariant::Toxfin,
        MinionVariant::TransmutedBramblewitch,
        MinionVariant::UnderhandedDealer,
        MinionVariant::UpbeatDuo,
        MinionVariant::UpbeatImpressionist,
        MinionVariant::UtilityDrone,
        MinionVariant::VanessaVanCleef,
        MinionVariant::Voidlord,
        MinionVariant::ArchdruidHamuul,
        MinionVariant::Bristlebach,
        MinionVariant::CaptainFlatTusk,
        MinionVariant::Charlga,
        MinionVariant::ChoralMrrrglr,
        MinionVariant::ColossusOfTheSun,
        MinionVariant::DarkgazeElder,
        MinionVariant::DreadAdmiralEliza,
        MinionVariant::ElementalOfSurprise,
        MinionVariant::EternalSummoner,
        MinionVariant::FamishedFelbat,
        MinionVariant::Felstomper,
        MinionVariant::FleetAdmiralTethys,
        MinionVariant::FoeReaper4000,
        MinionVariant::GentleDjinni,
        MinionVariant::Ghastcoiler,
        MinionVariant::GoldrinnTheGreatWolf,
        MinionVariant::GreaseBot,
        MinionVariant::GretaGoldGun,
        MinionVariant::ImpMama,
        MinionVariant::KalecgosArcaneAspect,
        MinionVariant::LieutenantGarr,
        MinionVariant::Maexxna,
        MinionVariant::MantidQueen,
        MinionVariant::MechaJaraxxus,
        MinionVariant::Murky,
        MinionVariant::NadinaTheRed,
        MinionVariant::NosyLooter,
        MinionVariant::OctosariWrapGod,
        MinionVariant::OmegaBuster,
        MinionVariant::OrgozoaTheTender,
        MinionVariant::PolarizingBeatboxer,
        MinionVariant::RelentlessMurGhoul,
        MinionVariant::RockRock,
        MinionVariant::SeafoodSlinger,
        MinionVariant::SisterDeathwhisper,
        MinionVariant::TeaMasterTheotar,
        MinionVariant::TheBoogieMonster,
        MinionVariant::TheWalkingFort,
        MinionVariant::TidemistressAthissa,
        MinionVariant::UtherTheLightbringer,
        MinionVariant::Warpwing,
        MinionVariant::YoungMurkEye,
        MinionVariant::ZappSlywick,
        MinionVariant::Amalgadon,
        MinionVariant::ArgentBraggart,
        MinionVariant::CaptainSanders,
        MinionVariant::ChampionOfSargeras,
        MinionVariant::GraniteGuardian,
        MinionVariant::KingVarian,
        MinionVariant::MoiraBronzebeard,
        MinionVariant::ObsidianRavager,
        MinionVariant::PapaBear,
        MinionVariant::RecurringNightmare,
        MinionVariant::SanguineChampion,
        MinionVariant::SeaWitchZarJira,
        MinionVariant::TheBoommobile,
        MinionVariant::TideOracleMorgl,
        MinionVariant::Amalgam,
        MinionVariant::AnnoyOSpawn,
        MinionVariant::BackpiggyImp,
        MinionVariant::Baltharak,
        MinionVariant::Crab,
        MinionVariant::Cubling,
        MinionVariant::DamagedGolem,
        MinionVariant::Devilsaur,
        MinionVariant::DiabloLordOfTerror,
        MinionVariant::EmperorCobra,
        MinionVariant::FieryImp,
        MinionVariant::FishOfNZoth,
        MinionVariant::GoldenMonkey,
        MinionVariant::HalfShell,
        MinionVariant::HelpingHand,
        MinionVariant::Hyena,
        MinionVariant::Imp,
        MinionVariant::MagtheridonPrime,
        MinionVariant::Mechapony,
        MinionVariant::Mechorse,
        MinionVariant::Microbot,
        MinionVariant::OnyxianWhelp,
        MinionVariant::OzumatSTentacle,
        MinionVariant::Plant,
        MinionVariant::Rat,
        MinionVariant::Robosaur,
        MinionVariant::RustedReggie,
        MinionVariant::Shudderling,
        MinionVariant::SkyPirate,
        MinionVariant::Smolderwing,
        MinionVariant::Snake,
        MinionVariant::StoneElemental,
        MinionVariant::Tabbycat,
        MinionVariant::TentacleOfOctosari,
        MinionVariant::WaterDroplet,
    ];
    pub fn data(self) -> MinionVariantData {
        match self {
            MinionVariant::Invalid => panic!("Invalid MinionVariant"),
            MinionVariant::Alleycat => data::alleycat::data(),
            MinionVariant::AnnoyOTron => data::annoy_o_tron::data(),
            MinionVariant::Bubblette => data::bubblette::data(),
            MinionVariant::DeckSwabbie => data::deck_swabbie::data(),
            MinionVariant::DozyWhelp => data::dozy_whelp::data(),
            MinionVariant::EvolvingChromawing => data::evolving_chromawing::data(),
            MinionVariant::IckyImp => data::icky_imp::data(),
            MinionVariant::Imprisoner => data::imprisoner::data(),
            MinionVariant::IncorporealCorporal => data::incorporeal_corporal::data(),
            MinionVariant::Manasaber => data::manasaber::data(),
            MinionVariant::MicroMummy => data::micro_mummy::data(),
            MinionVariant::MiniMyrmidon => data::mini_myrmidon::data(),
            MinionVariant::Mistake => data::mistake::data(),
            MinionVariant::PickyEater => data::picky_eater::data(),
            MinionVariant::Pupbot => data::pupbot::data(),
            MinionVariant::RazorfenGeomancer => data::razorfen_geomancer::data(),
            MinionVariant::RedWhelp => data::red_whelp::data(),
            MinionVariant::RefreshingAnomaly => data::refreshing_anomaly::data(),
            MinionVariant::RisenRider => data::risen_rider::data(),
            MinionVariant::RockpoolHunter => data::rockpool_hunter::data(),
            MinionVariant::RotHideGnoll => data::rot_hide_gnoll::data(),
            MinionVariant::Scallywag => data::scallywag::data(),
            MinionVariant::ScavengingHyena => data::scavenging_hyena::data(),
            MinionVariant::Sellemental => data::sellemental::data(),
            MinionVariant::ShellCollector => data::shell_collector::data(),
            MinionVariant::SilverbackPatriarch => data::silverback_patriarch::data(),
            MinionVariant::SouthseaBusker => data::southsea_busker::data(),
            MinionVariant::SunBaconRelaxer => data::sun_bacon_relaxer::data(),
            MinionVariant::SurfNSurf => data::surf_n_surf::data(),
            MinionVariant::Swampstriker => data::swampstriker::data(),
            MinionVariant::TavernTipper => data::tavern_tipper::data(),
            MinionVariant::Thorncaptain => data::thorncaptain::data(),
            MinionVariant::UpbeatFrontdrake => data::upbeat_frontdrake::data(),
            MinionVariant::WrathWeaver => data::wrath_weaver::data(),
            MinionVariant::AcolyteOfCThun => data::acolyte_of_c_thun::data(),
            MinionVariant::BackstageSecurity => data::backstage_security::data(),
            MinionVariant::BejeweledDuelist => data::bejeweled_duelist::data(),
            MinionVariant::BlazingSkyfin => data::blazing_skyfin::data(),
            MinionVariant::BriarbackBookie => data::briarback_bookie::data(),
            MinionVariant::CogworkCopter => data::cogwork_copter::data(),
            MinionVariant::CorpseRefiner => data::corpse_refiner::data(),
            MinionVariant::DeepSeaAngler => data::deep_sea_angler::data(),
            MinionVariant::EternalKnight => data::eternal_knight::data(),
            MinionVariant::FlourishingFrostling => data::flourishing_frostling::data(),
            MinionVariant::FreedealingGambler => data::freedealing_gambler::data(),
            MinionVariant::GlyphGuardian => data::glyph_guardian::data(),
            MinionVariant::HarvestGolem => data::harvest_golem::data(),
            MinionVariant::HummingBird => data::humming_bird::data(),
            MinionVariant::HungrySnapjaw => data::hungry_snapjaw::data(),
            MinionVariant::ImpulsiveTrickster => data::impulsive_trickster::data(),
            MinionVariant::InventOMatic => data::invent_o_matic::data(),
            MinionVariant::KaboomBot => data::kaboom_bot::data(),
            MinionVariant::KookyChemist => data::kooky_chemist::data(),
            MinionVariant::LavaLurker => data::lava_lurker::data(),
            MinionVariant::Leapfrogger => data::leapfrogger::data(),
            MinionVariant::LowFlier => data::low_flier::data(),
            MinionVariant::Lullabot => data::lullabot::data(),
            MinionVariant::MenagerieMug => data::menagerie_mug::data(),
            MinionVariant::MetaltoothLeaper => data::metaltooth_leaper::data(),
            MinionVariant::MindMuck => data::mind_muck::data(),
            MinionVariant::MoltenRock => data::molten_rock::data(),
            MinionVariant::Murcules => data::murcules::data(),
            MinionVariant::MurlocWarleader => data::murloc_warleader::data(),
            MinionVariant::NathrezimOverseer => data::nathrezim_overseer::data(),
            MinionVariant::NerubianDeathswarmer => data::nerubian_deathswarmer::data(),
            MinionVariant::OozelingGladiator => data::oozeling_gladiator::data(),
            MinionVariant::PatientScout => data::patient_scout::data(),
            MinionVariant::PiggybackImp => data::piggyback_imp::data(),
            MinionVariant::PoeticPenPal => data::poetic_pen_pal::data(),
            MinionVariant::ProphetOfTheBoar => data::prophet_of_the_boar::data(),
            MinionVariant::RabidSaurolisk => data::rabid_saurolisk::data(),
            MinionVariant::ReefRiffer => data::reef_riffer::data(),
            MinionVariant::RipsnarlCaptain => data::ripsnarl_captain::data(),
            MinionVariant::Roadboar => data::roadboar::data(),
            MinionVariant::SaltscaleHoncho => data::saltscale_honcho::data(),
            MinionVariant::ScarletSkull => data::scarlet_skull::data(),
            MinionVariant::SeabornSummoner => data::seaborn_summoner::data(),
            MinionVariant::SelflessHero => data::selfless_hero::data(),
            MinionVariant::SewerRat => data::sewer_rat::data(),
            MinionVariant::SnailCavalry => data::snail_cavalry::data(),
            MinionVariant::SoulRewinder => data::soul_rewinder::data(),
            MinionVariant::SouthseaCaptain => data::southsea_captain::data(),
            MinionVariant::SparringPartner => data::sparring_partner::data(),
            MinionVariant::SpawnOfNZoth => data::spawn_of_n_zoth::data(),
            MinionVariant::StewardOfTime => data::steward_of_time::data(),
            MinionVariant::Tad => data::tad::data(),
            MinionVariant::Thorncaller => data::thorncaller::data(),
            MinionVariant::ToughTusk => data::tough_tusk::data(),
            MinionVariant::TwilightEmissary => data::twilight_emissary::data(),
            MinionVariant::UnstableGhoul => data::unstable_ghoul::data(),
            MinionVariant::UpbeatFlutist => data::upbeat_flutist::data(),
            MinionVariant::WhelpSmuggler => data::whelp_smuggler::data(),
            MinionVariant::YoHoOgre => data::yo_ho_ogre::data(),
            MinionVariant::Yrel => data::yrel::data(),
            MinionVariant::AccordOTron => data::accord_o_tron::data(),
            MinionVariant::AmberGuardian => data::amber_guardian::data(),
            MinionVariant::ArmOfTheEmpire => data::arm_of_the_empire::data(),
            MinionVariant::BirdBuddy => data::bird_buddy::data(),
            MinionVariant::BloodsailCannoneer => data::bloodsail_cannoneer::data(),
            MinionVariant::BrinyBootlegger => data::briny_bootlegger::data(),
            MinionVariant::BristlebackBrute => data::bristleback_brute::data(),
            MinionVariant::BristlemaneScrapsmith => data::bristlemane_scrapsmith::data(),
            MinionVariant::BronzeWarden => data::bronze_warden::data(),
            MinionVariant::BuddingGreenthumb => data::budding_greenthumb::data(),
            MinionVariant::ColdlightSeer => data::coldlight_seer::data(),
            MinionVariant::CracklingCyclone => data::crackling_cyclone::data(),
            MinionVariant::DaggerspineThrasher => data::daggerspine_thrasher::data(),
            MinionVariant::DeflectOBot => data::deflect_o_bot::data(),
            MinionVariant::Dreadbeard => data::dreadbeard::data(),
            MinionVariant::EventideBrute => data::eventide_brute::data(),
            MinionVariant::FacelessDisciple => data::faceless_disciple::data(),
            MinionVariant::Felemental => data::felemental::data(),
            MinionVariant::FelfinNavigator => data::felfin_navigator::data(),
            MinionVariant::FirstMatePip => data::first_mate_pip::data(),
            MinionVariant::FreeFlyingFeathermane => data::free_flying_feathermane::data(),
            MinionVariant::Gemsplitter => data::gemsplitter::data(),
            MinionVariant::GhoulOfTheFeast => data::ghoul_of_the_feast::data(),
            MinionVariant::GunpowderCourier => data::gunpowder_courier::data(),
            MinionVariant::HandlessForsaken => data::handless_forsaken::data(),
            MinionVariant::Houndmaster => data::houndmaster::data(),
            MinionVariant::IronGroundskeeper => data::iron_groundskeeper::data(),
            MinionVariant::IronSensei => data::iron_sensei::data(),
            MinionVariant::JellyBelly => data::jelly_belly::data(),
            MinionVariant::KathraNatir => data::kathra_natir::data(),
            MinionVariant::KeyboardIgniter => data::keyboard_igniter::data(),
            MinionVariant::Khadgar => data::khadgar::data(),
            MinionVariant::LeechingFelhound => data::leeching_felhound::data(),
            MinionVariant::LegionOverseer => data::legion_overseer::data(),
            MinionVariant::LichDoctor => data::lich_doctor::data(),
            MinionVariant::LivingConstellation => data::living_constellation::data(),
            MinionVariant::MonstrousMacaw => data::monstrous_macaw::data(),
            MinionVariant::MoonBaconJazzer => data::moon_bacon_jazzer::data(),
            MinionVariant::NetherDrake => data::nether_drake::data(),
            MinionVariant::NightmareAmalgam => data::nightmare_amalgam::data(),
            MinionVariant::PartyElemental => data::party_elemental::data(),
            MinionVariant::PashmarTheVengeful => data::pashmar_the_vengeful::data(),
            MinionVariant::PricklyPiper => data::prickly_piper::data(),
            MinionVariant::Pufferquil => data::pufferquil::data(),
            MinionVariant::RadioStar => data::radio_star::data(),
            MinionVariant::RatPack => data::rat_pack::data(),
            MinionVariant::RecyclingWraith => data::recycling_wraith::data(),
            MinionVariant::RelentlessSentry => data::relentless_sentry::data(),
            MinionVariant::ReplicatingMenace => data::replicating_menace::data(),
            MinionVariant::SaltyLooter => data::salty_looter::data(),
            MinionVariant::Scourfin => data::scourfin::data(),
            MinionVariant::ScrewjankClunker => data::screwjank_clunker::data(),
            MinionVariant::ShifterZerus => data::shifter_zerus::data(),
            MinionVariant::ShoalCommander => data::shoal_commander::data(),
            MinionVariant::Smogger => data::smogger::data(),
            MinionVariant::SoreLoser => data::sore_loser::data(),
            MinionVariant::SoulDevourer => data::soul_devourer::data(),
            MinionVariant::SoulJuggler => data::soul_juggler::data(),
            MinionVariant::SouthseaStrongarm => data::southsea_strongarm::data(),
            MinionVariant::SparkLing => data::spark_ling::data(),
            MinionVariant::StasisElemental => data::stasis_elemental::data(),
            MinionVariant::Swolefin => data::swolefin::data(),
            MinionVariant::Tarecgosa => data::tarecgosa::data(),
            MinionVariant::TheGladIator => data::the_glad_iator::data(),
            MinionVariant::TimeSaver => data::time_saver::data(),
            MinionVariant::WardenOfOld => data::warden_of_old::data(),
            MinionVariant::WitheredSpearhide => data::withered_spearhide::data(),
            MinionVariant::ZestyShaker => data::zesty_shaker::data(),
            MinionVariant::AnnihilanBattlemaster => data::annihilan_battlemaster::data(),
            MinionVariant::AnnoyOModule => data::annoy_o_module::data(),
            MinionVariant::AnubArakNerubianKing => data::anub_arak_nerubian_king::data(),
            MinionVariant::Atramedes => data::atramedes::data(),
            MinionVariant::BallOfMinions => data::ball_of_minions::data(),
            MinionVariant::BananaSlamma => data::banana_slamma::data(),
            MinionVariant::Bannerboar => data::bannerboar::data(),
            MinionVariant::Bassgill => data::bassgill::data(),
            MinionVariant::Bigfernal => data::bigfernal::data(),
            MinionVariant::BladeCollector => data::blade_collector::data(),
            MinionVariant::Bonker => data::bonker::data(),
            MinionVariant::BreamCounter => data::bream_counter::data(),
            MinionVariant::CarbonicCopy => data::carbonic_copy::data(),
            MinionVariant::CaveHydra => data::cave_hydra::data(),
            MinionVariant::ChampionOfYShaarj => data::champion_of_y_shaarj::data(),
            MinionVariant::CobaltScalebane => data::cobalt_scalebane::data(),
            MinionVariant::DancingBarnstormer => data::dancing_barnstormer::data(),
            MinionVariant::DazzlingLightspawn => data::dazzling_lightspawn::data(),
            MinionVariant::DeepBlueCrooner => data::deep_blue_crooner::data(),
            MinionVariant::DefenderOfArgus => data::defender_of_argus::data(),
            MinionVariant::DrakonidEnforcer => data::drakonid_enforcer::data(),
            MinionVariant::DynamicDuo => data::dynamic_duo::data(),
            MinionVariant::EelboundArcher => data::eelbound_archer::data(),
            MinionVariant::ElectricSynthesizer => data::electric_synthesizer::data(),
            MinionVariant::EmergentFlame => data::emergent_flame::data(),
            MinionVariant::FairyTaleCaroler => data::fairy_tale_caroler::data(),
            MinionVariant::FireworksFanatic => data::fireworks_fanatic::data(),
            MinionVariant::FloatingWatcher => data::floating_watcher::data(),
            MinionVariant::GemSmuggler => data::gem_smuggler::data(),
            MinionVariant::Goldgrubber => data::goldgrubber::data(),
            MinionVariant::Groundshaker => data::groundshaker::data(),
            MinionVariant::ImpatientDoomsayer => data::impatient_doomsayer::data(),
            MinionVariant::LovesickBalladist => data::lovesick_balladist::data(),
            MinionVariant::MajordomoExecutus => data::majordomo_executus::data(),
            MinionVariant::MalchezaarPrinceOfDance => data::malchezaar_prince_of_dance::data(),
            MinionVariant::MasterOfRealities => data::master_of_realities::data(),
            MinionVariant::MechanoEgg => data::mechano_egg::data(),
            MinionVariant::MechanoTank => data::mechano_tank::data(),
            MinionVariant::MenagerieJug => data::menagerie_jug::data(),
            MinionVariant::Necrolyte => data::necrolyte::data(),
            MinionVariant::PeckishFeldrake => data::peckish_feldrake::data(),
            MinionVariant::PeggyBrittlebone => data::peggy_brittlebone::data(),
            MinionVariant::PeggySturdybone => data::peggy_sturdybone::data(),
            MinionVariant::PlaguedTidewalker => data::plagued_tidewalker::data(),
            MinionVariant::PossessiveBanshee => data::possessive_banshee::data(),
            MinionVariant::PrestorSPyrospawn => data::prestor_s_pyrospawn::data(),
            MinionVariant::PrimalfinLookout => data::primalfin_lookout::data(),
            MinionVariant::PrizedPromoDrake => data::prized_promo_drake::data(),
            MinionVariant::QirajiHarbinger => data::qiraji_harbinger::data(),
            MinionVariant::ReanimatingRattler => data::reanimating_rattler::data(),
            MinionVariant::ReefExplorer => data::reef_explorer::data(),
            MinionVariant::RendleTheMistermind => data::rendle_the_mistermind::data(),
            MinionVariant::RingMatron => data::ring_matron::data(),
            MinionVariant::RylakMetalhead => data::rylak_metalhead::data(),
            MinionVariant::SavannahHighmane => data::savannah_highmane::data(),
            MinionVariant::ScrapScraper => data::scrap_scraper::data(),
            MinionVariant::SilentSwimmer => data::silent_swimmer::data(),
            MinionVariant::SinDoreiStraightShot => data::sin_dorei_straight_shot::data(),
            MinionVariant::SlyRaptor => data::sly_raptor::data(),
            MinionVariant::Soulsplitter => data::soulsplitter::data(),
            MinionVariant::Stormbringer => data::stormbringer::data(),
            MinionVariant::StrongshellScavenger => data::strongshell_scavenger::data(),
            MinionVariant::TreasureSeekerElise => data::treasure_seeker_elise::data(),
            MinionVariant::TunnelBlaster => data::tunnel_blaster::data(),
            MinionVariant::UpbeatUpstart => data::upbeat_upstart::data(),
            MinionVariant::VigilantStoneborn => data::vigilant_stoneborn::data(),
            MinionVariant::Wargear => data::wargear::data(),
            MinionVariant::Waverider => data::waverider::data(),
            MinionVariant::WildfireElemental => data::wildfire_elemental::data(),
            MinionVariant::WitchwingNestmatron => data::witchwing_nestmatron::data(),
            MinionVariant::XyloBones => data::xylo_bones::data(),
            MinionVariant::AdaptableBarricade => data::adaptable_barricade::data(),
            MinionVariant::AgamagganTheGreatBoar => data::agamaggan_the_great_boar::data(),
            MinionVariant::AggemThorncurse => data::aggem_thorncurse::data(),
            MinionVariant::AnnoyOTroupe => data::annoy_o_troupe::data(),
            MinionVariant::BabyKrush => data::baby_krush::data(),
            MinionVariant::BaronRivendare => data::baron_rivendare::data(),
            MinionVariant::Bonemare => data::bonemare::data(),
            MinionVariant::BongoBopper => data::bongo_bopper::data(),
            MinionVariant::BrannBronzebeard => data::brann_bronzebeard::data(),
            MinionVariant::BristlebackKnight => data::bristleback_knight::data(),
            MinionVariant::CapNHoggarr => data::cap_n_hoggarr::data(),
            MinionVariant::ChampionOfThePrimus => data::champion_of_the_primus::data(),
            MinionVariant::Chronormu => data::chronormu::data(),
            MinionVariant::CorruptedMyrmidon => data::corrupted_myrmidon::data(),
            MinionVariant::CritterWrangler => data::critter_wrangler::data(),
            MinionVariant::CyborgDrake => data::cyborg_drake::data(),
            MinionVariant::DeadlySpore => data::deadly_spore::data(),
            MinionVariant::DiscoShuffler => data::disco_shuffler::data(),
            MinionVariant::DrBoombox => data::dr_boombox::data(),
            MinionVariant::DrakkariEnchanter => data::drakkari_enchanter::data(),
            MinionVariant::FriendOfAFriend => data::friend_of_a_friend::data(),
            MinionVariant::GeneralDrakkisath => data::general_drakkisath::data(),
            MinionVariant::Glowscale => data::glowscale::data(),
            MinionVariant::GustyTrumpeter => data::gusty_trumpeter::data(),
            MinionVariant::HolyMecherel => data::holy_mecherel::data(),
            MinionVariant::HungeringAbomination => data::hungering_abomination::data(),
            MinionVariant::HunterOfGatherers => data::hunter_of_gatherers::data(),
            MinionVariant::ImposingPercussionist => data::imposing_percussionist::data(),
            MinionVariant::InsatiableUrZul => data::insatiable_ur_zul::data(),
            MinionVariant::InterrogatorWhitemane => data::interrogator_whitemane::data(),
            MinionVariant::KangorSApprentice => data::kangor_s_apprentice::data(),
            MinionVariant::KingBagurgle => data::king_bagurgle::data(),
            MinionVariant::LeeroyTheReckless => data::leeroy_the_reckless::data(),
            MinionVariant::LightfangEnforcer => data::lightfang_enforcer::data(),
            MinionVariant::LilRag => data::lil_rag::data(),
            MinionVariant::Magmaloc => data::magmaloc::data(),
            MinionVariant::MamaBear => data::mama_bear::data(),
            MinionVariant::Mannoroth => data::mannoroth::data(),
            MinionVariant::MechanizedGiftHorse => data::mechanized_gift_horse::data(),
            MinionVariant::Murozond => data::murozond::data(),
            MinionVariant::MythraxTheUnraveler => data::mythrax_the_unraveler::data(),
            MinionVariant::Niuzao => data::niuzao::data(),
            MinionVariant::NomiKitchenNightmare => data::nomi_kitchen_nightmare::data(),
            MinionVariant::OperaticBelcher => data::operatic_belcher::data(),
            MinionVariant::PalescaleCrocolisk => data::palescale_crocolisk::data(),
            MinionVariant::RazorgoreTheUntamed => data::razorgore_the_untamed::data(),
            MinionVariant::RecordSmuggler => data::record_smuggler::data(),
            MinionVariant::SanctumRester => data::sanctum_rester::data(),
            MinionVariant::SiSefin => data::si_sefin::data(),
            MinionVariant::SinrunnerBlanchy => data::sinrunner_blanchy::data(),
            MinionVariant::StormscaleSiren => data::stormscale_siren::data(),
            MinionVariant::TavernTempest => data::tavern_tempest::data(),
            MinionVariant::Tichondrius => data::tichondrius::data(),
            MinionVariant::TitusRivendare => data::titus_rivendare::data(),
            MinionVariant::TonyTwoTusk => data::tony_two_tusk::data(),
            MinionVariant::TortollanBlueShell => data::tortollan_blue_shell::data(),
            MinionVariant::Toxfin => data::toxfin::data(),
            MinionVariant::TransmutedBramblewitch => data::transmuted_bramblewitch::data(),
            MinionVariant::UnderhandedDealer => data::underhanded_dealer::data(),
            MinionVariant::UpbeatDuo => data::upbeat_duo::data(),
            MinionVariant::UpbeatImpressionist => data::upbeat_impressionist::data(),
            MinionVariant::UtilityDrone => data::utility_drone::data(),
            MinionVariant::VanessaVanCleef => data::vanessa_van_cleef::data(),
            MinionVariant::Voidlord => data::voidlord::data(),
            MinionVariant::ArchdruidHamuul => data::archdruid_hamuul::data(),
            MinionVariant::Bristlebach => data::bristlebach::data(),
            MinionVariant::CaptainFlatTusk => data::captain_flat_tusk::data(),
            MinionVariant::Charlga => data::charlga::data(),
            MinionVariant::ChoralMrrrglr => data::choral_mrrrglr::data(),
            MinionVariant::ColossusOfTheSun => data::colossus_of_the_sun::data(),
            MinionVariant::DarkgazeElder => data::darkgaze_elder::data(),
            MinionVariant::DreadAdmiralEliza => data::dread_admiral_eliza::data(),
            MinionVariant::ElementalOfSurprise => data::elemental_of_surprise::data(),
            MinionVariant::EternalSummoner => data::eternal_summoner::data(),
            MinionVariant::FamishedFelbat => data::famished_felbat::data(),
            MinionVariant::Felstomper => data::felstomper::data(),
            MinionVariant::FleetAdmiralTethys => data::fleet_admiral_tethys::data(),
            MinionVariant::FoeReaper4000 => data::foe_reaper_4000::data(),
            MinionVariant::GentleDjinni => data::gentle_djinni::data(),
            MinionVariant::Ghastcoiler => data::ghastcoiler::data(),
            MinionVariant::GoldrinnTheGreatWolf => data::goldrinn_the_great_wolf::data(),
            MinionVariant::GreaseBot => data::grease_bot::data(),
            MinionVariant::GretaGoldGun => data::greta_gold_gun::data(),
            MinionVariant::ImpMama => data::imp_mama::data(),
            MinionVariant::KalecgosArcaneAspect => data::kalecgos_arcane_aspect::data(),
            MinionVariant::LieutenantGarr => data::lieutenant_garr::data(),
            MinionVariant::Maexxna => data::maexxna::data(),
            MinionVariant::MantidQueen => data::mantid_queen::data(),
            MinionVariant::MechaJaraxxus => data::mecha_jaraxxus::data(),
            MinionVariant::Murky => data::murky::data(),
            MinionVariant::NadinaTheRed => data::nadina_the_red::data(),
            MinionVariant::NosyLooter => data::nosy_looter::data(),
            MinionVariant::OctosariWrapGod => data::octosari_wrap_god::data(),
            MinionVariant::OmegaBuster => data::omega_buster::data(),
            MinionVariant::OrgozoaTheTender => data::orgozoa_the_tender::data(),
            MinionVariant::PolarizingBeatboxer => data::polarizing_beatboxer::data(),
            MinionVariant::RelentlessMurGhoul => data::relentless_mur_ghoul::data(),
            MinionVariant::RockRock => data::rock_rock::data(),
            MinionVariant::SeafoodSlinger => data::seafood_slinger::data(),
            MinionVariant::SisterDeathwhisper => data::sister_deathwhisper::data(),
            MinionVariant::TeaMasterTheotar => data::tea_master_theotar::data(),
            MinionVariant::TheBoogieMonster => data::the_boogie_monster::data(),
            MinionVariant::TheWalkingFort => data::the_walking_fort::data(),
            MinionVariant::TidemistressAthissa => data::tidemistress_athissa::data(),
            MinionVariant::UtherTheLightbringer => data::uther_the_lightbringer::data(),
            MinionVariant::Warpwing => data::warpwing::data(),
            MinionVariant::YoungMurkEye => data::young_murk_eye::data(),
            MinionVariant::ZappSlywick => data::zapp_slywick::data(),
            MinionVariant::Amalgadon => data::amalgadon::data(),
            MinionVariant::ArgentBraggart => data::argent_braggart::data(),
            MinionVariant::CaptainSanders => data::captain_sanders::data(),
            MinionVariant::ChampionOfSargeras => data::champion_of_sargeras::data(),
            MinionVariant::GraniteGuardian => data::granite_guardian::data(),
            MinionVariant::KingVarian => data::king_varian::data(),
            MinionVariant::MoiraBronzebeard => data::moira_bronzebeard::data(),
            MinionVariant::ObsidianRavager => data::obsidian_ravager::data(),
            MinionVariant::PapaBear => data::papa_bear::data(),
            MinionVariant::RecurringNightmare => data::recurring_nightmare::data(),
            MinionVariant::SanguineChampion => data::sanguine_champion::data(),
            MinionVariant::SeaWitchZarJira => data::sea_witch_zar_jira::data(),
            MinionVariant::TheBoommobile => data::the_boommobile::data(),
            MinionVariant::TideOracleMorgl => data::tide_oracle_morgl::data(),
            MinionVariant::Amalgam => data::amalgam::data(),
            MinionVariant::AnnoyOSpawn => data::annoy_o_spawn::data(),
            MinionVariant::BackpiggyImp => data::backpiggy_imp::data(),
            MinionVariant::Baltharak => data::baltharak::data(),
            MinionVariant::Crab => data::crab::data(),
            MinionVariant::Cubling => data::cubling::data(),
            MinionVariant::DamagedGolem => data::damaged_golem::data(),
            MinionVariant::Devilsaur => data::devilsaur::data(),
            MinionVariant::DiabloLordOfTerror => data::diablo_lord_of_terror::data(),
            MinionVariant::EmperorCobra => data::emperor_cobra::data(),
            MinionVariant::FieryImp => data::fiery_imp::data(),
            MinionVariant::FishOfNZoth => data::fish_of_n_zoth::data(),
            MinionVariant::GoldenMonkey => data::golden_monkey::data(),
            MinionVariant::HalfShell => data::half_shell::data(),
            MinionVariant::HelpingHand => data::helping_hand::data(),
            MinionVariant::Hyena => data::hyena::data(),
            MinionVariant::Imp => data::imp::data(),
            MinionVariant::MagtheridonPrime => data::magtheridon_prime::data(),
            MinionVariant::Mechapony => data::mechapony::data(),
            MinionVariant::Mechorse => data::mechorse::data(),
            MinionVariant::Microbot => data::microbot::data(),
            MinionVariant::OnyxianWhelp => data::onyxian_whelp::data(),
            MinionVariant::OzumatSTentacle => data::ozumat_s_tentacle::data(),
            MinionVariant::Plant => data::plant::data(),
            MinionVariant::Rat => data::rat::data(),
            MinionVariant::Robosaur => data::robosaur::data(),
            MinionVariant::RustedReggie => data::rusted_reggie::data(),
            MinionVariant::Shudderling => data::shudderling::data(),
            MinionVariant::SkyPirate => data::sky_pirate::data(),
            MinionVariant::Smolderwing => data::smolderwing::data(),
            MinionVariant::Snake => data::snake::data(),
            MinionVariant::StoneElemental => data::stone_elemental::data(),
            MinionVariant::Tabbycat => data::tabbycat::data(),
            MinionVariant::TentacleOfOctosari => data::tentacle_of_octosari::data(),
            MinionVariant::WaterDroplet => data::water_droplet::data(),
        }
    }
    pub fn event_handlers(self) -> EventHandlers {
        match self {
            MinionVariant::Invalid => panic!("Invalid MinionVariant"),
            MinionVariant::Alleycat => alleycat::event_handlers(),
            MinionVariant::AnnoyOTron => annoy_o_tron::event_handlers(),
            MinionVariant::Bubblette => bubblette::event_handlers(),
            MinionVariant::DeckSwabbie => deck_swabbie::event_handlers(),
            MinionVariant::DozyWhelp => dozy_whelp::event_handlers(),
            MinionVariant::EvolvingChromawing => evolving_chromawing::event_handlers(),
            MinionVariant::IckyImp => icky_imp::event_handlers(),
            MinionVariant::Imprisoner => imprisoner::event_handlers(),
            MinionVariant::IncorporealCorporal => incorporeal_corporal::event_handlers(),
            MinionVariant::Manasaber => manasaber::event_handlers(),
            MinionVariant::MicroMummy => micro_mummy::event_handlers(),
            MinionVariant::MiniMyrmidon => mini_myrmidon::event_handlers(),
            MinionVariant::Mistake => mistake::event_handlers(),
            MinionVariant::PickyEater => picky_eater::event_handlers(),
            MinionVariant::Pupbot => pupbot::event_handlers(),
            MinionVariant::RazorfenGeomancer => razorfen_geomancer::event_handlers(),
            MinionVariant::RedWhelp => red_whelp::event_handlers(),
            MinionVariant::RefreshingAnomaly => refreshing_anomaly::event_handlers(),
            MinionVariant::RisenRider => risen_rider::event_handlers(),
            MinionVariant::RockpoolHunter => rockpool_hunter::event_handlers(),
            MinionVariant::RotHideGnoll => rot_hide_gnoll::event_handlers(),
            MinionVariant::Scallywag => scallywag::event_handlers(),
            MinionVariant::ScavengingHyena => scavenging_hyena::event_handlers(),
            MinionVariant::Sellemental => sellemental::event_handlers(),
            MinionVariant::ShellCollector => shell_collector::event_handlers(),
            MinionVariant::SilverbackPatriarch => silverback_patriarch::event_handlers(),
            MinionVariant::SouthseaBusker => southsea_busker::event_handlers(),
            MinionVariant::SunBaconRelaxer => sun_bacon_relaxer::event_handlers(),
            MinionVariant::SurfNSurf => surf_n_surf::event_handlers(),
            MinionVariant::Swampstriker => swampstriker::event_handlers(),
            MinionVariant::TavernTipper => tavern_tipper::event_handlers(),
            MinionVariant::Thorncaptain => thorncaptain::event_handlers(),
            MinionVariant::UpbeatFrontdrake => upbeat_frontdrake::event_handlers(),
            MinionVariant::WrathWeaver => wrath_weaver::event_handlers(),
            MinionVariant::AcolyteOfCThun => acolyte_of_c_thun::event_handlers(),
            MinionVariant::BackstageSecurity => backstage_security::event_handlers(),
            MinionVariant::BejeweledDuelist => bejeweled_duelist::event_handlers(),
            MinionVariant::BlazingSkyfin => blazing_skyfin::event_handlers(),
            MinionVariant::BriarbackBookie => briarback_bookie::event_handlers(),
            MinionVariant::CogworkCopter => cogwork_copter::event_handlers(),
            MinionVariant::CorpseRefiner => corpse_refiner::event_handlers(),
            MinionVariant::DeepSeaAngler => deep_sea_angler::event_handlers(),
            MinionVariant::EternalKnight => eternal_knight::event_handlers(),
            MinionVariant::FlourishingFrostling => flourishing_frostling::event_handlers(),
            MinionVariant::FreedealingGambler => freedealing_gambler::event_handlers(),
            MinionVariant::GlyphGuardian => glyph_guardian::event_handlers(),
            MinionVariant::HarvestGolem => harvest_golem::event_handlers(),
            MinionVariant::HummingBird => humming_bird::event_handlers(),
            MinionVariant::HungrySnapjaw => hungry_snapjaw::event_handlers(),
            MinionVariant::ImpulsiveTrickster => impulsive_trickster::event_handlers(),
            MinionVariant::InventOMatic => invent_o_matic::event_handlers(),
            MinionVariant::KaboomBot => kaboom_bot::event_handlers(),
            MinionVariant::KookyChemist => kooky_chemist::event_handlers(),
            MinionVariant::LavaLurker => lava_lurker::event_handlers(),
            MinionVariant::Leapfrogger => leapfrogger::event_handlers(),
            MinionVariant::LowFlier => low_flier::event_handlers(),
            MinionVariant::Lullabot => lullabot::event_handlers(),
            MinionVariant::MenagerieMug => menagerie_mug::event_handlers(),
            MinionVariant::MetaltoothLeaper => metaltooth_leaper::event_handlers(),
            MinionVariant::MindMuck => mind_muck::event_handlers(),
            MinionVariant::MoltenRock => molten_rock::event_handlers(),
            MinionVariant::Murcules => murcules::event_handlers(),
            MinionVariant::MurlocWarleader => murloc_warleader::event_handlers(),
            MinionVariant::NathrezimOverseer => nathrezim_overseer::event_handlers(),
            MinionVariant::NerubianDeathswarmer => nerubian_deathswarmer::event_handlers(),
            MinionVariant::OozelingGladiator => oozeling_gladiator::event_handlers(),
            MinionVariant::PatientScout => patient_scout::event_handlers(),
            MinionVariant::PiggybackImp => piggyback_imp::event_handlers(),
            MinionVariant::PoeticPenPal => poetic_pen_pal::event_handlers(),
            MinionVariant::ProphetOfTheBoar => prophet_of_the_boar::event_handlers(),
            MinionVariant::RabidSaurolisk => rabid_saurolisk::event_handlers(),
            MinionVariant::ReefRiffer => reef_riffer::event_handlers(),
            MinionVariant::RipsnarlCaptain => ripsnarl_captain::event_handlers(),
            MinionVariant::Roadboar => roadboar::event_handlers(),
            MinionVariant::SaltscaleHoncho => saltscale_honcho::event_handlers(),
            MinionVariant::ScarletSkull => scarlet_skull::event_handlers(),
            MinionVariant::SeabornSummoner => seaborn_summoner::event_handlers(),
            MinionVariant::SelflessHero => selfless_hero::event_handlers(),
            MinionVariant::SewerRat => sewer_rat::event_handlers(),
            MinionVariant::SnailCavalry => snail_cavalry::event_handlers(),
            MinionVariant::SoulRewinder => soul_rewinder::event_handlers(),
            MinionVariant::SouthseaCaptain => southsea_captain::event_handlers(),
            MinionVariant::SparringPartner => sparring_partner::event_handlers(),
            MinionVariant::SpawnOfNZoth => spawn_of_n_zoth::event_handlers(),
            MinionVariant::StewardOfTime => steward_of_time::event_handlers(),
            MinionVariant::Tad => tad::event_handlers(),
            MinionVariant::Thorncaller => thorncaller::event_handlers(),
            MinionVariant::ToughTusk => tough_tusk::event_handlers(),
            MinionVariant::TwilightEmissary => twilight_emissary::event_handlers(),
            MinionVariant::UnstableGhoul => unstable_ghoul::event_handlers(),
            MinionVariant::UpbeatFlutist => upbeat_flutist::event_handlers(),
            MinionVariant::WhelpSmuggler => whelp_smuggler::event_handlers(),
            MinionVariant::YoHoOgre => yo_ho_ogre::event_handlers(),
            MinionVariant::Yrel => yrel::event_handlers(),
            MinionVariant::AccordOTron => accord_o_tron::event_handlers(),
            MinionVariant::AmberGuardian => amber_guardian::event_handlers(),
            MinionVariant::ArmOfTheEmpire => arm_of_the_empire::event_handlers(),
            MinionVariant::BirdBuddy => bird_buddy::event_handlers(),
            MinionVariant::BloodsailCannoneer => bloodsail_cannoneer::event_handlers(),
            MinionVariant::BrinyBootlegger => briny_bootlegger::event_handlers(),
            MinionVariant::BristlebackBrute => bristleback_brute::event_handlers(),
            MinionVariant::BristlemaneScrapsmith => bristlemane_scrapsmith::event_handlers(),
            MinionVariant::BronzeWarden => bronze_warden::event_handlers(),
            MinionVariant::BuddingGreenthumb => budding_greenthumb::event_handlers(),
            MinionVariant::ColdlightSeer => coldlight_seer::event_handlers(),
            MinionVariant::CracklingCyclone => crackling_cyclone::event_handlers(),
            MinionVariant::DaggerspineThrasher => daggerspine_thrasher::event_handlers(),
            MinionVariant::DeflectOBot => deflect_o_bot::event_handlers(),
            MinionVariant::Dreadbeard => dreadbeard::event_handlers(),
            MinionVariant::EventideBrute => eventide_brute::event_handlers(),
            MinionVariant::FacelessDisciple => faceless_disciple::event_handlers(),
            MinionVariant::Felemental => felemental::event_handlers(),
            MinionVariant::FelfinNavigator => felfin_navigator::event_handlers(),
            MinionVariant::FirstMatePip => first_mate_pip::event_handlers(),
            MinionVariant::FreeFlyingFeathermane => free_flying_feathermane::event_handlers(),
            MinionVariant::Gemsplitter => gemsplitter::event_handlers(),
            MinionVariant::GhoulOfTheFeast => ghoul_of_the_feast::event_handlers(),
            MinionVariant::GunpowderCourier => gunpowder_courier::event_handlers(),
            MinionVariant::HandlessForsaken => handless_forsaken::event_handlers(),
            MinionVariant::Houndmaster => houndmaster::event_handlers(),
            MinionVariant::IronGroundskeeper => iron_groundskeeper::event_handlers(),
            MinionVariant::IronSensei => iron_sensei::event_handlers(),
            MinionVariant::JellyBelly => jelly_belly::event_handlers(),
            MinionVariant::KathraNatir => kathra_natir::event_handlers(),
            MinionVariant::KeyboardIgniter => keyboard_igniter::event_handlers(),
            MinionVariant::Khadgar => khadgar::event_handlers(),
            MinionVariant::LeechingFelhound => leeching_felhound::event_handlers(),
            MinionVariant::LegionOverseer => legion_overseer::event_handlers(),
            MinionVariant::LichDoctor => lich_doctor::event_handlers(),
            MinionVariant::LivingConstellation => living_constellation::event_handlers(),
            MinionVariant::MonstrousMacaw => monstrous_macaw::event_handlers(),
            MinionVariant::MoonBaconJazzer => moon_bacon_jazzer::event_handlers(),
            MinionVariant::NetherDrake => nether_drake::event_handlers(),
            MinionVariant::NightmareAmalgam => nightmare_amalgam::event_handlers(),
            MinionVariant::PartyElemental => party_elemental::event_handlers(),
            MinionVariant::PashmarTheVengeful => pashmar_the_vengeful::event_handlers(),
            MinionVariant::PricklyPiper => prickly_piper::event_handlers(),
            MinionVariant::Pufferquil => pufferquil::event_handlers(),
            MinionVariant::RadioStar => radio_star::event_handlers(),
            MinionVariant::RatPack => rat_pack::event_handlers(),
            MinionVariant::RecyclingWraith => recycling_wraith::event_handlers(),
            MinionVariant::RelentlessSentry => relentless_sentry::event_handlers(),
            MinionVariant::ReplicatingMenace => replicating_menace::event_handlers(),
            MinionVariant::SaltyLooter => salty_looter::event_handlers(),
            MinionVariant::Scourfin => scourfin::event_handlers(),
            MinionVariant::ScrewjankClunker => screwjank_clunker::event_handlers(),
            MinionVariant::ShifterZerus => shifter_zerus::event_handlers(),
            MinionVariant::ShoalCommander => shoal_commander::event_handlers(),
            MinionVariant::Smogger => smogger::event_handlers(),
            MinionVariant::SoreLoser => sore_loser::event_handlers(),
            MinionVariant::SoulDevourer => soul_devourer::event_handlers(),
            MinionVariant::SoulJuggler => soul_juggler::event_handlers(),
            MinionVariant::SouthseaStrongarm => southsea_strongarm::event_handlers(),
            MinionVariant::SparkLing => spark_ling::event_handlers(),
            MinionVariant::StasisElemental => stasis_elemental::event_handlers(),
            MinionVariant::Swolefin => swolefin::event_handlers(),
            MinionVariant::Tarecgosa => tarecgosa::event_handlers(),
            MinionVariant::TheGladIator => the_glad_iator::event_handlers(),
            MinionVariant::TimeSaver => time_saver::event_handlers(),
            MinionVariant::WardenOfOld => warden_of_old::event_handlers(),
            MinionVariant::WitheredSpearhide => withered_spearhide::event_handlers(),
            MinionVariant::ZestyShaker => zesty_shaker::event_handlers(),
            MinionVariant::AnnihilanBattlemaster => annihilan_battlemaster::event_handlers(),
            MinionVariant::AnnoyOModule => annoy_o_module::event_handlers(),
            MinionVariant::AnubArakNerubianKing => anub_arak_nerubian_king::event_handlers(),
            MinionVariant::Atramedes => atramedes::event_handlers(),
            MinionVariant::BallOfMinions => ball_of_minions::event_handlers(),
            MinionVariant::BananaSlamma => banana_slamma::event_handlers(),
            MinionVariant::Bannerboar => bannerboar::event_handlers(),
            MinionVariant::Bassgill => bassgill::event_handlers(),
            MinionVariant::Bigfernal => bigfernal::event_handlers(),
            MinionVariant::BladeCollector => blade_collector::event_handlers(),
            MinionVariant::Bonker => bonker::event_handlers(),
            MinionVariant::BreamCounter => bream_counter::event_handlers(),
            MinionVariant::CarbonicCopy => carbonic_copy::event_handlers(),
            MinionVariant::CaveHydra => cave_hydra::event_handlers(),
            MinionVariant::ChampionOfYShaarj => champion_of_y_shaarj::event_handlers(),
            MinionVariant::CobaltScalebane => cobalt_scalebane::event_handlers(),
            MinionVariant::DancingBarnstormer => dancing_barnstormer::event_handlers(),
            MinionVariant::DazzlingLightspawn => dazzling_lightspawn::event_handlers(),
            MinionVariant::DeepBlueCrooner => deep_blue_crooner::event_handlers(),
            MinionVariant::DefenderOfArgus => defender_of_argus::event_handlers(),
            MinionVariant::DrakonidEnforcer => drakonid_enforcer::event_handlers(),
            MinionVariant::DynamicDuo => dynamic_duo::event_handlers(),
            MinionVariant::EelboundArcher => eelbound_archer::event_handlers(),
            MinionVariant::ElectricSynthesizer => electric_synthesizer::event_handlers(),
            MinionVariant::EmergentFlame => emergent_flame::event_handlers(),
            MinionVariant::FairyTaleCaroler => fairy_tale_caroler::event_handlers(),
            MinionVariant::FireworksFanatic => fireworks_fanatic::event_handlers(),
            MinionVariant::FloatingWatcher => floating_watcher::event_handlers(),
            MinionVariant::GemSmuggler => gem_smuggler::event_handlers(),
            MinionVariant::Goldgrubber => goldgrubber::event_handlers(),
            MinionVariant::Groundshaker => groundshaker::event_handlers(),
            MinionVariant::ImpatientDoomsayer => impatient_doomsayer::event_handlers(),
            MinionVariant::LovesickBalladist => lovesick_balladist::event_handlers(),
            MinionVariant::MajordomoExecutus => majordomo_executus::event_handlers(),
            MinionVariant::MalchezaarPrinceOfDance => malchezaar_prince_of_dance::event_handlers(),
            MinionVariant::MasterOfRealities => master_of_realities::event_handlers(),
            MinionVariant::MechanoEgg => mechano_egg::event_handlers(),
            MinionVariant::MechanoTank => mechano_tank::event_handlers(),
            MinionVariant::MenagerieJug => menagerie_jug::event_handlers(),
            MinionVariant::Necrolyte => necrolyte::event_handlers(),
            MinionVariant::PeckishFeldrake => peckish_feldrake::event_handlers(),
            MinionVariant::PeggyBrittlebone => peggy_brittlebone::event_handlers(),
            MinionVariant::PeggySturdybone => peggy_sturdybone::event_handlers(),
            MinionVariant::PlaguedTidewalker => plagued_tidewalker::event_handlers(),
            MinionVariant::PossessiveBanshee => possessive_banshee::event_handlers(),
            MinionVariant::PrestorSPyrospawn => prestor_s_pyrospawn::event_handlers(),
            MinionVariant::PrimalfinLookout => primalfin_lookout::event_handlers(),
            MinionVariant::PrizedPromoDrake => prized_promo_drake::event_handlers(),
            MinionVariant::QirajiHarbinger => qiraji_harbinger::event_handlers(),
            MinionVariant::ReanimatingRattler => reanimating_rattler::event_handlers(),
            MinionVariant::ReefExplorer => reef_explorer::event_handlers(),
            MinionVariant::RendleTheMistermind => rendle_the_mistermind::event_handlers(),
            MinionVariant::RingMatron => ring_matron::event_handlers(),
            MinionVariant::RylakMetalhead => rylak_metalhead::event_handlers(),
            MinionVariant::SavannahHighmane => savannah_highmane::event_handlers(),
            MinionVariant::ScrapScraper => scrap_scraper::event_handlers(),
            MinionVariant::SilentSwimmer => silent_swimmer::event_handlers(),
            MinionVariant::SinDoreiStraightShot => sin_dorei_straight_shot::event_handlers(),
            MinionVariant::SlyRaptor => sly_raptor::event_handlers(),
            MinionVariant::Soulsplitter => soulsplitter::event_handlers(),
            MinionVariant::Stormbringer => stormbringer::event_handlers(),
            MinionVariant::StrongshellScavenger => strongshell_scavenger::event_handlers(),
            MinionVariant::TreasureSeekerElise => treasure_seeker_elise::event_handlers(),
            MinionVariant::TunnelBlaster => tunnel_blaster::event_handlers(),
            MinionVariant::UpbeatUpstart => upbeat_upstart::event_handlers(),
            MinionVariant::VigilantStoneborn => vigilant_stoneborn::event_handlers(),
            MinionVariant::Wargear => wargear::event_handlers(),
            MinionVariant::Waverider => waverider::event_handlers(),
            MinionVariant::WildfireElemental => wildfire_elemental::event_handlers(),
            MinionVariant::WitchwingNestmatron => witchwing_nestmatron::event_handlers(),
            MinionVariant::XyloBones => xylo_bones::event_handlers(),
            MinionVariant::AdaptableBarricade => adaptable_barricade::event_handlers(),
            MinionVariant::AgamagganTheGreatBoar => agamaggan_the_great_boar::event_handlers(),
            MinionVariant::AggemThorncurse => aggem_thorncurse::event_handlers(),
            MinionVariant::AnnoyOTroupe => annoy_o_troupe::event_handlers(),
            MinionVariant::BabyKrush => baby_krush::event_handlers(),
            MinionVariant::BaronRivendare => baron_rivendare::event_handlers(),
            MinionVariant::Bonemare => bonemare::event_handlers(),
            MinionVariant::BongoBopper => bongo_bopper::event_handlers(),
            MinionVariant::BrannBronzebeard => brann_bronzebeard::event_handlers(),
            MinionVariant::BristlebackKnight => bristleback_knight::event_handlers(),
            MinionVariant::CapNHoggarr => cap_n_hoggarr::event_handlers(),
            MinionVariant::ChampionOfThePrimus => champion_of_the_primus::event_handlers(),
            MinionVariant::Chronormu => chronormu::event_handlers(),
            MinionVariant::CorruptedMyrmidon => corrupted_myrmidon::event_handlers(),
            MinionVariant::CritterWrangler => critter_wrangler::event_handlers(),
            MinionVariant::CyborgDrake => cyborg_drake::event_handlers(),
            MinionVariant::DeadlySpore => deadly_spore::event_handlers(),
            MinionVariant::DiscoShuffler => disco_shuffler::event_handlers(),
            MinionVariant::DrBoombox => dr_boombox::event_handlers(),
            MinionVariant::DrakkariEnchanter => drakkari_enchanter::event_handlers(),
            MinionVariant::FriendOfAFriend => friend_of_a_friend::event_handlers(),
            MinionVariant::GeneralDrakkisath => general_drakkisath::event_handlers(),
            MinionVariant::Glowscale => glowscale::event_handlers(),
            MinionVariant::GustyTrumpeter => gusty_trumpeter::event_handlers(),
            MinionVariant::HolyMecherel => holy_mecherel::event_handlers(),
            MinionVariant::HungeringAbomination => hungering_abomination::event_handlers(),
            MinionVariant::HunterOfGatherers => hunter_of_gatherers::event_handlers(),
            MinionVariant::ImposingPercussionist => imposing_percussionist::event_handlers(),
            MinionVariant::InsatiableUrZul => insatiable_ur_zul::event_handlers(),
            MinionVariant::InterrogatorWhitemane => interrogator_whitemane::event_handlers(),
            MinionVariant::KangorSApprentice => kangor_s_apprentice::event_handlers(),
            MinionVariant::KingBagurgle => king_bagurgle::event_handlers(),
            MinionVariant::LeeroyTheReckless => leeroy_the_reckless::event_handlers(),
            MinionVariant::LightfangEnforcer => lightfang_enforcer::event_handlers(),
            MinionVariant::LilRag => lil_rag::event_handlers(),
            MinionVariant::Magmaloc => magmaloc::event_handlers(),
            MinionVariant::MamaBear => mama_bear::event_handlers(),
            MinionVariant::Mannoroth => mannoroth::event_handlers(),
            MinionVariant::MechanizedGiftHorse => mechanized_gift_horse::event_handlers(),
            MinionVariant::Murozond => murozond::event_handlers(),
            MinionVariant::MythraxTheUnraveler => mythrax_the_unraveler::event_handlers(),
            MinionVariant::Niuzao => niuzao::event_handlers(),
            MinionVariant::NomiKitchenNightmare => nomi_kitchen_nightmare::event_handlers(),
            MinionVariant::OperaticBelcher => operatic_belcher::event_handlers(),
            MinionVariant::PalescaleCrocolisk => palescale_crocolisk::event_handlers(),
            MinionVariant::RazorgoreTheUntamed => razorgore_the_untamed::event_handlers(),
            MinionVariant::RecordSmuggler => record_smuggler::event_handlers(),
            MinionVariant::SanctumRester => sanctum_rester::event_handlers(),
            MinionVariant::SiSefin => si_sefin::event_handlers(),
            MinionVariant::SinrunnerBlanchy => sinrunner_blanchy::event_handlers(),
            MinionVariant::StormscaleSiren => stormscale_siren::event_handlers(),
            MinionVariant::TavernTempest => tavern_tempest::event_handlers(),
            MinionVariant::Tichondrius => tichondrius::event_handlers(),
            MinionVariant::TitusRivendare => titus_rivendare::event_handlers(),
            MinionVariant::TonyTwoTusk => tony_two_tusk::event_handlers(),
            MinionVariant::TortollanBlueShell => tortollan_blue_shell::event_handlers(),
            MinionVariant::Toxfin => toxfin::event_handlers(),
            MinionVariant::TransmutedBramblewitch => transmuted_bramblewitch::event_handlers(),
            MinionVariant::UnderhandedDealer => underhanded_dealer::event_handlers(),
            MinionVariant::UpbeatDuo => upbeat_duo::event_handlers(),
            MinionVariant::UpbeatImpressionist => upbeat_impressionist::event_handlers(),
            MinionVariant::UtilityDrone => utility_drone::event_handlers(),
            MinionVariant::VanessaVanCleef => vanessa_van_cleef::event_handlers(),
            MinionVariant::Voidlord => voidlord::event_handlers(),
            MinionVariant::ArchdruidHamuul => archdruid_hamuul::event_handlers(),
            MinionVariant::Bristlebach => bristlebach::event_handlers(),
            MinionVariant::CaptainFlatTusk => captain_flat_tusk::event_handlers(),
            MinionVariant::Charlga => charlga::event_handlers(),
            MinionVariant::ChoralMrrrglr => choral_mrrrglr::event_handlers(),
            MinionVariant::ColossusOfTheSun => colossus_of_the_sun::event_handlers(),
            MinionVariant::DarkgazeElder => darkgaze_elder::event_handlers(),
            MinionVariant::DreadAdmiralEliza => dread_admiral_eliza::event_handlers(),
            MinionVariant::ElementalOfSurprise => elemental_of_surprise::event_handlers(),
            MinionVariant::EternalSummoner => eternal_summoner::event_handlers(),
            MinionVariant::FamishedFelbat => famished_felbat::event_handlers(),
            MinionVariant::Felstomper => felstomper::event_handlers(),
            MinionVariant::FleetAdmiralTethys => fleet_admiral_tethys::event_handlers(),
            MinionVariant::FoeReaper4000 => foe_reaper_4000::event_handlers(),
            MinionVariant::GentleDjinni => gentle_djinni::event_handlers(),
            MinionVariant::Ghastcoiler => ghastcoiler::event_handlers(),
            MinionVariant::GoldrinnTheGreatWolf => goldrinn_the_great_wolf::event_handlers(),
            MinionVariant::GreaseBot => grease_bot::event_handlers(),
            MinionVariant::GretaGoldGun => greta_gold_gun::event_handlers(),
            MinionVariant::ImpMama => imp_mama::event_handlers(),
            MinionVariant::KalecgosArcaneAspect => kalecgos_arcane_aspect::event_handlers(),
            MinionVariant::LieutenantGarr => lieutenant_garr::event_handlers(),
            MinionVariant::Maexxna => maexxna::event_handlers(),
            MinionVariant::MantidQueen => mantid_queen::event_handlers(),
            MinionVariant::MechaJaraxxus => mecha_jaraxxus::event_handlers(),
            MinionVariant::Murky => murky::event_handlers(),
            MinionVariant::NadinaTheRed => nadina_the_red::event_handlers(),
            MinionVariant::NosyLooter => nosy_looter::event_handlers(),
            MinionVariant::OctosariWrapGod => octosari_wrap_god::event_handlers(),
            MinionVariant::OmegaBuster => omega_buster::event_handlers(),
            MinionVariant::OrgozoaTheTender => orgozoa_the_tender::event_handlers(),
            MinionVariant::PolarizingBeatboxer => polarizing_beatboxer::event_handlers(),
            MinionVariant::RelentlessMurGhoul => relentless_mur_ghoul::event_handlers(),
            MinionVariant::RockRock => rock_rock::event_handlers(),
            MinionVariant::SeafoodSlinger => seafood_slinger::event_handlers(),
            MinionVariant::SisterDeathwhisper => sister_deathwhisper::event_handlers(),
            MinionVariant::TeaMasterTheotar => tea_master_theotar::event_handlers(),
            MinionVariant::TheBoogieMonster => the_boogie_monster::event_handlers(),
            MinionVariant::TheWalkingFort => the_walking_fort::event_handlers(),
            MinionVariant::TidemistressAthissa => tidemistress_athissa::event_handlers(),
            MinionVariant::UtherTheLightbringer => uther_the_lightbringer::event_handlers(),
            MinionVariant::Warpwing => warpwing::event_handlers(),
            MinionVariant::YoungMurkEye => young_murk_eye::event_handlers(),
            MinionVariant::ZappSlywick => zapp_slywick::event_handlers(),
            MinionVariant::Amalgadon => amalgadon::event_handlers(),
            MinionVariant::ArgentBraggart => argent_braggart::event_handlers(),
            MinionVariant::CaptainSanders => captain_sanders::event_handlers(),
            MinionVariant::ChampionOfSargeras => champion_of_sargeras::event_handlers(),
            MinionVariant::GraniteGuardian => granite_guardian::event_handlers(),
            MinionVariant::KingVarian => king_varian::event_handlers(),
            MinionVariant::MoiraBronzebeard => moira_bronzebeard::event_handlers(),
            MinionVariant::ObsidianRavager => obsidian_ravager::event_handlers(),
            MinionVariant::PapaBear => papa_bear::event_handlers(),
            MinionVariant::RecurringNightmare => recurring_nightmare::event_handlers(),
            MinionVariant::SanguineChampion => sanguine_champion::event_handlers(),
            MinionVariant::SeaWitchZarJira => sea_witch_zar_jira::event_handlers(),
            MinionVariant::TheBoommobile => the_boommobile::event_handlers(),
            MinionVariant::TideOracleMorgl => tide_oracle_morgl::event_handlers(),
            MinionVariant::Amalgam => amalgam::event_handlers(),
            MinionVariant::AnnoyOSpawn => annoy_o_spawn::event_handlers(),
            MinionVariant::BackpiggyImp => backpiggy_imp::event_handlers(),
            MinionVariant::Baltharak => baltharak::event_handlers(),
            MinionVariant::Crab => crab::event_handlers(),
            MinionVariant::Cubling => cubling::event_handlers(),
            MinionVariant::DamagedGolem => damaged_golem::event_handlers(),
            MinionVariant::Devilsaur => devilsaur::event_handlers(),
            MinionVariant::DiabloLordOfTerror => diablo_lord_of_terror::event_handlers(),
            MinionVariant::EmperorCobra => emperor_cobra::event_handlers(),
            MinionVariant::FieryImp => fiery_imp::event_handlers(),
            MinionVariant::FishOfNZoth => fish_of_n_zoth::event_handlers(),
            MinionVariant::GoldenMonkey => golden_monkey::event_handlers(),
            MinionVariant::HalfShell => half_shell::event_handlers(),
            MinionVariant::HelpingHand => helping_hand::event_handlers(),
            MinionVariant::Hyena => hyena::event_handlers(),
            MinionVariant::Imp => imp::event_handlers(),
            MinionVariant::MagtheridonPrime => magtheridon_prime::event_handlers(),
            MinionVariant::Mechapony => mechapony::event_handlers(),
            MinionVariant::Mechorse => mechorse::event_handlers(),
            MinionVariant::Microbot => microbot::event_handlers(),
            MinionVariant::OnyxianWhelp => onyxian_whelp::event_handlers(),
            MinionVariant::OzumatSTentacle => ozumat_s_tentacle::event_handlers(),
            MinionVariant::Plant => plant::event_handlers(),
            MinionVariant::Rat => rat::event_handlers(),
            MinionVariant::Robosaur => robosaur::event_handlers(),
            MinionVariant::RustedReggie => rusted_reggie::event_handlers(),
            MinionVariant::Shudderling => shudderling::event_handlers(),
            MinionVariant::SkyPirate => sky_pirate::event_handlers(),
            MinionVariant::Smolderwing => smolderwing::event_handlers(),
            MinionVariant::Snake => snake::event_handlers(),
            MinionVariant::StoneElemental => stone_elemental::event_handlers(),
            MinionVariant::Tabbycat => tabbycat::event_handlers(),
            MinionVariant::TentacleOfOctosari => tentacle_of_octosari::event_handlers(),
            MinionVariant::WaterDroplet => water_droplet::event_handlers(),
        }
    }
    pub fn random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        *Self::ALL.choose(rng).unwrap()
    }
}
