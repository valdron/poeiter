
use poe_json::ApiProperties;
use poe_item_types::CurrencyType;

#[derive(Debug)]
enum ItemSpecifics {
    Currency { c_type: CurrencyType, stack_size: u16}, //make enum
    Gear{quality: u8,}, // everything equippable with sockets
    Jewel{}, // enum for suff/prefix or (Option<Jewelmod>, Option<JewelMod>)
    Flask{quality: u8, f_type: Flasktype}, // Flasktype should be enum for lifeflask with tier
    Misc{},  
    Gem{ quality: u8, level: u8, },
    DivinationCard{ name: String, stack_size: u16},//make enum? or indicate by String?
    Prophecy{name: String,}, //make enum? or indicate by String?
    Map{name: String, tier: u8, quality: u8},
    MapFragments{}, //sacrifice ... (are normal items)
    LeagueStone{ls_type: LeagueStoneType, mods: Vec<LeagueStoneMod>}  //normal items aswell

}


#[derive(Debug, PartialEq)]
enum Requirement {
    Str(u16),
    Dex(u16),
    Int(u16),
    Lvl(u16),
    Unknown,
}

#[derive(Debug, PartialEq)]
enum StashType {
    NormalStash,
    PremiumStash,
    QuadStash,
    EssenceStash,
    CurrencyStash,
    DivinationStash,
    Unknown(String),
}

#[derive(Debug, PartialEq)]
struct Sockets(Vec<(SocketColour, u8)>);

#[derive(Debug, PartialEq)]
enum SocketColour {
    Red,
    Green,
    Blue,
    White,
    Unknown(String),
}

#[derive(Debug, PartialEq)]
enum PropertyColour {
    White,
    Blue,
    Fire,
    Cold,
    Lightning,
    Chaos,
    Unknown(u8),
}

#[derive(Debug, PartialEq)]
enum FrameType {
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
    Unknown(u8),
}

impl From<ApiProperties> for Requirement {
    fn from(prop: ApiProperties) -> Self {
        use self::Requirement::*;
        if let Some(&(ref val, _)) = prop.values.get(0) {

            let value: u16 = match val.parse() {
                Ok(v) => v,
                _ => return Unknown,
            };

            match prop.name.as_str() {
                "Str" => Str(value),
                "Dex" => Dex(value),
                "Int" => Int(value),
                "Level" => Lvl(value),
                _ => Unknown,
            }
        } else {
            Unknown
        }

    }
}

impl From<u8> for FrameType {
    fn from(number: u8) -> Self {
        use self::FrameType::*;
        match number {
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
            _ => Unknown(number),
        }
    }
}

impl From<u8> for PropertyColour {
    fn from(number: u8) -> Self {
        use self::PropertyColour::*;
        match number {
            0 => White,
            1 => Blue,
            2 => Fire,
            3 => Cold,
            4 => Lightning,
            5 => Chaos,
            _ => Unknown(number),
        }
    }
}

impl From<String> for StashType {
    fn from(s: String) -> Self {
        use self::StashType::*;
        match s.as_str() {
            "NormalStash" => NormalStash,
            "PremiumStash" => PremiumStash,
            "QuadStash" => QuadStash,
            "EssenceStash" => EssenceStash,
            "CurrencyStash" => CurrencyStash,
            "DivinationStash" => DivinationStash,
            _ => Unknown(s),
        }
    }
}

impl From<String> for SocketColour {
    fn from(s: String) -> Self {
        use self::SocketColour::*;
        match s.as_ref() {
            "D" => Green,
            "S" => Red,
            "I" => Blue,
            "G" => White,
            _ => Unknown(s),
        }
    }
}

impl From<Vec<(u8, String)>> for Sockets {
    fn from(v: Vec<(u8, String)>) -> Self {
        Sockets(v.into_iter().map(|(grp, col)| (col.into(), grp)).collect())
    }
}

impl Sockets {
    fn socket_amount(&self) -> usize {
        self.0.iter().count()
    }

    fn max_link_count(&self) -> usize {
        self.0
            .iter()
            .fold((0, 0, 0),
                  |(curr_group, curr_count, max_count), &(_, grp)| if grp == curr_group {
                      if curr_count == max_count {
                          (curr_group, curr_count + 1, max_count + 1)
                      } else {
                          (curr_group, curr_count + 1, max_count)
                      }
                  } else {
                      (grp, 1, max_count)
                  })
            .2
    }
}


#[cfg(test)]
mod tests {
    use super::Sockets;
    use super::SocketColour;
    use super::FrameType;
    use super::StashType;


    fn examplesockets() -> Sockets {
        use super::SocketColour::*;
        Sockets(vec![(Green, 0), (White, 0), (Red, 1), (Blue, 1), (Green, 1), (White, 2)])
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
    fn into_frametype() {
        let f: FrameType = 0u8.into();
        assert_eq!(f, FrameType::Normal);

        let f: FrameType = 1u8.into();
        assert_eq!(f, FrameType::Magic);

        let f: FrameType = 2u8.into();
        assert_eq!(f, FrameType::Rare);

        let f: FrameType = 3u8.into();
        assert_eq!(f, FrameType::Unique);

        let f: FrameType = 22u8.into();

        assert!(if let FrameType::Unknown(_) = f {
                    true
                } else {
                    false
                });
    }

    #[test]
    fn into_stashtype() {
        let st: StashType = "NormalStash".to_owned().into();
        assert_eq!(st, StashType::NormalStash);

        let st: StashType = "DivinationStash".to_owned().into();
        assert_eq!(st, StashType::DivinationStash);

        let st: StashType = "hgaskdfjh".to_owned().into();
        assert!(if let StashType::Unknown(_) = st {
                    true
                } else {
                    false
                });
    }

    #[test]
    fn into_socketcolour() {
        let sc: SocketColour = "D".to_owned().into();
        assert_eq!(sc, SocketColour::Green);

        let sc: SocketColour = "G".to_owned().into();
        assert_eq!(sc, SocketColour::White);

        let sc: SocketColour = "S".to_owned().into();
        assert_eq!(sc, SocketColour::Red);

        let sc: SocketColour = "y".to_owned().into();
        assert!(if let SocketColour::Unknown(_) = sc {
                    true
                } else {
                    false
                });
    }

    #[test]
    fn into_propertycolour() {
        use super::PropertyColour;

        let pc: PropertyColour = 0u8.into();
        assert_eq!(pc, PropertyColour::White);

        let pc: PropertyColour = 18u8.into();
        assert!(if let PropertyColour::Unknown(_) = pc {
                    true
                } else {
                    false
                });
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
            progress: None
        };

        let req: Requirement = p.into();

        assert_eq!(req, Requirement::Int(123));


    }


}
