use std::collections::HashSet;

pub fn n100() -> HashSet<String> {
    vec![
        "ATVI", "ADBE", "AMD", "ALXN", "ALGN", "GOOGL", "GOOG", "AMZN", "AEP", "AMGN", "ADI",
        "ANSS", "AAPL", "AMAT", "ASML", "TEAM", "ADSK", "ADP", "BIDU", "BIIB", "BKNG", "AVGO",
        "CDNS", "CDW", "CERN", "CHTR", "CHKP", "CTAS", "CSCO", "CTSH", "CMCSA", "CPRT", "COST",
        "CSX", "DXCM", "DOCU", "DLTR", "EBAY", "EA", "EXC", "FB", "FAST", "FISV", "FOXA", "FOX",
        "GILD", "IDXX", "ILMN", "INCY", "INTC", "INTU", "ISRG", "JD", "KDP", "KLAC", "KHC", "LRCX",
        "LULU", "MAR", "MRVL", "MTCH", "MXIM", "MELI", "MCHP", "MU", "MSFT", "MRNA", "MDLZ",
        "MNST", "NTES", "NFLX", "NVDA", "NXPI", "ORLY", "OKTA", "PCAR", "PAYX", "PYPL", "PTON",
        "PEP", "PDD", "QCOM", "REGN", "ROST", "SGEN", "SIRI", "SWKS", "SPLK", "SBUX", "SNPS",
        "TMUS", "TSLA", "TXN", "TCOM", "VRSN", "VRSK", "VRTX", "WBA", "WDAY", "XEL", "XLNX", "ZM",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}
