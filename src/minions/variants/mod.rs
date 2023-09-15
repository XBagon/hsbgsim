use rand::seq::SliceRandom;
use super::{Abilities, MinionInstance};
use crate::events::EventHandler;
mod data;
pub mod alleycat;
pub mod annoy_o_tron;
pub mod bubblette;
pub mod deck_swabbie;
pub mod dozy_whelp;
pub mod evolving_chromawing;
pub mod icky_imp;
pub mod imprisoner;
pub mod incorporeal_corporal;
pub mod manasaber;
pub mod micro_mummy;
pub mod mini_myrmidon;
pub mod mistake;
pub mod picky_eater;
pub mod pupbot;
pub mod razorfen_geomancer;
pub mod red_whelp;
pub mod refreshing_anomaly;
pub mod risen_rider;
pub mod rockpool_hunter;
pub mod rot_hide_gnoll;
pub mod scallywag;
pub mod scavenging_hyena;
pub mod sellemental;
pub mod shell_collector;
pub mod silverback_patriarch;
pub mod southsea_busker;
pub mod sun_bacon_relaxer;
pub mod surf_n_surf;
pub mod swampstriker;
pub mod tavern_tipper;
pub mod thorncaptain;
pub mod upbeat_frontdrake;
pub mod wrath_weaver;
pub mod acolyte_of_c_thun;
pub mod backstage_security;
pub mod bejeweled_duelist;
pub mod blazing_skyfin;
pub mod briarback_bookie;
pub mod cogwork_copter;
pub mod corpse_refiner;
pub mod deep_sea_angler;
pub mod eternal_knight;
pub mod flourishing_frostling;
pub mod freedealing_gambler;
pub mod glyph_guardian;
pub mod harvest_golem;
pub mod humming_bird;
pub mod hungry_snapjaw;
pub mod impulsive_trickster;
pub mod invent_o_matic;
pub mod kaboom_bot;
pub mod kooky_chemist;
pub mod lava_lurker;
pub mod leapfrogger;
pub mod low_flier;
pub mod lullabot;
pub mod menagerie_mug;
pub mod metaltooth_leaper;
pub mod mind_muck;
pub mod molten_rock;
pub mod murcules;
pub mod murloc_warleader;
pub mod nathrezim_overseer;
pub mod nerubian_deathswarmer;
pub mod oozeling_gladiator;
pub mod patient_scout;
pub mod piggyback_imp;
pub mod poetic_pen_pal;
pub mod prophet_of_the_boar;
pub mod rabid_saurolisk;
pub mod reef_riffer;
pub mod ripsnarl_captain;
pub mod roadboar;
pub mod saltscale_honcho;
pub mod scarlet_skull;
pub mod seaborn_summoner;
pub mod selfless_hero;
pub mod sewer_rat;
pub mod snail_cavalry;
pub mod soul_rewinder;
pub mod southsea_captain;
pub mod sparring_partner;
pub mod spawn_of_n_zoth;
pub mod steward_of_time;
pub mod tad;
pub mod thorncaller;
pub mod tough_tusk;
pub mod twilight_emissary;
pub mod unstable_ghoul;
pub mod upbeat_flutist;
pub mod whelp_smuggler;
pub mod yo_ho_ogre;
pub mod yrel;
pub mod accord_o_tron;
pub mod amber_guardian;
pub mod arm_of_the_empire;
pub mod bird_buddy;
pub mod bloodsail_cannoneer;
pub mod briny_bootlegger;
pub mod bristleback_brute;
pub mod bristlemane_scrapsmith;
pub mod bronze_warden;
pub mod budding_greenthumb;
pub mod coldlight_seer;
pub mod crackling_cyclone;
pub mod daggerspine_thrasher;
pub mod deflect_o_bot;
pub mod dreadbeard;
pub mod eventide_brute;
pub mod faceless_disciple;
pub mod felemental;
pub mod felfin_navigator;
pub mod first_mate_pip;
pub mod free_flying_feathermane;
pub mod gemsplitter;
pub mod ghoul_of_the_feast;
pub mod gunpowder_courier;
pub mod handless_forsaken;
pub mod houndmaster;
pub mod iron_groundskeeper;
pub mod iron_sensei;
pub mod jelly_belly;
pub mod kathra_natir;
pub mod keyboard_igniter;
pub mod khadgar;
pub mod leeching_felhound;
pub mod legion_overseer;
pub mod lich_doctor;
pub mod living_constellation;
pub mod monstrous_macaw;
pub mod moon_bacon_jazzer;
pub mod nether_drake;
pub mod nightmare_amalgam;
pub mod party_elemental;
pub mod pashmar_the_vengeful;
pub mod prickly_piper;
pub mod pufferquil;
pub mod radio_star;
pub mod rat_pack;
pub mod recycling_wraith;
pub mod relentless_sentry;
pub mod replicating_menace;
pub mod salty_looter;
pub mod scourfin;
pub mod screwjank_clunker;
pub mod shifter_zerus;
pub mod shoal_commander;
pub mod smogger;
pub mod sore_loser;
pub mod soul_devourer;
pub mod soul_juggler;
pub mod southsea_strongarm;
pub mod spark_ling;
pub mod stasis_elemental;
pub mod swolefin;
pub mod tarecgosa;
pub mod the_glad_iator;
pub mod time_saver;
pub mod warden_of_old;
pub mod withered_spearhide;
pub mod zesty_shaker;
pub mod annihilan_battlemaster;
pub mod annoy_o_module;
pub mod anub_arak_nerubian_king;
pub mod atramedes;
pub mod ball_of_minions;
pub mod banana_slamma;
pub mod bannerboar;
pub mod bassgill;
pub mod bigfernal;
pub mod blade_collector;
pub mod bonker;
pub mod bream_counter;
pub mod carbonic_copy;
pub mod cave_hydra;
pub mod champion_of_y_shaarj;
pub mod cobalt_scalebane;
pub mod dancing_barnstormer;
pub mod dazzling_lightspawn;
pub mod deep_blue_crooner;
pub mod defender_of_argus;
pub mod drakonid_enforcer;
pub mod dynamic_duo;
pub mod eelbound_archer;
pub mod electric_synthesizer;
pub mod emergent_flame;
pub mod fairy_tale_caroler;
pub mod fireworks_fanatic;
pub mod floating_watcher;
pub mod gem_smuggler;
pub mod goldgrubber;
pub mod groundshaker;
pub mod impatient_doomsayer;
pub mod lovesick_balladist;
pub mod majordomo_executus;
pub mod malchezaar_prince_of_dance;
pub mod master_of_realities;
pub mod mechano_egg;
pub mod mechano_tank;
pub mod menagerie_jug;
pub mod necrolyte;
pub mod peckish_feldrake;
pub mod peggy_brittlebone;
pub mod peggy_sturdybone;
pub mod plagued_tidewalker;
pub mod possessive_banshee;
pub mod prestor_s_pyrospawn;
pub mod primalfin_lookout;
pub mod prized_promo_drake;
pub mod qiraji_harbinger;
pub mod reanimating_rattler;
pub mod reef_explorer;
pub mod rendle_the_mistermind;
pub mod ring_matron;
pub mod rylak_metalhead;
pub mod savannah_highmane;
pub mod scrap_scraper;
pub mod silent_swimmer;
pub mod sin_dorei_straight_shot;
pub mod sly_raptor;
pub mod soulsplitter;
pub mod stormbringer;
pub mod strongshell_scavenger;
pub mod treasure_seeker_elise;
pub mod tunnel_blaster;
pub mod upbeat_upstart;
pub mod vigilant_stoneborn;
pub mod wargear;
pub mod waverider;
pub mod wildfire_elemental;
pub mod witchwing_nestmatron;
pub mod xylo_bones;
pub mod adaptable_barricade;
pub mod agamaggan_the_great_boar;
pub mod aggem_thorncurse;
pub mod annoy_o_troupe;
pub mod baby_krush;
pub mod baron_rivendare;
pub mod bonemare;
pub mod bongo_bopper;
pub mod brann_bronzebeard;
pub mod bristleback_knight;
pub mod cap_n_hoggarr;
pub mod champion_of_the_primus;
pub mod chronormu;
pub mod corrupted_myrmidon;
pub mod critter_wrangler;
pub mod cyborg_drake;
pub mod deadly_spore;
pub mod disco_shuffler;
pub mod dr_boombox;
pub mod drakkari_enchanter;
pub mod friend_of_a_friend;
pub mod general_drakkisath;
pub mod glowscale;
pub mod gusty_trumpeter;
pub mod holy_mecherel;
pub mod hungering_abomination;
pub mod hunter_of_gatherers;
pub mod imposing_percussionist;
pub mod insatiable_ur_zul;
pub mod interrogator_whitemane;
pub mod kangor_s_apprentice;
pub mod king_bagurgle;
pub mod leeroy_the_reckless;
pub mod lightfang_enforcer;
pub mod lil_rag;
pub mod magmaloc;
pub mod mama_bear;
pub mod mannoroth;
pub mod mechanized_gift_horse;
pub mod murozond;
pub mod mythrax_the_unraveler;
pub mod niuzao;
pub mod nomi_kitchen_nightmare;
pub mod operatic_belcher;
pub mod palescale_crocolisk;
pub mod razorgore_the_untamed;
pub mod record_smuggler;
pub mod sanctum_rester;
pub mod si_sefin;
pub mod sinrunner_blanchy;
pub mod stormscale_siren;
pub mod tavern_tempest;
pub mod tichondrius;
pub mod titus_rivendare;
pub mod tony_two_tusk;
pub mod tortollan_blue_shell;
pub mod toxfin;
pub mod transmuted_bramblewitch;
pub mod underhanded_dealer;
pub mod upbeat_duo;
pub mod upbeat_impressionist;
pub mod utility_drone;
pub mod vanessa_van_cleef;
pub mod voidlord;
pub mod archdruid_hamuul;
pub mod bristlebach;
pub mod captain_flat_tusk;
pub mod charlga;
pub mod choral_mrrrglr;
pub mod colossus_of_the_sun;
pub mod darkgaze_elder;
pub mod dread_admiral_eliza;
pub mod elemental_of_surprise;
pub mod eternal_summoner;
pub mod famished_felbat;
pub mod felstomper;
pub mod fleet_admiral_tethys;
pub mod foe_reaper_4000;
pub mod gentle_djinni;
pub mod ghastcoiler;
pub mod goldrinn_the_great_wolf;
pub mod grease_bot;
pub mod greta_gold_gun;
pub mod imp_mama;
pub mod kalecgos_arcane_aspect;
pub mod lieutenant_garr;
pub mod maexxna;
pub mod mantid_queen;
pub mod mecha_jaraxxus;
pub mod murky;
pub mod nadina_the_red;
pub mod nosy_looter;
pub mod octosari_wrap_god;
pub mod omega_buster;
pub mod orgozoa_the_tender;
pub mod polarizing_beatboxer;
pub mod relentless_mur_ghoul;
pub mod rock_rock;
pub mod seafood_slinger;
pub mod sister_deathwhisper;
pub mod tea_master_theotar;
pub mod the_boogie_monster;
pub mod the_walking_fort;
pub mod tidemistress_athissa;
pub mod uther_the_lightbringer;
pub mod warpwing;
pub mod young_murk_eye;
pub mod zapp_slywick;
pub mod amalgadon;
pub mod argent_braggart;
pub mod captain_sanders;
pub mod champion_of_sargeras;
pub mod granite_guardian;
pub mod king_varian;
pub mod moira_bronzebeard;
pub mod obsidian_ravager;
pub mod papa_bear;
pub mod recurring_nightmare;
pub mod sanguine_champion;
pub mod sea_witch_zar_jira;
pub mod the_boommobile;
pub mod tide_oracle_morgl;
pub mod amalgam;
pub mod annoy_o_spawn;
pub mod backpiggy_imp;
pub mod baltharak;
pub mod crab;
pub mod cubling;
pub mod damaged_golem;
pub mod devilsaur;
pub mod diablo_lord_of_terror;
pub mod emperor_cobra;
pub mod fiery_imp;
pub mod fish_of_n_zoth;
pub mod golden_monkey;
pub mod half_shell;
pub mod helping_hand;
pub mod hyena;
pub mod imp;
pub mod magtheridon_prime;
pub mod mechapony;
pub mod mechorse;
pub mod microbot;
pub mod onyxian_whelp;
pub mod ozumat_s_tentacle;
pub mod plant;
pub mod rat;
pub mod robosaur;
pub mod rusted_reggie;
pub mod shudderling;
pub mod sky_pirate;
pub mod smolderwing;
pub mod snake;
pub mod stone_elemental;
pub mod tabbycat;
pub mod tentacle_of_octosari;
pub mod water_droplet;
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
            position: None,
            event_handler: self.event_handler(),
        }
    }
}
impl Default for MinionVariant {
    fn default() -> Self {
        MinionVariant::Invalid
    }
}
#[derive(Clone, Copy)]
pub enum MinionVariant {
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
            MinionVariant::MalchezaarPrinceOfDance => {
                data::malchezaar_prince_of_dance::data()
            }
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
            MinionVariant::AgamagganTheGreatBoar => {
                data::agamaggan_the_great_boar::data()
            }
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
            MinionVariant::TransmutedBramblewitch => {
                data::transmuted_bramblewitch::data()
            }
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
    pub fn event_handler(self) -> EventHandler {
        match self {
            MinionVariant::Invalid => panic!("Invalid MinionVariant"),
            MinionVariant::Alleycat => alleycat::event_handler(),
            MinionVariant::AnnoyOTron => annoy_o_tron::event_handler(),
            MinionVariant::Bubblette => bubblette::event_handler(),
            MinionVariant::DeckSwabbie => deck_swabbie::event_handler(),
            MinionVariant::DozyWhelp => dozy_whelp::event_handler(),
            MinionVariant::EvolvingChromawing => evolving_chromawing::event_handler(),
            MinionVariant::IckyImp => icky_imp::event_handler(),
            MinionVariant::Imprisoner => imprisoner::event_handler(),
            MinionVariant::IncorporealCorporal => incorporeal_corporal::event_handler(),
            MinionVariant::Manasaber => manasaber::event_handler(),
            MinionVariant::MicroMummy => micro_mummy::event_handler(),
            MinionVariant::MiniMyrmidon => mini_myrmidon::event_handler(),
            MinionVariant::Mistake => mistake::event_handler(),
            MinionVariant::PickyEater => picky_eater::event_handler(),
            MinionVariant::Pupbot => pupbot::event_handler(),
            MinionVariant::RazorfenGeomancer => razorfen_geomancer::event_handler(),
            MinionVariant::RedWhelp => red_whelp::event_handler(),
            MinionVariant::RefreshingAnomaly => refreshing_anomaly::event_handler(),
            MinionVariant::RisenRider => risen_rider::event_handler(),
            MinionVariant::RockpoolHunter => rockpool_hunter::event_handler(),
            MinionVariant::RotHideGnoll => rot_hide_gnoll::event_handler(),
            MinionVariant::Scallywag => scallywag::event_handler(),
            MinionVariant::ScavengingHyena => scavenging_hyena::event_handler(),
            MinionVariant::Sellemental => sellemental::event_handler(),
            MinionVariant::ShellCollector => shell_collector::event_handler(),
            MinionVariant::SilverbackPatriarch => silverback_patriarch::event_handler(),
            MinionVariant::SouthseaBusker => southsea_busker::event_handler(),
            MinionVariant::SunBaconRelaxer => sun_bacon_relaxer::event_handler(),
            MinionVariant::SurfNSurf => surf_n_surf::event_handler(),
            MinionVariant::Swampstriker => swampstriker::event_handler(),
            MinionVariant::TavernTipper => tavern_tipper::event_handler(),
            MinionVariant::Thorncaptain => thorncaptain::event_handler(),
            MinionVariant::UpbeatFrontdrake => upbeat_frontdrake::event_handler(),
            MinionVariant::WrathWeaver => wrath_weaver::event_handler(),
            MinionVariant::AcolyteOfCThun => acolyte_of_c_thun::event_handler(),
            MinionVariant::BackstageSecurity => backstage_security::event_handler(),
            MinionVariant::BejeweledDuelist => bejeweled_duelist::event_handler(),
            MinionVariant::BlazingSkyfin => blazing_skyfin::event_handler(),
            MinionVariant::BriarbackBookie => briarback_bookie::event_handler(),
            MinionVariant::CogworkCopter => cogwork_copter::event_handler(),
            MinionVariant::CorpseRefiner => corpse_refiner::event_handler(),
            MinionVariant::DeepSeaAngler => deep_sea_angler::event_handler(),
            MinionVariant::EternalKnight => eternal_knight::event_handler(),
            MinionVariant::FlourishingFrostling => flourishing_frostling::event_handler(),
            MinionVariant::FreedealingGambler => freedealing_gambler::event_handler(),
            MinionVariant::GlyphGuardian => glyph_guardian::event_handler(),
            MinionVariant::HarvestGolem => harvest_golem::event_handler(),
            MinionVariant::HummingBird => humming_bird::event_handler(),
            MinionVariant::HungrySnapjaw => hungry_snapjaw::event_handler(),
            MinionVariant::ImpulsiveTrickster => impulsive_trickster::event_handler(),
            MinionVariant::InventOMatic => invent_o_matic::event_handler(),
            MinionVariant::KaboomBot => kaboom_bot::event_handler(),
            MinionVariant::KookyChemist => kooky_chemist::event_handler(),
            MinionVariant::LavaLurker => lava_lurker::event_handler(),
            MinionVariant::Leapfrogger => leapfrogger::event_handler(),
            MinionVariant::LowFlier => low_flier::event_handler(),
            MinionVariant::Lullabot => lullabot::event_handler(),
            MinionVariant::MenagerieMug => menagerie_mug::event_handler(),
            MinionVariant::MetaltoothLeaper => metaltooth_leaper::event_handler(),
            MinionVariant::MindMuck => mind_muck::event_handler(),
            MinionVariant::MoltenRock => molten_rock::event_handler(),
            MinionVariant::Murcules => murcules::event_handler(),
            MinionVariant::MurlocWarleader => murloc_warleader::event_handler(),
            MinionVariant::NathrezimOverseer => nathrezim_overseer::event_handler(),
            MinionVariant::NerubianDeathswarmer => nerubian_deathswarmer::event_handler(),
            MinionVariant::OozelingGladiator => oozeling_gladiator::event_handler(),
            MinionVariant::PatientScout => patient_scout::event_handler(),
            MinionVariant::PiggybackImp => piggyback_imp::event_handler(),
            MinionVariant::PoeticPenPal => poetic_pen_pal::event_handler(),
            MinionVariant::ProphetOfTheBoar => prophet_of_the_boar::event_handler(),
            MinionVariant::RabidSaurolisk => rabid_saurolisk::event_handler(),
            MinionVariant::ReefRiffer => reef_riffer::event_handler(),
            MinionVariant::RipsnarlCaptain => ripsnarl_captain::event_handler(),
            MinionVariant::Roadboar => roadboar::event_handler(),
            MinionVariant::SaltscaleHoncho => saltscale_honcho::event_handler(),
            MinionVariant::ScarletSkull => scarlet_skull::event_handler(),
            MinionVariant::SeabornSummoner => seaborn_summoner::event_handler(),
            MinionVariant::SelflessHero => selfless_hero::event_handler(),
            MinionVariant::SewerRat => sewer_rat::event_handler(),
            MinionVariant::SnailCavalry => snail_cavalry::event_handler(),
            MinionVariant::SoulRewinder => soul_rewinder::event_handler(),
            MinionVariant::SouthseaCaptain => southsea_captain::event_handler(),
            MinionVariant::SparringPartner => sparring_partner::event_handler(),
            MinionVariant::SpawnOfNZoth => spawn_of_n_zoth::event_handler(),
            MinionVariant::StewardOfTime => steward_of_time::event_handler(),
            MinionVariant::Tad => tad::event_handler(),
            MinionVariant::Thorncaller => thorncaller::event_handler(),
            MinionVariant::ToughTusk => tough_tusk::event_handler(),
            MinionVariant::TwilightEmissary => twilight_emissary::event_handler(),
            MinionVariant::UnstableGhoul => unstable_ghoul::event_handler(),
            MinionVariant::UpbeatFlutist => upbeat_flutist::event_handler(),
            MinionVariant::WhelpSmuggler => whelp_smuggler::event_handler(),
            MinionVariant::YoHoOgre => yo_ho_ogre::event_handler(),
            MinionVariant::Yrel => yrel::event_handler(),
            MinionVariant::AccordOTron => accord_o_tron::event_handler(),
            MinionVariant::AmberGuardian => amber_guardian::event_handler(),
            MinionVariant::ArmOfTheEmpire => arm_of_the_empire::event_handler(),
            MinionVariant::BirdBuddy => bird_buddy::event_handler(),
            MinionVariant::BloodsailCannoneer => bloodsail_cannoneer::event_handler(),
            MinionVariant::BrinyBootlegger => briny_bootlegger::event_handler(),
            MinionVariant::BristlebackBrute => bristleback_brute::event_handler(),
            MinionVariant::BristlemaneScrapsmith => {
                bristlemane_scrapsmith::event_handler()
            }
            MinionVariant::BronzeWarden => bronze_warden::event_handler(),
            MinionVariant::BuddingGreenthumb => budding_greenthumb::event_handler(),
            MinionVariant::ColdlightSeer => coldlight_seer::event_handler(),
            MinionVariant::CracklingCyclone => crackling_cyclone::event_handler(),
            MinionVariant::DaggerspineThrasher => daggerspine_thrasher::event_handler(),
            MinionVariant::DeflectOBot => deflect_o_bot::event_handler(),
            MinionVariant::Dreadbeard => dreadbeard::event_handler(),
            MinionVariant::EventideBrute => eventide_brute::event_handler(),
            MinionVariant::FacelessDisciple => faceless_disciple::event_handler(),
            MinionVariant::Felemental => felemental::event_handler(),
            MinionVariant::FelfinNavigator => felfin_navigator::event_handler(),
            MinionVariant::FirstMatePip => first_mate_pip::event_handler(),
            MinionVariant::FreeFlyingFeathermane => {
                free_flying_feathermane::event_handler()
            }
            MinionVariant::Gemsplitter => gemsplitter::event_handler(),
            MinionVariant::GhoulOfTheFeast => ghoul_of_the_feast::event_handler(),
            MinionVariant::GunpowderCourier => gunpowder_courier::event_handler(),
            MinionVariant::HandlessForsaken => handless_forsaken::event_handler(),
            MinionVariant::Houndmaster => houndmaster::event_handler(),
            MinionVariant::IronGroundskeeper => iron_groundskeeper::event_handler(),
            MinionVariant::IronSensei => iron_sensei::event_handler(),
            MinionVariant::JellyBelly => jelly_belly::event_handler(),
            MinionVariant::KathraNatir => kathra_natir::event_handler(),
            MinionVariant::KeyboardIgniter => keyboard_igniter::event_handler(),
            MinionVariant::Khadgar => khadgar::event_handler(),
            MinionVariant::LeechingFelhound => leeching_felhound::event_handler(),
            MinionVariant::LegionOverseer => legion_overseer::event_handler(),
            MinionVariant::LichDoctor => lich_doctor::event_handler(),
            MinionVariant::LivingConstellation => living_constellation::event_handler(),
            MinionVariant::MonstrousMacaw => monstrous_macaw::event_handler(),
            MinionVariant::MoonBaconJazzer => moon_bacon_jazzer::event_handler(),
            MinionVariant::NetherDrake => nether_drake::event_handler(),
            MinionVariant::NightmareAmalgam => nightmare_amalgam::event_handler(),
            MinionVariant::PartyElemental => party_elemental::event_handler(),
            MinionVariant::PashmarTheVengeful => pashmar_the_vengeful::event_handler(),
            MinionVariant::PricklyPiper => prickly_piper::event_handler(),
            MinionVariant::Pufferquil => pufferquil::event_handler(),
            MinionVariant::RadioStar => radio_star::event_handler(),
            MinionVariant::RatPack => rat_pack::event_handler(),
            MinionVariant::RecyclingWraith => recycling_wraith::event_handler(),
            MinionVariant::RelentlessSentry => relentless_sentry::event_handler(),
            MinionVariant::ReplicatingMenace => replicating_menace::event_handler(),
            MinionVariant::SaltyLooter => salty_looter::event_handler(),
            MinionVariant::Scourfin => scourfin::event_handler(),
            MinionVariant::ScrewjankClunker => screwjank_clunker::event_handler(),
            MinionVariant::ShifterZerus => shifter_zerus::event_handler(),
            MinionVariant::ShoalCommander => shoal_commander::event_handler(),
            MinionVariant::Smogger => smogger::event_handler(),
            MinionVariant::SoreLoser => sore_loser::event_handler(),
            MinionVariant::SoulDevourer => soul_devourer::event_handler(),
            MinionVariant::SoulJuggler => soul_juggler::event_handler(),
            MinionVariant::SouthseaStrongarm => southsea_strongarm::event_handler(),
            MinionVariant::SparkLing => spark_ling::event_handler(),
            MinionVariant::StasisElemental => stasis_elemental::event_handler(),
            MinionVariant::Swolefin => swolefin::event_handler(),
            MinionVariant::Tarecgosa => tarecgosa::event_handler(),
            MinionVariant::TheGladIator => the_glad_iator::event_handler(),
            MinionVariant::TimeSaver => time_saver::event_handler(),
            MinionVariant::WardenOfOld => warden_of_old::event_handler(),
            MinionVariant::WitheredSpearhide => withered_spearhide::event_handler(),
            MinionVariant::ZestyShaker => zesty_shaker::event_handler(),
            MinionVariant::AnnihilanBattlemaster => {
                annihilan_battlemaster::event_handler()
            }
            MinionVariant::AnnoyOModule => annoy_o_module::event_handler(),
            MinionVariant::AnubArakNerubianKing => {
                anub_arak_nerubian_king::event_handler()
            }
            MinionVariant::Atramedes => atramedes::event_handler(),
            MinionVariant::BallOfMinions => ball_of_minions::event_handler(),
            MinionVariant::BananaSlamma => banana_slamma::event_handler(),
            MinionVariant::Bannerboar => bannerboar::event_handler(),
            MinionVariant::Bassgill => bassgill::event_handler(),
            MinionVariant::Bigfernal => bigfernal::event_handler(),
            MinionVariant::BladeCollector => blade_collector::event_handler(),
            MinionVariant::Bonker => bonker::event_handler(),
            MinionVariant::BreamCounter => bream_counter::event_handler(),
            MinionVariant::CarbonicCopy => carbonic_copy::event_handler(),
            MinionVariant::CaveHydra => cave_hydra::event_handler(),
            MinionVariant::ChampionOfYShaarj => champion_of_y_shaarj::event_handler(),
            MinionVariant::CobaltScalebane => cobalt_scalebane::event_handler(),
            MinionVariant::DancingBarnstormer => dancing_barnstormer::event_handler(),
            MinionVariant::DazzlingLightspawn => dazzling_lightspawn::event_handler(),
            MinionVariant::DeepBlueCrooner => deep_blue_crooner::event_handler(),
            MinionVariant::DefenderOfArgus => defender_of_argus::event_handler(),
            MinionVariant::DrakonidEnforcer => drakonid_enforcer::event_handler(),
            MinionVariant::DynamicDuo => dynamic_duo::event_handler(),
            MinionVariant::EelboundArcher => eelbound_archer::event_handler(),
            MinionVariant::ElectricSynthesizer => electric_synthesizer::event_handler(),
            MinionVariant::EmergentFlame => emergent_flame::event_handler(),
            MinionVariant::FairyTaleCaroler => fairy_tale_caroler::event_handler(),
            MinionVariant::FireworksFanatic => fireworks_fanatic::event_handler(),
            MinionVariant::FloatingWatcher => floating_watcher::event_handler(),
            MinionVariant::GemSmuggler => gem_smuggler::event_handler(),
            MinionVariant::Goldgrubber => goldgrubber::event_handler(),
            MinionVariant::Groundshaker => groundshaker::event_handler(),
            MinionVariant::ImpatientDoomsayer => impatient_doomsayer::event_handler(),
            MinionVariant::LovesickBalladist => lovesick_balladist::event_handler(),
            MinionVariant::MajordomoExecutus => majordomo_executus::event_handler(),
            MinionVariant::MalchezaarPrinceOfDance => {
                malchezaar_prince_of_dance::event_handler()
            }
            MinionVariant::MasterOfRealities => master_of_realities::event_handler(),
            MinionVariant::MechanoEgg => mechano_egg::event_handler(),
            MinionVariant::MechanoTank => mechano_tank::event_handler(),
            MinionVariant::MenagerieJug => menagerie_jug::event_handler(),
            MinionVariant::Necrolyte => necrolyte::event_handler(),
            MinionVariant::PeckishFeldrake => peckish_feldrake::event_handler(),
            MinionVariant::PeggyBrittlebone => peggy_brittlebone::event_handler(),
            MinionVariant::PeggySturdybone => peggy_sturdybone::event_handler(),
            MinionVariant::PlaguedTidewalker => plagued_tidewalker::event_handler(),
            MinionVariant::PossessiveBanshee => possessive_banshee::event_handler(),
            MinionVariant::PrestorSPyrospawn => prestor_s_pyrospawn::event_handler(),
            MinionVariant::PrimalfinLookout => primalfin_lookout::event_handler(),
            MinionVariant::PrizedPromoDrake => prized_promo_drake::event_handler(),
            MinionVariant::QirajiHarbinger => qiraji_harbinger::event_handler(),
            MinionVariant::ReanimatingRattler => reanimating_rattler::event_handler(),
            MinionVariant::ReefExplorer => reef_explorer::event_handler(),
            MinionVariant::RendleTheMistermind => rendle_the_mistermind::event_handler(),
            MinionVariant::RingMatron => ring_matron::event_handler(),
            MinionVariant::RylakMetalhead => rylak_metalhead::event_handler(),
            MinionVariant::SavannahHighmane => savannah_highmane::event_handler(),
            MinionVariant::ScrapScraper => scrap_scraper::event_handler(),
            MinionVariant::SilentSwimmer => silent_swimmer::event_handler(),
            MinionVariant::SinDoreiStraightShot => {
                sin_dorei_straight_shot::event_handler()
            }
            MinionVariant::SlyRaptor => sly_raptor::event_handler(),
            MinionVariant::Soulsplitter => soulsplitter::event_handler(),
            MinionVariant::Stormbringer => stormbringer::event_handler(),
            MinionVariant::StrongshellScavenger => strongshell_scavenger::event_handler(),
            MinionVariant::TreasureSeekerElise => treasure_seeker_elise::event_handler(),
            MinionVariant::TunnelBlaster => tunnel_blaster::event_handler(),
            MinionVariant::UpbeatUpstart => upbeat_upstart::event_handler(),
            MinionVariant::VigilantStoneborn => vigilant_stoneborn::event_handler(),
            MinionVariant::Wargear => wargear::event_handler(),
            MinionVariant::Waverider => waverider::event_handler(),
            MinionVariant::WildfireElemental => wildfire_elemental::event_handler(),
            MinionVariant::WitchwingNestmatron => witchwing_nestmatron::event_handler(),
            MinionVariant::XyloBones => xylo_bones::event_handler(),
            MinionVariant::AdaptableBarricade => adaptable_barricade::event_handler(),
            MinionVariant::AgamagganTheGreatBoar => {
                agamaggan_the_great_boar::event_handler()
            }
            MinionVariant::AggemThorncurse => aggem_thorncurse::event_handler(),
            MinionVariant::AnnoyOTroupe => annoy_o_troupe::event_handler(),
            MinionVariant::BabyKrush => baby_krush::event_handler(),
            MinionVariant::BaronRivendare => baron_rivendare::event_handler(),
            MinionVariant::Bonemare => bonemare::event_handler(),
            MinionVariant::BongoBopper => bongo_bopper::event_handler(),
            MinionVariant::BrannBronzebeard => brann_bronzebeard::event_handler(),
            MinionVariant::BristlebackKnight => bristleback_knight::event_handler(),
            MinionVariant::CapNHoggarr => cap_n_hoggarr::event_handler(),
            MinionVariant::ChampionOfThePrimus => champion_of_the_primus::event_handler(),
            MinionVariant::Chronormu => chronormu::event_handler(),
            MinionVariant::CorruptedMyrmidon => corrupted_myrmidon::event_handler(),
            MinionVariant::CritterWrangler => critter_wrangler::event_handler(),
            MinionVariant::CyborgDrake => cyborg_drake::event_handler(),
            MinionVariant::DeadlySpore => deadly_spore::event_handler(),
            MinionVariant::DiscoShuffler => disco_shuffler::event_handler(),
            MinionVariant::DrBoombox => dr_boombox::event_handler(),
            MinionVariant::DrakkariEnchanter => drakkari_enchanter::event_handler(),
            MinionVariant::FriendOfAFriend => friend_of_a_friend::event_handler(),
            MinionVariant::GeneralDrakkisath => general_drakkisath::event_handler(),
            MinionVariant::Glowscale => glowscale::event_handler(),
            MinionVariant::GustyTrumpeter => gusty_trumpeter::event_handler(),
            MinionVariant::HolyMecherel => holy_mecherel::event_handler(),
            MinionVariant::HungeringAbomination => hungering_abomination::event_handler(),
            MinionVariant::HunterOfGatherers => hunter_of_gatherers::event_handler(),
            MinionVariant::ImposingPercussionist => {
                imposing_percussionist::event_handler()
            }
            MinionVariant::InsatiableUrZul => insatiable_ur_zul::event_handler(),
            MinionVariant::InterrogatorWhitemane => {
                interrogator_whitemane::event_handler()
            }
            MinionVariant::KangorSApprentice => kangor_s_apprentice::event_handler(),
            MinionVariant::KingBagurgle => king_bagurgle::event_handler(),
            MinionVariant::LeeroyTheReckless => leeroy_the_reckless::event_handler(),
            MinionVariant::LightfangEnforcer => lightfang_enforcer::event_handler(),
            MinionVariant::LilRag => lil_rag::event_handler(),
            MinionVariant::Magmaloc => magmaloc::event_handler(),
            MinionVariant::MamaBear => mama_bear::event_handler(),
            MinionVariant::Mannoroth => mannoroth::event_handler(),
            MinionVariant::MechanizedGiftHorse => mechanized_gift_horse::event_handler(),
            MinionVariant::Murozond => murozond::event_handler(),
            MinionVariant::MythraxTheUnraveler => mythrax_the_unraveler::event_handler(),
            MinionVariant::Niuzao => niuzao::event_handler(),
            MinionVariant::NomiKitchenNightmare => {
                nomi_kitchen_nightmare::event_handler()
            }
            MinionVariant::OperaticBelcher => operatic_belcher::event_handler(),
            MinionVariant::PalescaleCrocolisk => palescale_crocolisk::event_handler(),
            MinionVariant::RazorgoreTheUntamed => razorgore_the_untamed::event_handler(),
            MinionVariant::RecordSmuggler => record_smuggler::event_handler(),
            MinionVariant::SanctumRester => sanctum_rester::event_handler(),
            MinionVariant::SiSefin => si_sefin::event_handler(),
            MinionVariant::SinrunnerBlanchy => sinrunner_blanchy::event_handler(),
            MinionVariant::StormscaleSiren => stormscale_siren::event_handler(),
            MinionVariant::TavernTempest => tavern_tempest::event_handler(),
            MinionVariant::Tichondrius => tichondrius::event_handler(),
            MinionVariant::TitusRivendare => titus_rivendare::event_handler(),
            MinionVariant::TonyTwoTusk => tony_two_tusk::event_handler(),
            MinionVariant::TortollanBlueShell => tortollan_blue_shell::event_handler(),
            MinionVariant::Toxfin => toxfin::event_handler(),
            MinionVariant::TransmutedBramblewitch => {
                transmuted_bramblewitch::event_handler()
            }
            MinionVariant::UnderhandedDealer => underhanded_dealer::event_handler(),
            MinionVariant::UpbeatDuo => upbeat_duo::event_handler(),
            MinionVariant::UpbeatImpressionist => upbeat_impressionist::event_handler(),
            MinionVariant::UtilityDrone => utility_drone::event_handler(),
            MinionVariant::VanessaVanCleef => vanessa_van_cleef::event_handler(),
            MinionVariant::Voidlord => voidlord::event_handler(),
            MinionVariant::ArchdruidHamuul => archdruid_hamuul::event_handler(),
            MinionVariant::Bristlebach => bristlebach::event_handler(),
            MinionVariant::CaptainFlatTusk => captain_flat_tusk::event_handler(),
            MinionVariant::Charlga => charlga::event_handler(),
            MinionVariant::ChoralMrrrglr => choral_mrrrglr::event_handler(),
            MinionVariant::ColossusOfTheSun => colossus_of_the_sun::event_handler(),
            MinionVariant::DarkgazeElder => darkgaze_elder::event_handler(),
            MinionVariant::DreadAdmiralEliza => dread_admiral_eliza::event_handler(),
            MinionVariant::ElementalOfSurprise => elemental_of_surprise::event_handler(),
            MinionVariant::EternalSummoner => eternal_summoner::event_handler(),
            MinionVariant::FamishedFelbat => famished_felbat::event_handler(),
            MinionVariant::Felstomper => felstomper::event_handler(),
            MinionVariant::FleetAdmiralTethys => fleet_admiral_tethys::event_handler(),
            MinionVariant::FoeReaper4000 => foe_reaper_4000::event_handler(),
            MinionVariant::GentleDjinni => gentle_djinni::event_handler(),
            MinionVariant::Ghastcoiler => ghastcoiler::event_handler(),
            MinionVariant::GoldrinnTheGreatWolf => {
                goldrinn_the_great_wolf::event_handler()
            }
            MinionVariant::GreaseBot => grease_bot::event_handler(),
            MinionVariant::GretaGoldGun => greta_gold_gun::event_handler(),
            MinionVariant::ImpMama => imp_mama::event_handler(),
            MinionVariant::KalecgosArcaneAspect => {
                kalecgos_arcane_aspect::event_handler()
            }
            MinionVariant::LieutenantGarr => lieutenant_garr::event_handler(),
            MinionVariant::Maexxna => maexxna::event_handler(),
            MinionVariant::MantidQueen => mantid_queen::event_handler(),
            MinionVariant::MechaJaraxxus => mecha_jaraxxus::event_handler(),
            MinionVariant::Murky => murky::event_handler(),
            MinionVariant::NadinaTheRed => nadina_the_red::event_handler(),
            MinionVariant::NosyLooter => nosy_looter::event_handler(),
            MinionVariant::OctosariWrapGod => octosari_wrap_god::event_handler(),
            MinionVariant::OmegaBuster => omega_buster::event_handler(),
            MinionVariant::OrgozoaTheTender => orgozoa_the_tender::event_handler(),
            MinionVariant::PolarizingBeatboxer => polarizing_beatboxer::event_handler(),
            MinionVariant::RelentlessMurGhoul => relentless_mur_ghoul::event_handler(),
            MinionVariant::RockRock => rock_rock::event_handler(),
            MinionVariant::SeafoodSlinger => seafood_slinger::event_handler(),
            MinionVariant::SisterDeathwhisper => sister_deathwhisper::event_handler(),
            MinionVariant::TeaMasterTheotar => tea_master_theotar::event_handler(),
            MinionVariant::TheBoogieMonster => the_boogie_monster::event_handler(),
            MinionVariant::TheWalkingFort => the_walking_fort::event_handler(),
            MinionVariant::TidemistressAthissa => tidemistress_athissa::event_handler(),
            MinionVariant::UtherTheLightbringer => {
                uther_the_lightbringer::event_handler()
            }
            MinionVariant::Warpwing => warpwing::event_handler(),
            MinionVariant::YoungMurkEye => young_murk_eye::event_handler(),
            MinionVariant::ZappSlywick => zapp_slywick::event_handler(),
            MinionVariant::Amalgadon => amalgadon::event_handler(),
            MinionVariant::ArgentBraggart => argent_braggart::event_handler(),
            MinionVariant::CaptainSanders => captain_sanders::event_handler(),
            MinionVariant::ChampionOfSargeras => champion_of_sargeras::event_handler(),
            MinionVariant::GraniteGuardian => granite_guardian::event_handler(),
            MinionVariant::KingVarian => king_varian::event_handler(),
            MinionVariant::MoiraBronzebeard => moira_bronzebeard::event_handler(),
            MinionVariant::ObsidianRavager => obsidian_ravager::event_handler(),
            MinionVariant::PapaBear => papa_bear::event_handler(),
            MinionVariant::RecurringNightmare => recurring_nightmare::event_handler(),
            MinionVariant::SanguineChampion => sanguine_champion::event_handler(),
            MinionVariant::SeaWitchZarJira => sea_witch_zar_jira::event_handler(),
            MinionVariant::TheBoommobile => the_boommobile::event_handler(),
            MinionVariant::TideOracleMorgl => tide_oracle_morgl::event_handler(),
            MinionVariant::Amalgam => amalgam::event_handler(),
            MinionVariant::AnnoyOSpawn => annoy_o_spawn::event_handler(),
            MinionVariant::BackpiggyImp => backpiggy_imp::event_handler(),
            MinionVariant::Baltharak => baltharak::event_handler(),
            MinionVariant::Crab => crab::event_handler(),
            MinionVariant::Cubling => cubling::event_handler(),
            MinionVariant::DamagedGolem => damaged_golem::event_handler(),
            MinionVariant::Devilsaur => devilsaur::event_handler(),
            MinionVariant::DiabloLordOfTerror => diablo_lord_of_terror::event_handler(),
            MinionVariant::EmperorCobra => emperor_cobra::event_handler(),
            MinionVariant::FieryImp => fiery_imp::event_handler(),
            MinionVariant::FishOfNZoth => fish_of_n_zoth::event_handler(),
            MinionVariant::GoldenMonkey => golden_monkey::event_handler(),
            MinionVariant::HalfShell => half_shell::event_handler(),
            MinionVariant::HelpingHand => helping_hand::event_handler(),
            MinionVariant::Hyena => hyena::event_handler(),
            MinionVariant::Imp => imp::event_handler(),
            MinionVariant::MagtheridonPrime => magtheridon_prime::event_handler(),
            MinionVariant::Mechapony => mechapony::event_handler(),
            MinionVariant::Mechorse => mechorse::event_handler(),
            MinionVariant::Microbot => microbot::event_handler(),
            MinionVariant::OnyxianWhelp => onyxian_whelp::event_handler(),
            MinionVariant::OzumatSTentacle => ozumat_s_tentacle::event_handler(),
            MinionVariant::Plant => plant::event_handler(),
            MinionVariant::Rat => rat::event_handler(),
            MinionVariant::Robosaur => robosaur::event_handler(),
            MinionVariant::RustedReggie => rusted_reggie::event_handler(),
            MinionVariant::Shudderling => shudderling::event_handler(),
            MinionVariant::SkyPirate => sky_pirate::event_handler(),
            MinionVariant::Smolderwing => smolderwing::event_handler(),
            MinionVariant::Snake => snake::event_handler(),
            MinionVariant::StoneElemental => stone_elemental::event_handler(),
            MinionVariant::Tabbycat => tabbycat::event_handler(),
            MinionVariant::TentacleOfOctosari => tentacle_of_octosari::event_handler(),
            MinionVariant::WaterDroplet => water_droplet::event_handler(),
        }
    }
    pub fn random() -> Self {
        *[
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
        ]
            .choose(&mut rand::thread_rng())
            .unwrap()
    }
}
