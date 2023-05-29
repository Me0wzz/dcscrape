pub const UA: &str = "Mozilla/5.0 (Linux; Android 10; SM-G980F Build/QP1A.190711.020; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/78.0.3904.96 Mobile Safari/537.36";
pub const ROOT_SCRAPE: &str =
    "body > div > div > div > section:nth-child(3) > ul.gall-detail-lst > li:nth-child(n)";
pub const URL_SCRAPE: &str = "div > a.lt";

pub const ARTICLE_ROOT_SCRAPER: &str =
    "body > div.container > div > div > section:nth-child(3) > div.gallview-tit-box > span";
pub const ARTICLE_WRITE_SCRAPER: &str = "body > div.container > div > div > section:nth-child(3) > div.gallview-tit-box > div > ul > li:nth-child(n)";
pub const ARTICLE_GONIC_CHECK_SCRAPER: &str = "body > div.container > div > div > section:nth-child(3) > div.gallview-tit-box > div > div > a";
pub const ARTICLE_BODY_SCRAPER: &str = "body > div.container > div > div > section:nth-child(3) > div.gall-thum-btm > div > div.thum-txt";
pub const ARTICLE_VIEW_SCRAPER: &str =
    "body > div.container > div > div > section:nth-child(3) > div.gall-thum-btm > div > ul > li:nth-child(1)";
pub const ARTICLE_RECOMMEND_SCRAPER: &str = "body > div.container > div > div > section:nth-child(3) > div.gall-thum-btm > div > ul > li:nth-child(2) > span";
pub const ARTICLE_COMMENT_CNT_SCRAPER: &str = "body > div.container > div > div > section:nth-child(3) > div.gall-thum-btm > div > ul > li:nth-child(3) > a > span";
pub const COMMENT_SCRAPER: &str = "body > ul > li";
pub const COMMENT_REQUEST_URL: &str = "https://m.dcinside.com/ajax/response-comment";