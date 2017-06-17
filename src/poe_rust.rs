

#[derive(Debug)]
enum StashType {
    NormalStash,
    PremiumStash,
    QuadStash,
    EssenceStash,
    CurrencyStash,
    DivinationStash,
    Other(String)
}


#[derive(Debug)]
enum SocketColour {
    Red,
    Green,
    Blue,
    White
}

#[derive(Debug)]
enum PropertyColour {
    White,
    Blue,
    Fire,
    Cold,
    Lightning,
    Chaos
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
    Relic
}