

#[derive(Debug)]
enum StashType {
    NormalStash,
    PremiumStash,
    QuadStash,
    EssenceStash,
    CurrencyStash,
    DivinationStash,
    Other(String),
}

#[derive(Debug)]
struct Sockets(Vec<(SocketColour, u8)>);

#[derive(Debug)]
enum SocketColour {
    Red,
    Green,
    Blue,
    White,
    Unknown(String),
}

#[derive(Debug)]
enum PropertyColour {
    White,
    Blue,
    Fire,
    Cold,
    Lightning,
    Chaos,
    Unknown(u8),
}

#[derive(Debug)]
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
            _ => Other(s),
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
    use super::SocketColour::*;

 
    fn examplesockets() -> Sockets {
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


}