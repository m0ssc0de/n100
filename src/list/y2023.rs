use std::collections::HashSet;

pub fn n100() -> HashSet<String> {
    vec![
        "ADBE", "ADP", "ABNB", "GOOGL", "GOOG", "AMZN", "AMD", "AEP", "AMGN", "ADI", "ANSS",
        "AAPL", "AMAT", "ASML", "AZN", "TEAM", "ADSK", "BKR", "BIIB", "BKNG", "AVGO", "CDNS",
        "CDW", "CHTR", "CTAS", "CSCO", "CCEP", "CTSH", "CMCSA", "CEG", "CPRT", "CSGP", "COST",
        "CRWD", "CSX", "DDOG", "DXCM", "FANG", "DLTR", "DASH", "EA", "EXC", "FAST", "FTNT", "GEHC",
        "GILD", "GFS", "HON", "IDXX", "ILMN", "INTC", "INTU", "ISRG", "KDP", "KLAC", "KHC", "LRCX",
        "LULU", "MAR", "MRVL", "MELI", "META", "MCHP", "MU", "MSFT", "MRNA", "MDLZ", "MDB", "MNST",
        "NFLX", "NVDA", "NXPI", "ORLY", "ODFL", "ON", "PCAR", "PANW", "PAYX", "PYPL", "PDD", "PEP",
        "QCOM", "REGN", "ROP", "ROST", "SIRI", "SPLK", "SBUX", "SNPS", "TTWO", "TMUS", "TSLA",
        "TXN", "TTD", "VRSK", "VRTX", "WBA", "WBD", "WDAY", "XEL", "ZS",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}
