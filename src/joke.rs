use ferris_says::say;

pub struct JokeFlags {
    nsfw: bool,
    religious: bool,
    political: bool,
    racist: bool,
    sexist: bool,
    explicit: bool,
}

impl JokeFlags {
    pub fn empty() -> JokeFlags {
        JokeFlags {
            nsfw: false,
            religious: false,
            political: false,
            racist: false,
            sexist: false,
            explicit: false,
        }
    }
    pub fn new(json_data: serde_json::Value) -> JokeFlags {
        let nsfw = json_data
            .get("nsfw")
            .expect("A joke should contain an nsfw flag")
            .as_bool()
            .unwrap();
        let religious = json_data
            .get("religious")
            .expect("A joke should contain a religious flag")
            .as_bool()
            .unwrap();
        let political = json_data
            .get("political")
            .expect("A joke should contain a political flag")
            .as_bool()
            .unwrap();
        let racist = json_data
            .get("racist")
            .expect("A joke should contain a racist flag")
            .as_bool()
            .unwrap();
        let sexist = json_data
            .get("sexist")
            .expect("A joke should contain a sexist flag")
            .as_bool()
            .unwrap();
        let explicit = json_data
            .get("explicit")
            .expect("A joke should contain an explicit flag")
            .as_bool()
            .unwrap();

        JokeFlags {
            nsfw,
            religious,
            political,
            racist,
            sexist,
            explicit,
        }
    }

    pub fn to_string(&self) -> String {
        let start = format!(
            "nsfw: {},\n\treligious: {},\n\tpolitical: {},\n\t",
            self.nsfw, self.religious, self.political
        );
        let end = format!(
            "racist: {},\n\tsexist: {},\n\texplicit: {},\n",
            self.racist, self.sexist, self.explicit
        );

        return start + end.as_str();
    }
}

#[derive(Clone)]
pub enum JokeData {
    Single { joke: String },                     //"single",
    TwoPart { setup: String, delivery: String }, //"twopart"
}

pub struct Joke {
    id: u32,
    category: String,
    data: JokeData,
    flags: JokeFlags,
    safe: bool,
}

impl Joke {
    pub fn empty() -> Joke {
        Joke {
            id: 0,
            category: "Fake".to_string(),
            data: JokeData::Single {
                joke: "My life".to_string(),
            },
            flags: JokeFlags::empty(),
            safe: true,
        }
    }
    pub fn new(json_data: serde_json::Value) -> Joke {
        let id_value = json_data
            .get("id")
            .expect("Joke should have an id")
            .as_i64();
        let id = u32::try_from(id_value.unwrap()).unwrap();
        let category = json_data
            .get("category")
            .expect("Joke should have a category")
            .as_str()
            .unwrap()
            .to_string();
        let joke_type = json_data
            .get("type")
            .expect("Joke must have a type")
            .as_str()
            .unwrap();

        let data = match joke_type.trim() {
            "single" => {
                let joke = json_data
                    .get("joke")
                    .expect("A joke of type 'single' should have a joke field")
                    .as_str()
                    .unwrap()
                    .to_string();

                JokeData::Single { joke }
            }
            "twopart" => {
                let setup = json_data
                    .get("setup")
                    .expect("A joke of type 'twopart' should have a setup field")
                    .as_str()
                    .unwrap()
                    .to_string();
                let delivery = json_data
                    .get("delivery")
                    .expect("A joke of type 'twopart' should have a delivery field")
                    .as_str()
                    .unwrap()
                    .to_string();

                JokeData::TwoPart { setup, delivery }
            }
            _ => {
                println!("There's an hidden type!!! {joke_type}");

                JokeData::Single {
                    joke: "Your life".to_string(),
                }
            }
        };

        let flags_data = json_data
            .get("flags")
            .expect("Joke should have a flags object");
        let flags = JokeFlags::new(flags_data.to_owned());

        let safe = json_data
            .get("safe")
            .expect("A joke should contain a safe flag")
            .as_bool()
            .unwrap();

        Joke {
            id,
            category,
            data,
            flags,
            safe,
        }
    }

    pub fn to_string(&self) -> String {
        let start = format!("id: {},\ncategory: {},\n", self.id, self.category);

        let additional_data = match self.data.clone() {
            JokeData::Single { joke } => format!("type: single,\njoke: {joke},\n"),
            JokeData::TwoPart { setup, delivery } => {
                format!("type: twopart,\nsetup: {setup},\ndelivery: {delivery},\n")
            }
        };

        let end = format!(
            "flags: {{\n\t{}}},\nsafe: {},\n",
            self.flags.to_string(),
            self.safe
        );

        return start + additional_data.as_str() + end.as_str();
    }

    pub fn full_joke(&self) -> String {
        match self.data.clone() {
            JokeData::Single { joke } => joke,
            JokeData::TwoPart { setup, delivery } => setup + "\n" + delivery.as_str(),
        }
    }

    pub fn ferris_delivery(&self) {
        let introduction = format!(
            "Here's a {} joke for you.\n Joke number {}...",
            self.category, self.id
        );
        ferris_say(introduction);
        ferris_say(self.full_joke());
    }
}

fn ferris_say(message: String) {
    let width = message.chars().count();
    let mut writer = std::io::BufWriter::new(std::io::stdout().lock());
    say(&message, width, &mut writer).unwrap();
}
