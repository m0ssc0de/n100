use std::collections::HashSet;

pub fn n100() -> HashSet<String> {
    // https://en.wikipedia.org/w/index.php?title=Nasdaq-100&oldid=694924978
    vec![
        "FOXA", "FOX", "ATVI", "ADBE", "AKAM", "ALXN", "GOOGL", "GOOG", "AMZN", "AAL", "AMGN",
        "ADI", "AAPL", "AMAT", "ADSK", "ADP", "AVGO", "BIDU", "BBBY", "BIIB", "BMRN", "CHRW", "CA",
        "CELG", "CERN", "CHTR", "CHKP", "CSCO", "CTXS", "CTSH", "CMCSK", "CMCSA", "COST", "DISCA",
        "DISCK", "DISH", "DLTR", "EBAY", "EA", "EXPD", "ESRX", "FB", "FAST", "FISV", "GRMN",
        "GILD", "HSIC", "ILMN", "INCY", "INTC", "INTU", "ISRG", "JD", "KLAC", "GMCR", "KHC",
        "LRCX", "LBTYA", "LBTYK", "QVCA", "LILA", "LILAK", "LMCK", "LMCA", "LVNTA", "LLTC", "MAR",
        "MAT", "MU", "MSFT", "MDLZ", "MNST", "MYL", "NTAP", "NFLX", "NVDA", "NXPI", "ORLY", "PCAR",
        "PYPL", "PAYX", "QCOM", "REGN", "ROST", "SNDK", "SBAC", "STX", "SIRI", "SWKS", "SPLS",
        "SBUX", "SRCL", "SYMC", "TSLA", "TXN", "PCLN", "TSCO", "TRIP", "VRSK", "VRTX", "VIAB",
        "VIP", "VOD", "WBA", "WDC", "WFM", "WYNN", "XLNX", "YHOO",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}
