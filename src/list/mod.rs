use std::collections::{HashMap, HashSet};

mod y2015;
mod y2016;
mod y2017;
mod y2018;
mod y2019;
mod y2020;
mod y2021;
mod y2022;
mod y2023;
mod y2024;
mod ylatest;

pub fn n100() -> HashMap<String, HashSet<String>> {
    let mut n100_map = HashMap::new();
    n100_map.insert("2015".to_owned(), y2015::n100());
    n100_map.insert("2016".to_owned(), y2016::n100());
    n100_map.insert("2017".to_owned(), y2017::n100());
    n100_map.insert("2018".to_owned(), y2018::n100());
    n100_map.insert("2019".to_owned(), y2019::n100());
    n100_map.insert("2020".to_owned(), y2020::n100());
    n100_map.insert("2021".to_owned(), y2021::n100());
    n100_map.insert("2022".to_owned(), y2022::n100());
    n100_map.insert("2023".to_owned(), y2023::n100());
    n100_map.insert("2024".to_owned(), y2024::n100());
    n100_map.insert("latest".to_owned(), ylatest::n100());

    n100_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n100() {
        let n100_map = n100();
        for (year, n100) in n100_map.iter() {
            println!("{}: {:?}", year, n100);
        }
        assert_eq!(n100_map.len(), 4);
        assert_eq!(n100_map.get("2015").unwrap().len(), 100);
        assert_eq!(n100_map.get("2016").unwrap().len(), 100);
        assert_eq!(n100_map.get("2017").unwrap().len(), 100);
        assert_eq!(n100_map.get("2018").unwrap().len(), 100);
    }
}
