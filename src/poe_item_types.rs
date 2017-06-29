
use std::convert::{TryFrom,TryInto};
use errors::*;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    RemnantofCorruption,
    Essence(Essence)
}


impl TryFrom<String> for CurrencyType {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
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
                m.insert("Jeweller's Orb",  JewelersOrb);
                m.insert("Orb of Fusing",  OrbofFusing);
                m.insert("Chromatic Orb",  ChromaticOrb);
                m.insert("Armourer's Scrap",  ArmourersScrap);
                m.insert("Blacksmith's Whetstone",  BlacksmithsWhetstone);
                m.insert("Cartographer's Chisel", CartographersChisel);
                m.insert("Gemcutter's Prism",  GemcuttersPrism);
                m.insert("Glassblower's Bauble",  GlassblowersBauble);
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
                m.insert("Remnant of Corruption", RemnantofCorruption);
                m
            };
        }

        Ok(
            match CURRMAP.get(s.as_str()) {
                Some(ct) => ct.clone(),
                None => {
                    if s.contains("Essence") {
                        Essence(s.as_str().try_into()?)
                    } else {
                        bail!("Unknown Currencytype \'{}\'", s)
                    }
                }
            }
        )
    }
}

#[derive(Debug,Clone,Copy, PartialEq)]
pub enum Essence {
    Greed(EssenceTier),
    Hatred(EssenceTier),
    Woe(EssenceTier),
    Contempt(EssenceTier),
    Sorrow(EssenceTier),
    Anger(EssenceTier),
    Torment(EssenceTier),
    Fear(EssenceTier),
    Suffering(EssenceTier),
    Rage(EssenceTier),
    Wrath(EssenceTier),
    Doubt(EssenceTier),
    Anguish(EssenceTier),
    Loathing(EssenceTier),
    Spite(EssenceTier),
    Zeal(EssenceTier),
    Misery(EssenceTier),
    Dread(EssenceTier),
    Scorn(EssenceTier),
    Envy(EssenceTier),
    Hysteria,
    Insanity,
    Horror,
    Delirium,
}

impl<'a> TryFrom<&'a str> for Essence {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        use self::Essence::*;
        let tier: Result<EssenceTier> = s.split_whitespace().next().expect("unreachable!").try_into();
        Ok(
            match s {
                s if s.contains("Greed") => Greed(tier?),
                s if s.contains("Hatred") => Hatred(tier?),
                s if s.contains("Woe") => Woe(tier?),
                s if s.contains("Contempt") => Contempt(tier?),
                s if s.contains("Sorrow") => Sorrow(tier?),
                s if s.contains("Anger") => Anger(tier?),
                s if s.contains("Torment") => Torment(tier?),
                s if s.contains("Fear") => Fear(tier?),
                s if s.contains("Suffering") => Suffering(tier?),
                s if s.contains("Rage") => Rage(tier?),
                s if s.contains("Wrath") => Wrath(tier?),
                s if s.contains("Doubt") => Doubt(tier?),
                s if s.contains("Anguish") => Anguish(tier?),
                s if s.contains("Loathing") => Loathing(tier?),
                s if s.contains("Spite") => Spite(tier?),
                s if s.contains("Zeal") => Zeal(tier?),
                s if s.contains("Misery") => Misery(tier?),
                s if s.contains("Dread") => Dread(tier?),
                s if s.contains("Scorn") => Scorn(tier?),
                s if s.contains("Envy") => Envy(tier?),
                s if s.contains("Hysteria") => Hysteria,
                s if s.contains("Insanity") => Insanity,
                s if s.contains("Horror") => Horror,
                s if s.contains("Delirium") => Delirium,
                _ => bail!("Unknown EssenceType: \'{}\'", s)
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EssenceTier{
    Whispering,
    Muttering,
    Weeping,
    Wailing,
    Screaming,
    Shrieking,
    Deafening
}

impl<'a> TryFrom<&'a str> for EssenceTier {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        use self::EssenceTier::*;
        Ok(
            match s {
                "Whispering" => Whispering,
                "Muttering" => Muttering,
                "Weeping" => Weeping,
                "Wailing" => Wailing,
                "Screaming" => Screaming,
                "Shrieking" => Shrieking,
                "Deafening" => Deafening,
                _ => bail!("Unknown EssenceTier \'{}\'", s)
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::CurrencyType;
    use std::convert::TryInto;
    use errors::*;

    #[test]
    fn into_currencytype() {
        let ct: Result<CurrencyType> = "Regal Orb".to_owned().try_into();
        println!("{:?}", ct);
        assert!(match ct {
            Ok(c) => c == CurrencyType::RegalOrb,
            _ => false
        });

        let ct: Result<CurrencyType> = "Armourer's Scrap".to_owned().try_into();
        assert!(match ct {
            Ok(c) => c == CurrencyType::ArmourersScrap,
            _ => false
        });

        let ct: Result<CurrencyType> = "Chaos Orb".to_owned().try_into();
        assert!(match ct {
            Ok(c) => c == CurrencyType::ChaosOrb,
            _ => false
        });
    }
}
