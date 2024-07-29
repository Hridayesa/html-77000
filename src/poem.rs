use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Poem {
    pub nn: u32,
    pub nn_str: String,
    pub en: Vec<String>,
    pub ru: Vec<String>,
}

impl Poem {
    pub fn new(nn: u32, en: Vec<String>, ru: Vec<String>) -> Self {
        Self {
            nn,
            nn_str: Self::nn_str(nn, " "),
            en,
            ru,
        }
    }

    fn nn_str(nn: u32, sep: &str) -> String {
        // let nn = match self {
        //     Poem::En { nn,.. } => nn,
        //     Poem::EnRu { nn, .. } => nn,
        // };
        let res = nn.to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(sep);  // separator
        res
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn get_test_poem(nn: u32) -> Poem {
        Poem::new(
            nn,
            vec![String::from(format!("Qwerty {}-1", nn)),
                 String::from(format!("Qwerty {}-2", nn)),
                 String::from(format!("Qwerty {}-3", nn)),
            ],
            vec![String::from(format!("Йцукен {}-1", nn)),
                 String::from(format!("Йцукен {}-2", nn)),
                 String::from(format!("Йцукен {}-3", nn)),
            ],
        )
    }

    #[ctor::ctor]
    fn init() {
        tracing_subscriber::fmt::init();
    }

    #[test]
    fn test2(){
        let p = get_test_poem(13_234_567);
        println!("Print {}", p.nn_str);
    }
}