use std::collections::HashSet;

pub fn n100() -> HashSet<String> {
    vec![
        "ATVI", "ADBE", "AMD", "ABNB", "ALGN", "GOOGL", "GOOG", "AMZN", "AEP", "AMGN", "ADI",
        "ANSS", "AAPL", "AMAT", "ASML", "TEAM", "ADSK", "ADP", "BIDU", "BIIB", "BKNG", "AVGO",
        "CDNS", "CHTR", "CTAS", "CSCO", "CTSH", "CMCSA", "CPRT", "COST", "CRWD", "CSX", "DDOG",
        "DXCM", "DOCU", "DLTR", "EBAY", "EA", "EXC", "FAST", "FISV", "FTNT", "GILD", "HON", "IDXX",
        "ILMN", "INTC", "INTU", "ISRG", "JD", "KDP", "KLAC", "KHC", "LRCX", "LCID", "LULU", "MAR",
        "MRVL", "MTCH", "MELI", "FB", "MCHP", "MU", "MSFT", "MRNA", "MDLZ", "MNST", "NTES", "NFLX",
        "NVDA", "NXPI", "ORLY", "OKTA", "PCAR", "PANW", "PAYX", "PYPL", "PTON", "PEP", "PDD",
        "QCOM", "REGN", "ROST", "SGEN", "SIRI", "SWKS", "SPLK", "SBUX", "SNPS", "TMUS", "TSLA",
        "TXN", "VRSN", "VRSK", "VRTX", "WBA", "WDAY", "XEL", "XLNX", "ZM", "ZS",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}
