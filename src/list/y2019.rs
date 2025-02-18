use std::collections::HashSet;

pub fn n100() -> HashSet<String> {
    vec![
        "ATVI", "ADBE", "AMD", "ALXN", "ALGN", "GOOGL", "GOOG", "AMZN", "AAL", "AMGN", "ADI",
        "ANSS", "AAPL", "AMAT", "ASML", "ADSK", "ADP", "BIDU", "BIIB", "BMRN", "BKNG", "AVGO",
        "CDNS", "CDW", "CERN", "CHTR", "CHKP", "CTAS", "CSCO", "CTXS", "CTSH", "CMCSA", "CPRT",
        "CSGP", "COST", "CSX", "DLTR", "EBAY", "EA", "EXC", "EXPE", "FB", "FAST", "FISV", "FOXA",
        "FOX", "GILD", "IDXX", "ILMN", "INCY", "INTC", "INTU", "ISRG", "JD", "KLAC", "KHC", "LRCX",
        "LBTYA", "LBTYK", "LULU", "MAR", "MXIM", "MELI", "MCHP", "MU", "MSFT", "MDLZ", "MNST",
        "NTAP", "NTES", "NFLX", "NVDA", "NXPI", "ORLY", "PCAR", "PAYX", "PYPL", "PEP", "QCOM",
        "REGN", "ROST", "SGEN", "SIRI", "SWKS", "SPLK", "SBUX", "SNPS", "TMUS", "TTWO", "TSLA",
        "TXN", "TCOM", "ULTA", "UAL", "VRSN", "VRSK", "VRTX", "WBA", "WDAY", "WDC", "WLTW", "XEL",
        "XLNX",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}
