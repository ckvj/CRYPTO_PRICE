use enum_iterator::{all, Sequence};

#[derive(Debug, Clone, Sequence, Copy, PartialEq)]
pub enum Asset {
    Bitcoin,
    Ethereum,
    Solana,
}

impl Asset {
    pub fn get_enum(int_input: usize) -> Option<Asset> {
        let assets: Vec<Asset> = all::<Asset>().collect::<Vec<Asset>>();
        assets.get(int_input - 1).cloned() // Subtract one to get index (values started at 1)
    }

    pub fn display_enum_options() {
        for (count, asset) in all::<Asset>().collect::<Vec<Asset>>().iter().enumerate() {
            println!("{}) {:?}", count + 1, asset); // Display values starting at 1)
        }
        println!("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enum_iterator::{cardinality, first, last};

    #[test]
    fn test_get_enum_first() {
        let input = 1;
        assert_eq!(Asset::get_enum(input), first::<Asset>());
    }

    #[test]
    fn test_get_enum_last() {
        let input = cardinality::<Asset>();
        assert_eq!(Asset::get_enum(input), last::<Asset>());
    }

    #[test]
    fn test_get_enum_out_of_index() {
        let input = cardinality::<Asset>() + 1;
        assert_eq!(Asset::get_enum(input), None);
    }
}
