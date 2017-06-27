

#[derive(Debug, Clone, Copy)]
pub enum CurrencyType {
    ScrollFragment,
    ScrollofWisdom,
    PortalScroll,
    TransmutationShard,
    OrbofTransmutation,
    OrbofAugmentation,
    AlterationShard,
    OrbofAlteration,
    AlchemyShard,
    OrbofAlchemy,
    ExaltedOrb,
    ChaosOrb,
    RegalOrb,
    OrbofChance,
    VaalOrb,
    MirrorofKalandra,
    DivineOrb,
    BlessedOrb,
    OrbofScouring,
    OrbofRegret,
    JewelersOrb,
    OrbofFusing,
    ChromaticOrb,
    ArmourersScrap,
    BlacksmithsWhetstone,
    CartographersChisel,
    GemcuttersPrism,
    GlassblowersBauble,
    ApprenticeCartographersSextant,
    JourneymanCartographersSextant,
    MasterCartographersSextant,
    UnshapingOrb,
    ApprenticeCartographersSeal,
    JourneymanCartographersSeal,
    MasterCartographersSeal,
    SilverCoin,
    PerandusCoin,
    SplinterofChayula,
    BlessingofChayula,
    SplinterofEsh,
    BlessingofEsh,
    SplinterofTul,
    BlessingofTul,
    SplinterofXoph,
    BlessingofXoph,
    SplinterofUulNetol,
    BlessingofUulNetol,
    StackedDeck,
    AlbinoRhoaFeather,
}


impl From<String> for CurrencyType {
    fn from(s: String) -> Self {
        use self::CurrencyType::*;
        use std::collections::HashMap;

        lazy_static!{
            static ref CURRMAP: HashMap<&'static str, CurrencyType> = {
                let mut m = HashMap::with_capacity(60);
                m.insert("Scroll of Wisdom",  ScrollofWisdom);
                m.insert("Scroll Fragment",  ScrollFragment);
                m.insert("Portal Scroll",  PortalScroll);
                m.insert("Transmutation Shard",  TransmutationShard);
                m.insert("Orb of Transmutation",  OrbofTransmutation);
                m.insert("Orb of Augmentation",  OrbofAugmentation);
                m.insert("Alteration Shard",  AlterationShard);
                m.insert("Orb of Alteration",  OrbofAlteration);
                m.insert("Alchemy Shard",  AlchemyShard);
                m.insert("Orb of Alchemy",  OrbofAlchemy);
                m.insert("Exalted Orb",  ExaltedOrb);
                m.insert("Chaos Orb",  ChaosOrb);
                m.insert("Regal Orb",  RegalOrb);
                m.insert("Orb of Chance",  OrbofChance);
                m.insert("Vaal Orb",  VaalOrb);
                m.insert("Mirror of Kalandra",  MirrorofKalandra);
                m.insert("Divine Orb",  DivineOrb);
                m.insert("Blessed Orb",  BlessedOrb);
                m.insert("Orb of Scouring",  OrbofScouring);
                m.insert("Orb of Regret",  OrbofRegret);
                m.insert("Jewelers Orb",  JewelersOrb);
                m.insert("Orb of Fusing",  OrbofFusing);
                m.insert("Chromatic Orb",  ChromaticOrb);
                m.insert("Armourers Scrap",  ArmourersScrap);
                m.insert("Blacksmith's Whetstone",  BlacksmithsWhetstone);
                m.insert("Cartographer's Chisel", CartographersChisel);
                m.insert("Gemcutter's Prism",  GemcuttersPrism);
                m.insert("Glasblower's Bauble",  GlassblowersBauble);
                m.insert("Apprentice Cartographer's Sextant",  ApprenticeCartographersSextant);
                m.insert("Journeyman Cartographer's Sextant",  JourneymanCartographersSextant);
                m.insert("Master Cartographer's Sextant",  MasterCartographersSextant);
                m.insert("Unshaping Orb",  UnshapingOrb);
                m.insert("Apprentice Cartographer's Seal",  ApprenticeCartographersSeal);
                m.insert("Journeyman Cartographer's Seal",  JourneymanCartographersSeal);
                m.insert("Master Cartographer's Seal",  MasterCartographersSeal);
                m.insert("Silver Coin",  SilverCoin);
                m.insert("Perandus Coin",  PerandusCoin);
                m.insert("Splinter of Chayula",  SplinterofChayula);
                m.insert("Blessing of Chayula",  BlessingofChayula);
                m.insert("Splinter of Esh",  SplinterofEsh);
                m.insert("Blessing of Esh",  BlessingofEsh);
                m.insert("Splinter of Tul",  SplinterofTul);
                m.insert("Blessing of Tul",  BlessingofTul);
                m.insert("Splinter of Xoph",  SplinterofXoph);
                m.insert("Blessing of Xoph",  BlessingofXoph);
                m.insert("Splinter of Uul-Netol",  SplinterofUulNetol);
                m.insert("Blessing of Uul-Netol",  BlessingofUulNetol);
                m.insert("Stacked Deck",  StackedDeck);
                m.insert("Albino Rhoa Feather",  AlbinoRhoaFeather);
                m
            };
        }
        *CURRMAP.get(s.as_str()).unwrap()

        }
}
