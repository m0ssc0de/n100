use std::collections::HashSet;

pub fn n100() -> HashSet<String> {
    vec![
        "ADBE", "AMD", "ABNB", "GOOGL", "GOOG", "AMZN", "AEP", "AMGN", "ADI", "ANSS", "AAPL",
        "AMAT", "APP", "ARM", "ASML", "AZN", "TEAM", "ADSK", "ADP", "AXON", "BKR", "BIIB", "BKNG",
        "AVGO", "CDNS", "CDW", "CHTR", "CTAS", "CSCO", "CCEP", "CTSH", "CMCSA", "CEG", "CPRT",
        "CSGP", "COST", "CRWD", "CSX", "DDOG", "DXCM", "FANG", "DASH", "EA", "EXC", "FAST", "FTNT",
        "GEHC", "GILD", "GFS", "HON", "IDXX", "INTC", "INTU", "ISRG", "KDP", "KLAC", "KHC", "LRCX",
        "LIN", "LULU", "MAR", "MRVL", "MELI", "META", "MCHP", "MU", "MSFT", "MSTR", "MDLZ", "MDB",
        "MNST", "NFLX", "NVDA", "NXPI", "ORLY", "ODFL", "ON", "PCAR", "PLTR", "PANW", "PAYX",
        "PYPL", "PDD", "PEP", "QCOM", "REGN", "ROP", "ROST", "SBUX", "SNPS", "TTWO", "TMUS",
        "TSLA", "TXN", "TTD", "VRSK", "VRTX", "WBD", "WDAY", "XEL", "ZS",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}
