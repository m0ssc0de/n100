use std::collections::HashSet;

pub fn n100() -> HashSet<String> {
    vec![
        "ATVI", "ADBE", "ALXN", "ALGN", "GOOGL", "GOOG", "AMZN", "AAL", "AMGN", "ADI", "AAPL",
        "AMAT", "ASML", "ADSK", "ADP", "BIDU", "BIIB", "BMRN", "BKNG", "AVGO", "CDNS", "CELG",
        "CERN", "CHTR", "CHKP", "CTAS", "CSCO", "CTXS", "CTSH", "CMCSA", "COST", "CSX", "CTRP",
        "DLTR", "EBAY", "EA", "EXPE", "FB", "FAST", "FISV", "GILD", "HAS", "HSIC", "IDXX", "ILMN",
        "INCY", "INTC", "INTU", "ISRG", "JBHT", "JD", "KLAC", "LRCX", "LBTYA", "LBTYK", "MAR",
        "MXIM", "MELI", "MCHP", "MU", "MSFT", "MDLZ", "MNST", "MYL", "NTES", "NFLX", "NVDA",
        "NXPI", "ORLY", "PCAR", "PAYX", "PYPL", "PEP", "QCOM", "REGN", "ROST", "SIRI", "SWKS",
        "SBUX", "SYMC", "SNPS", "TMUS", "TTWO", "TSLA", "TXN", "KHC", "FOXA", "FOX", "ULTA",
        "VRSK", "VRTX", "WBA", "WDC", "WDAY", "WYNN", "XEL", "XLNX", "AMD", "LULU", "NTAP", "UAL",
        "VRSN", "WLTW",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}
