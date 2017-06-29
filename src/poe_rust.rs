
use poe_json::ApiProperties;
use poe_item_types::CurrencyType;
use std::convert::{TryFrom, TryInto};
use std::fmt::{self, Display};



use errors::*;

#[derive(Debug)]
enum ItemSpecifics {
    Currency { c_type: CurrencyType, stack_size: u16}, //make enum
//     Gear{g_type: GearType, quality: u8, mods: Vec<ItemMod>, def: Vec<Defences>}, // everything equippable with sockets
//     Jewel{j_type: JewelType, mods: Vec<JewelMod>}, // enum for suff/prefix or (Option<Jewelmod>, Option<JewelMod>)
//     Flask{quality: u8, f_type: Flasktype}, // Flasktype should be enum for lifeflask with tier
//     Misc{m_type: MiscType, mods: Vec<ItemMod>},
//     Gem{ quality: u8, level: u8, },
//     DivinationCard{ name: String, stack_size: u16},//make enum? or indicate by String?
//     Prophecy{name: String,}, //make enum? or indicate by String?
//     Map{name: String, tier: u8, quality: u8, map_mods: Vec<MapMods>},
//     MapFragments{frag_type: MapFragmentType}, //sacrifice ... (are normal items)
//     LeagueStone{ls_type: LeagueStoneType, mods: Vec<LeagueStoneMod>}  //normal items aswell

}


#[derive(Debug, PartialEq)]
enum Requirement {
    Str(u16),
    Dex(u16),
    Int(u16),
    Lvl(u16),
}


#[derive(Debug, PartialEq)]
enum StashType {
    NormalStash,
    PremiumStash,
    QuadStash,
    EssenceStash,
    CurrencyStash,
    DivinationStash,
}

#[derive(Debug, PartialEq)]
struct Sockets(Vec<(SocketColour, u8)>);

#[derive(Debug, PartialEq)]
enum SocketColour {
    Red,
    Green,
    Blue,
    White,
}

#[derive(Debug, PartialEq)]
enum PropertyColour {
    White,
    Blue,
    Fire,
    Cold,
    Lightning,
    Chaos,
}

#[derive(Debug, PartialEq)]
pub enum FrameType {
    Normal,
    Magic,
    Rare,
    Unique,
    Gem,
    Currency,
    DivinationCard,
    QuestItem,
    Prophecy,
    Relic,
}

impl TryFrom<ApiProperties> for Requirement {
    type Error = Error;
    fn try_from(prop: ApiProperties) -> Result<Self> {
        use self::Requirement::*;
        Ok(if let Some(&(ref val, _)) = prop.values.get(0) {

            let value: u16 = val.parse().chain_err(|| {
                ErrorKind::InvalidRequirementValue(val.clone())
            })?;

            match prop.name.as_str() {
                "Str" => Str(value),
                "Dex" => Dex(value),
                "Int" => Int(value),
                "Level" => Lvl(value),
                _ => bail!(ErrorKind::InvalidRequirementName(prop.name)),
            }
        } else {
            bail!(ErrorKind::RequirementArrayEmpty)
        })

    }
}

impl TryFrom<u8> for FrameType {
    type Error = Error;
    fn try_from(number: u8) -> Result<Self> {
        use self::FrameType::*;
        Ok(match number {
            0 => Normal,
            1 => Magic,
            2 => Rare,
            3 => Unique,
            4 => Gem,
            5 => Currency,
            6 => DivinationCard,
            7 => QuestItem,
            8 => Prophecy,
            9 => Relic,
            _ => bail!("Unknown Frametype: {}", number),
        })
    }
}

impl TryFrom<u8> for PropertyColour {
    type Error = Error;
    fn try_from(number: u8) -> Result<Self> {
        use self::PropertyColour::*;
        Ok(match number {
            0 => White,
            1 => Blue,
            2 => Fire,
            3 => Cold,
            4 => Lightning,
            5 => Chaos,
            _ => bail!("Unkown PropertyColour {}", number),
        })
    }
}

impl TryFrom<String> for StashType {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        use self::StashType::*;
        Ok(match s.as_str() {
            "NormalStash" => NormalStash,
            "PremiumStash" => PremiumStash,
            "QuadStash" => QuadStash,
            "EssenceStash" => EssenceStash,
            "CurrencyStash" => CurrencyStash,
            "DivinationStash" => DivinationStash,
            _ => bail!("Unkown Stashtype: {}", s),
        })
    }
}

impl TryFrom<String> for SocketColour {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        use self::SocketColour::*;
        Ok(match s.as_ref() {
            "D" => Green,
            "S" => Red,
            "I" => Blue,
            "G" => White,
            _ => bail!("Unknown SocketColour {}", s),
        })
    }
}

