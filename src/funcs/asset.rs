use enum_iterator::{all, Sequence};

#[derive(Debug, Clone, Sequence, Copy, PartialEq)]
pub enum Asset {
    Bitcoin,
    Ethereum,
    Solana,
    BinanceCoin,
    Sui,
    Aptos,
}

impl Asset {
    pub fn match_enum(int_input: usize) -> Option<Asset> {
        let assets: Vec<Asset> = all::<Asset>().collect::<Vec<Asset>>();
        assets.get(int_input - 1).cloned() // Subtract one to get index (values started at 1)
    }

    pub fn display_enum_options() {
        all::<Asset>()
            .enumerate()
            .for_each(|(count, asset)| println!("{}) {:?}", count + 1, asset));
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use enum_iterator::{cardinality, first, last};

    #[test]
    fn test_get_enum_first() {
        let input = 1;
        assert_eq!(Asset::match_enum(input), first::<Asset>());
    }

    #[test]
    fn test_get_enum_last() {
        let input = cardinality::<Asset>();
        assert_eq!(Asset::match_enum(input), last::<Asset>());
    }

    #[test]
    fn test_get_enum_out_of_index() {
        let input = cardinality::<Asset>() + 1;
        assert_eq!(Asset::match_enum(input), None);
    }
}
