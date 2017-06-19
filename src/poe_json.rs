
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ApiSite {
    next_change_id: String,
    pub stashes: Vec<ApiStash>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ApiStash {
    #[serde(rename = "accountName")]
    account_name: Option<String>,
    #[serde(rename = "lastCharacterName")]
    last_character_name: String,
    id: String,
    stash: String,
    #[serde(rename = "stashType")]
    stash_type: String,
    pub items: Vec<ApiItem>,
    public: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ApiItem {
    verified: bool,
    w: u8,
    h: u8,
    pub ilvl: u8,
    icon: String, // make check if is unided what unique it is
    league: String,
    id: String,
    sockets: Vec<ApiSocket>, //done
    name: String, 
    type_line: String,
    identified: bool,
    corrupted: bool,
    locked_to_character: bool,
    note: Option<String>, 
    pub properties: Option<Vec<ApiProperties>>,
    pub requirements: Option<Vec<ApiProperties>>, //done
    explicit_mods: Option<Vec<String>>,
    implicit_mods: Option<Vec<String>>,
    enchant_mods: Option<Vec<String>>,
    crafted_mods: Option<Vec<String>>,
    flavour_text: Option<Vec<String>>,
    frame_type: u8, //done
    x: Option<u8>,
    y: Option<u8>,
    inventory_id: Option<String>, // make enum
    socketed_items: Vec<ApiItem>,
    additional_properties: Option<Vec<ApiProperties>>,
    sec_descr_text: Option<String>,
    descr_text: Option<String>,
    art_filename: Option<String>,
    duplicated: Option<bool>,
    max_stack_size: Option<u16>,
    next_level_requirements: Option<Vec<ApiProperties>>,
    stack_size: Option<u16>,
    talisman_tier: Option<u8>,
    utility_mods: Option<Vec<String>>,
    support: Option<bool>,
    socket: Option<u8>,
    colour: Option<String>,
    cosmetic_mods: Option<Vec<String>>,
    th_race_reward: Option<bool>,
    prophecy_diff_text: Option<String>,
    prophecy_text: Option<String>,
    is_relic: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ApiSocket {
    group: u8,
    attr: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ApiProperties {
    pub name: String,
    pub values: Vec<(String, u8)>,
    pub display_mode: u8,
    #[serde(rename = "type")]
    pub prop_type: Option<u8>,
    pub progress: Option<f32>,
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    const EXAMPLEJSONITEM: &str = r#"{"verified":false,"w":2,"h":4,"ilvl":71,"icon":"http:\/\/web.poecdn.com\/image\/Art\/2DItems\/Weapons\/TwoHandWeapons\/Bows\/SarkhamsReach.png?scale=1&w=2&h=4&v=f333c2e4005ee20a84270731baa5fa6a3","league":"Hardcore","id":"176b5e6f7af0a5bb4b48d7fdafa47501a179f4ea095815a58c82c4b5244b3cdb","sockets":[{"group":0,"attr":"D"}],"name":"<<set:MS>><<set:M>><<set:S>>Roth's Reach","typeLine":"Recurve Bow","identified":true,"corrupted":false,"lockedToCharacter":false,"note":"~price 10 exa","properties":[{"name":"Bow","values":[],"displayMode":0},{"name":"Quality","values":[["+17%",1]],"displayMode":0,"type":6},{"name":"Physical Damage","values":[["20-63",1]],"displayMode":0,"type":9},{"name":"Critical Strike Chance","values":[["6.50%",0]],"displayMode":0,"type":12},{"name":"Attacks per Second","values":[["1.31",1]],"displayMode":0,"type":13}],"requirements":[{"name":"Level","values":[["18",0]],"displayMode":0},{"name":"Dex","values":[["65",0]],"displayMode":1}],"explicitMods":["68% increased Physical Damage","34% increased Elemental Damage with Weapons","5% increased Attack Speed","Skills Chain +1 times","30% increased Projectile Speed"],"flavourText":["\"Exiled to the sea; what a joke. \r","I'm more free than I've ever been.\"\r","- Captain Weylam \"Rot-tooth\" Roth of the Black Crest"],"frameType":3,"x":10,"y":0,"inventoryId":"Stash1","socketedItems":[]}"#;



    #[test]
    fn apiitem_deserialization() {
        assert!(serde_json::from_str::<ApiItem>(EXAMPLEJSONITEM).is_ok())
    }
}