impl TryFrom<Vec<(u8, String)>> for Sockets {
    type Error = Error;
    fn try_from(v: Vec<(u8, String)>) -> Result<Self> {
        let res: Result<Vec<_>> = v.into_iter()
            .map(|(grp, col)| match col.try_into() {
                Ok(c) => Ok((c, grp)),
                Err(e) => Err(e),
            })
            .collect();
        Ok(Sockets(res.chain_err(|| "Couldn't parse Sockets")?))
    }
}

impl Sockets {
    fn socket_amount(&self) -> usize {
        self.0.iter().count()
    }

    fn max_link_count(&self) -> usize {
        self.0
            .iter()
            .fold(
                (0, 0, 0),
                |(curr_group, curr_count, max_count), &(_, grp)| if grp == curr_group {
                    if curr_count == max_count {
                        (curr_group, curr_count + 1, max_count + 1)
                    } else {
                        (curr_group, curr_count + 1, max_count)
                    }
                } else {
                    (grp, 1, max_count)
                },
            )
            .2
    }
}


#[cfg(test)]
mod tests {
    use super::Sockets;
    use super::SocketColour;
    use super::FrameType;
    use super::StashType;
    use std::convert::TryInto;
    use errors::*;


    fn examplesockets() -> Sockets {
        use super::SocketColour::*;
        Sockets(vec![
            (Green, 0),
            (White, 0),
            (Red, 1),
            (Blue, 1),
            (Green, 1),
            (White, 2),
        ])
    }

    #[test]
    fn socket_links() {
        assert_eq!(examplesockets().max_link_count(), 3);
    }

    #[test]
    fn socket_amount() {
        assert_eq!(examplesockets().socket_amount(), 6);
    }

    #[test]
    fn into_sockets() {
        let soc: Result<Sockets> = vec![(0, "F".to_owned())].try_into();
        assert!(soc.is_err());
    }

    #[test]
    fn into_frametype() {
        let f: Result<FrameType> = 0u8.try_into();
        assert!(match f {
            Ok(ft) => ft == FrameType::Normal,
            _ => false,
        });

        let f: Result<FrameType> = 1u8.try_into();
        assert!(match f {
            Ok(ft) => ft == FrameType::Magic,
            _ => false,
        });

        let f: Result<FrameType> = 2u8.try_into();
        assert!(match f {
            Ok(ft) => ft == FrameType::Rare,
            _ => false,
        });

        let f: Result<FrameType> = 3u8.try_into();
        assert!(match f {
            Ok(ft) => ft == FrameType::Unique,
            _ => false,
        });

        let f: Result<FrameType> = 22u8.try_into();
        assert!(f.is_err());
    }

    #[test]
    fn into_stashtype() {
        let st: Result<StashType> = "NormalStash".to_owned().try_into();
        assert!(match st {
            Ok(s) => s == StashType::NormalStash,
            _ => false,
        });

        let st: Result<StashType> = "DivinationStash".to_owned().try_into();
        assert!(match st {
            Ok(s) => s == StashType::DivinationStash,
            _ => false,
        });

        let st: Result<StashType> = "hgaskdfjh".to_owned().try_into();
        assert!(st.is_err());
    }

    #[test]
    fn into_socketcolour() {
        let sc: Result<SocketColour> = "D".to_owned().try_into();
        assert!(match sc {
            Ok(c) => c == SocketColour::Green,
            _ => false,
        });


        let sc: Result<SocketColour> = "G".to_owned().try_into();
        assert!(match sc {
            Ok(c) => c == SocketColour::White,
            _ => false,
        });

        let sc: Result<SocketColour> = "S".to_owned().try_into();
        assert!(match sc {
            Ok(c) => c == SocketColour::Red,
            _ => false,
        });

        let sc: Result<SocketColour> = "y".to_owned().try_into();
        assert!(sc.is_err());
    }

    #[test]
    fn into_propertycolour() {
        use super::PropertyColour;

        let pc: Result<PropertyColour> = 0u8.try_into();
        assert!(match pc {
            Ok(p) => p == PropertyColour::White,
            _ => false,
        });

        let pc: Result<PropertyColour> = 18u8.try_into();
        assert!(pc.is_err());
    }

    #[test]
    fn into_requirement() {
        use super::Requirement;
        use poe_json::ApiProperties;

        let p = ApiProperties {
            name: "Int".into(),
            values: vec![("123".into(), 0)],
            display_mode: 1,
            prop_type: None,
            progress: None,
        };

        let req: Result<Requirement> = p.try_into();

        assert!(match req {
            Ok(r) => r == Requirement::Int(123),
            _ => false,
        });

    }


}
