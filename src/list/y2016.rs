use std::collections::HashSet;

use super::y2015;
pub fn n100() -> HashSet<String> {
    let mut y2015_n100: HashSet<String> = y2015::n100();

    y2015_n100.remove("KLAC");
    y2015_n100.insert("CSX".to_owned());

    y2015_n100.remove("SNDK");
    y2015_n100.insert("NTES".to_owned());

    y2015_n100.remove("LMCA");
    y2015_n100.remove("LMCK");
    y2015_n100.remove("BATRA");
    y2015_n100.remove("BATRK");
    y2015_n100.insert("XRAY".to_owned());

    y2015_n100.remove("ENDP");
    y2015_n100.insert("MCHP".to_owned());

    y2015_n100.remove("LLTC");
    y2015_n100.insert("SHPG".to_owned());

    y2015_n100.insert("CTAS".to_owned());
    y2015_n100.insert("HAS".to_owned());
    y2015_n100.insert("HOLX".to_owned());
    y2015_n100.insert("KLAC".to_owned());

    y2015_n100.remove("BBBY");
    y2015_n100.remove("NTAP");
    y2015_n100.remove("SRCL");
    y2015_n100.remove("WFM");

    y2015_n100
}
