pub fn try_find_key<'a>(kvps: &Vec<(&str, &'a str)>, needle: &str) -> Option<&'a str> {
    kvps.iter()
        .find_map(|(key, value)| if *needle == **key { Some(*value) } else { None })
}

pub mod part2 {
    use super::*;
    use std::usize;

    pub enum BirthYear {
        BirthYear(usize),
    }
    impl BirthYear {
        pub fn new(data: &str) -> Option<BirthYear> {
            let as_int = data.parse::<usize>().ok()?;

            if as_int >= 1920 && as_int <= 2002 {
                Some(BirthYear::BirthYear(as_int))
            } else {
                None
            }
        }
    }

    pub enum IssueYear {
        IssueYear(usize),
    }

    impl IssueYear {
        pub fn new(data: &str) -> Option<Self> {
            let as_int = data.parse::<usize>().ok()?;

            if as_int >= 2010 && as_int <= 2020 {
                Some(IssueYear::IssueYear(as_int))
            } else {
                None
            }
        }
    }

    pub enum ExpirationYear {
        ExpirationYear(usize),
    }

    impl ExpirationYear {
        pub fn new(data: &str) -> Option<ExpirationYear> {
            let as_int = data.parse::<usize>().ok()?;

            if as_int >= 2020 && as_int <= 2030 {
                Some(ExpirationYear::ExpirationYear(as_int))
            } else {
                None
            }
        }
    }

    pub enum Height {
        Centimeters(usize),
        Inches(usize),
    }

    impl Height {
        pub fn new(data: &str) -> Option<Height> {
            if data.ends_with("cm") {
                let value = data.strip_suffix("cm")?.parse::<usize>().ok()?;
                if value >= 150 && value <= 193 {
                    return Some(Height::Centimeters(value));
                }
            } else if data.ends_with("in") {
                let value = data.strip_suffix("in")?.parse::<usize>().ok()?;
                if value >= 59 && value <= 76 {
                    return Some(Height::Inches(value));
                }
            }
            None
        }
    }

    pub enum HairColor {
        Color(String),
    }

    impl HairColor {
        pub fn new(data: &str) -> Option<HairColor> {
            if data.len() == 7
                && data.starts_with('#')
                && data[1..].chars().all(|c| c.is_ascii_hexdigit())
            {
                Some(HairColor::Color(data.to_string()))
            } else {
                None
            }
        }
    }

    pub enum EyeColor {
        Amber,
        Blue,
        Brown,
        Gray,
        Green,
        Hazel,
        Other,
    }

    impl EyeColor {
        pub fn new(data: &str) -> Option<EyeColor> {
            match data {
                "amb" => Some(EyeColor::Amber),
                "blu" => Some(EyeColor::Blue),
                "brn" => Some(EyeColor::Brown),
                "gry" => Some(EyeColor::Gray),
                "grn" => Some(EyeColor::Green),
                "hzl" => Some(EyeColor::Hazel),
                "oth" => Some(EyeColor::Other),
                _ => None,
            }
        }
    }

    pub enum PassportId {
        Id(String),
    }

    impl PassportId {
        pub fn new(data: &str) -> Option<PassportId> {
            if data.len() == 9 && data.chars().all(|c| c.is_ascii_digit()) {
                Some(PassportId::Id(data.to_string()))
            } else {
                None
            }
        }
    }

    pub struct Document {
        birth_year: BirthYear,
        issue_year: IssueYear,
        expiration_year: ExpirationYear,
        height: Height,
        hair_color: HairColor,
        eye_color: EyeColor,
        passport_id: PassportId,
        country_id: Option<String>,
    }

    impl Document {
        pub fn new(data: &str) -> Option<Document> {
            let kvps: Vec<(&str, &str)> = data
                .split_ascii_whitespace()
                .map(|substring: &str| substring.split_once(':').expect("Missing colon"))
                .collect();

            let birth_year = BirthYear::new(try_find_key(&kvps, "byr")?)?;
            let issue_year = IssueYear::new(try_find_key(&kvps, "iyr")?)?;
            let expiration_year = ExpirationYear::new(try_find_key(&kvps, "eyr")?)?;
            let height = Height::new(try_find_key(&kvps, "hgt")?)?;
            let hair_color = HairColor::new(try_find_key(&kvps, "hcl")?)?;
            let eye_color = EyeColor::new(try_find_key(&kvps, "ecl")?)?;
            let passport_id = PassportId::new(try_find_key(&kvps, "pid")?)?;
            let country_id = try_find_key(&kvps, "cid").map(String::from);

            Some(Document {
                birth_year,
                issue_year,
                expiration_year,
                height,
                hair_color,
                eye_color,
                passport_id,
                country_id,
            })
        }
    }

    pub fn solve(data: &Vec<&str>) {
        let valid_passports = data.iter().filter_map(|s| Document::new(s)).count();

        println!("There are {valid_passports} strictly valid passports")
    }
}

pub mod part1 {
    use super::try_find_key;

    #[derive(Debug)]
    pub struct Document<'a> {
        birth_year: Option<&'a str>,
        issue_year: Option<&'a str>,
        expiration_year: Option<&'a str>,
        height: Option<&'a str>,
        hair_color: Option<&'a str>,
        eye_color: Option<&'a str>,
        passport_id: Option<&'a str>,
        country_id: Option<&'a str>,
    }

    pub fn solve(data: &Vec<&str>) {
        let valid_passports = data
            .iter()
            .map(|s| Document::new(*s))
            .filter(|doc| Document::is_valid_passport(doc))
            .count();

        println!("There are {valid_passports} valid passports")
    }

    impl<'a> Document<'a> {
        pub fn is_valid_passport(&self) -> bool {
            self.birth_year.is_some()
                && self.birth_year.is_some()
                && self.issue_year.is_some()
                && self.expiration_year.is_some()
                && self.height.is_some()
                && self.hair_color.is_some()
                && self.eye_color.is_some()
                && self.passport_id.is_some()
        }

        pub fn new<'b: 'a>(data: &'b str) -> Self {
            let kvps: Vec<(&str, &str)> = data
                .split_ascii_whitespace()
                .map(|substring: &str| substring.split_once(':').expect("Missing colon"))
                .collect();

            let birth_year = try_find_key(&kvps, "byr");
            let issue_year = try_find_key(&kvps, "iyr");
            let expiration_year = try_find_key(&kvps, "eyr");
            let height = try_find_key(&kvps, "hgt");
            let hair_color = try_find_key(&kvps, "hcl");
            let eye_color = try_find_key(&kvps, "ecl");
            let passport_id = try_find_key(&kvps, "pid");
            let country_id = try_find_key(&kvps, "cid");

            Document {
                birth_year,
                issue_year,
                expiration_year,
                height,
                hair_color,
                eye_color,
                passport_id,
                country_id,
            }
        }
    }
}
