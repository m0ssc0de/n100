use std::collections::HashSet;

use super::y2016;

pub fn n100() -> HashSet<String> {
    let mut y2016_n100: HashSet<String> = y2016::n100();

    y2016_n100.remove("NXPI");
    y2016_n100.insert("JBHT".to_owned());
    y2016_n100.remove("SBAC");
    y2016_n100.insert("IDXX".to_owned());

    y2016_n100.remove("TRIP");
    y2016_n100.insert("WYNN".to_owned());

    y2016_n100.remove("YHOO");
    y2016_n100.insert("MELI".to_owned());

    y2016_n100.remove("MAT");
    y2016_n100.insert("ALGN".to_owned());

    y2016_n100.insert("ASML".to_owned());
    y2016_n100.insert("CDNS".to_owned());
    y2016_n100.insert("SNPS".to_owned());
    y2016_n100.insert("TTWO".to_owned());
    y2016_n100.insert("WDAY".to_owned());
    y2016_n100.remove("AKAM");
    y2016_n100.remove("DISCA");
    y2016_n100.remove("DISCK");
    y2016_n100.remove("NCLH");
    y2016_n100.remove("TSCO");
    y2016_n100.remove("VIAB");

    y2016_n100
}
