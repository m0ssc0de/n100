use std::collections::HashSet;

pub fn n100() -> HashSet<String> {
    vec![
        "AAPL", "AMAT", "AMGN", "CMCSA", "INTC", "KLAC", "PCAR", "CTAS", "PAYX", "LRCX", "ADSK",
        "ROST", "MNST", "MSFT", "ADBE", "FAST", "EA", "CSCO", "REGN", "IDXX", "VRTX", "BIIB",
        "ODFL", "QCOM", "GILD", "SNPS", "SBUX", "INTU", "MCHP", "ORLY", "COST", "CPRT", "ASML",
        "ANSS", "TTWO", "AMZN", "MSTR", "CTSH", "CSGP", "NVDA", "BKNG", "ON", "ISRG", "MRVL",
        "ADI", "AEP", "AMD", "ADP", "AZN", "CDNS", "CSX", "HON", "MAR", "MU", "XEL", "EXC", "PEP",
        "ROP", "TXN", "AXON", "MDLZ", "NFLX", "GOOGL", "DXCM", "TMUS", "LULU", "MELI", "KDP",
        "AVGO", "VRSK", "FTNT", "CHTR", "TSLA", "NXPI", "FANG", "META", "PANW", "WDAY", "CDW",
        "GOOG", "PYPL", "KHC", "TEAM", "CCEP", "TTD", "BKR", "MDB", "ZS", "PDD", "CRWD", "DDOG",
        "PLTR", "ABNB", "DASH", "APP", "GFS", "CEG", "WBD", "GEHC", "LIN", "ARM",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}
