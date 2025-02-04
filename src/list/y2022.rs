use std::collections::HashSet;

pub fn n100() -> HashSet<String> {
    vec![
        "ATVI", "ADBE", "ADP", "ABNB", "ALGN", "GOOGL", "GOOG", "AMZN", "AMD", "AEP", "AMGN",
        "ADI", "ANSS", "AAPL", "AMAT", "ASML", "AZN", "TEAM", "ADSK", "BKR", "BIIB", "BKNG",
        "AVGO", "CDNS", "CHTR", "CTAS", "CSCO", "CTSH", "CMCSA", "CEG", "CPRT", "CSGP", "COST",
        "CRWD", "CSX", "DDOG", "DXCM", "FANG", "DLTR", "EBAY", "EA", "ENPH", "EXC", "FAST", "FISV",
        "FTNT", "GILD", "GFS", "HON", "IDXX", "ILMN", "INTC", "INTU", "ISRG", "JD", "KDP", "KLAC",
        "KHC", "LRCX", "LCID", "LULU", "MAR", "MRVL", "MELI", "META", "MCHP", "MU", "MSFT", "MRNA",
        "MDLZ", "MNST", "NFLX", "NVDA", "NXPI", "ORLY", "ODFL", "PCAR", "PANW", "PAYX", "PYPL",
        "PEP", "PDD", "QCOM", "REGN", "RIVN", "ROST", "SGEN", "SIRI", "SBUX", "SNPS", "TMUS",
        "TSLA", "TXN", "VRSK", "VRTX", "WBA", "WBD", "WDAY", "XEL", "ZM", "ZS",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}
