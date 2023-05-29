use std::{
    env,
    mem::take,
    sync::{Arc, Mutex},
};

use dcscrape::data_list::{ArticleList, CommentInfo};

use futures::Future;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use scraper::{Html, Selector};
use serde_json::json;

use crate::constants::*;

fn parse_selector(a: &str) -> Selector {
    scraper::Selector::parse(a).unwrap()
}

pub async fn get_article() -> Vec<ArticleList> {
    let args: Vec<String> = env::args().collect();
    let gallname = args.get(1).unwrap().clone();
    let articles = get_article_list(gallname, 1).await;
    let article_mutex: Arc<Mutex<Vec<ArticleList>>> = Arc::new(Mutex::new(articles));
    let futs = article_load(&article_mutex);

    futures::future::join_all(futs).await;
    //println!("{:?}", article_mutex.lock().unwrap());
    let mut v = article_mutex.lock().unwrap();
    let v_mem = take(&mut *v);
    v_mem
}

pub async fn get_article_list(gallname: String, page: u32) -> Vec<ArticleList> {
    let mut headers = HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (iPhone; CPU iPhone OS 14_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1 Mobile/15E148 Safari/604.1"));
    let client = reqwest::Client::builder().user_agent(UA).build().unwrap();
    let response = client
        .get(format!(
            "https://m.dcinside.com/board/{}?page={page}",
            gallname.clone()
        ))
        .send()
        .await
        .unwrap();
    let text = response.text().await.unwrap();
    let document = Html::parse_document(&text);
    let mobile_selector = parse_selector(ROOT_SCRAPE);
    let url_selector = parse_selector(URL_SCRAPE);
    let v_url: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let mut articles: Vec<ArticleList> = Vec::new();
    for title in document.select(&mobile_selector) {
        let a = Html::parse_document(&title.inner_html());
        let url: String = a
            .select(&url_selector)
            .map(|x| x.value().attr("href").unwrap().chars().collect::<String>())
            .collect();
        if url != "" {
            let (_, r) = url.rsplit_once("/").unwrap();
            let id = r
                .replace(format!("?page={}", page).as_str(), "")
                .parse::<u32>()
                .unwrap();
            v_url
                .lock()
                .unwrap()
                .push(format!("https://m.dcinside.com/board/{}/{}", gallname, id));
            articles.push(ArticleList::new(gallname.clone(), id));
        }
    }
    articles
}

pub fn article_load(mutex: &Arc<Mutex<Vec<ArticleList>>>) -> Vec<impl Future<Output = ()>> {
    let mut futs = vec![];
    let article_len = mutex.lock().unwrap().len();
    for i in 0..article_len {
        let url_counter = Arc::clone(&mutex);
        let (id, gallname) = url_counter
            .lock()
            .unwrap()
            .get(i)
            .unwrap()
            .get_article_num();

        let url = format!("https://m.dcinside.com/board/{}/{}", gallname, id);
        let title_selector = parse_selector(ARTICLE_ROOT_SCRAPER);
        let article_write_selector = parse_selector(ARTICLE_WRITE_SCRAPER);
        let article_gonik_selector = parse_selector(ARTICLE_GONIC_CHECK_SCRAPER);
        let article_body_selector = parse_selector(ARTICLE_BODY_SCRAPER);
        let article_view_selector = parse_selector(ARTICLE_VIEW_SCRAPER);
        let article_recommend_cnt_selector = parse_selector(ARTICLE_RECOMMEND_SCRAPER);
        let article_comment_cnt_selector = parse_selector(ARTICLE_COMMENT_CNT_SCRAPER);
        futs.push(async move {
            let response = reqwest::Client::builder()
                .user_agent(UA)
                .build()
                .unwrap()
                .get(url)
                .send()
                .await
                .unwrap();
            //println!("{i}");
            //println!("{:?}", response.text().await.unwrap());
            let text = response.text().await.unwrap();
            let document = Html::parse_document(&text);
            //let title_selector = scraper::Selector::parse("#container > section.left_content > article:nth-child(n) > div.gall_listwrap.list > table > tbody > tr:nth-child(n").unwrap();
            let title: String = document
                .select(&title_selector)
                .map(|x| {
                    x.text()
                        .collect::<String>()
                        .replace("\n", "")
                        .replace("\t", "")
                })
                .collect();
            let write_info: Vec<String> = document
                .select(&article_write_selector)
                .map(|x| x.text().collect::<String>())
                .collect();
            let rgx = Regex::new("<script\\b[^>]*>([\\s\\S]*?)</script>").unwrap();
            let body: String = document
                .select(&article_body_selector)
                .enumerate()
                .map(|(_, r)| {
                    rgx.replace_all(&Html::parse_document(&r.html()).html(), "")
                        .to_string()
                        .replace("\t", "")
                        .replace("\n", "")
                })
                .collect();
            let is_gonik: String = document
                .select(&article_gonik_selector)
                .map(|x| {
                    let a = x.value().attr("href");
                    match a {
                        Some(a) => a.chars().collect::<String>().to_string(),
                        None => String::new(),
                    }
                })
                .collect();
            let view: String = document
                .select(&article_view_selector)
                .map(|x| x.text().collect::<String>())
                .collect();
            let recommend: String = document
                .select(&article_recommend_cnt_selector)
                .map(|x| x.text().collect::<String>())
                .collect();
            let comment_cnt: String = document
                .select(&article_comment_cnt_selector)
                .map(|x| x.text().collect::<String>())
                .collect();

            let (id, gallname) = url_counter
                .lock()
                .unwrap()
                .get(i)
                .unwrap()
                .get_article_num();
            let comments = get_comments(gallname, id.to_string(), 1).await;
            let writer = write_info.get(0).unwrap().to_string();
            let time = write_info.get(1).unwrap().to_string();
            //body = re.replace(body.as_str(), "").to_string();
            url_counter.lock().unwrap()[i].update_list(
                body,
                title,
                writer,
                time,
                is_gonik,
                recommend.parse::<u32>().unwrap(),
                comment_cnt.parse::<u32>().unwrap(),
                comments,
                view.replace("조회수 ", "").parse::<u32>().unwrap(),
            )
        });
    }
    futs
}

pub async fn get_comments(gallname: String, id: String, c_page: u32) -> Vec<CommentInfo> {
    let mut comment_v: Vec<CommentInfo> = Vec::new();
    let mut headers = HeaderMap::new();
    let payload = json!({
        "id": gallname,
        "no": id,
        "cpage": c_page,
    });
    //    let mut multipart = Multipart::
    headers.insert(reqwest::header::USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (iPhone; CPU iPhone OS 14_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1 Mobile/15E148 Safari/604.1"));
    headers.insert(
        "Accept-Encoding",
        HeaderValue::from_static("Accept-Encoding: gzip, deflate, br"),
    );
    headers.insert(
        "Accept-Language",
        HeaderValue::from_static("ko-KR,ko;q=0.9,en-US;q=0.8,en;q=0.7"),
    );
    headers.insert("Accept", HeaderValue::from_static("*/*"));
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"),
    );
    headers.insert("Host", HeaderValue::from_static("m.dcinside.com"));
    headers.insert("Origin", HeaderValue::from_static("https://m.dcinside.com"));
    headers.insert(
        "Referer",
        HeaderValue::from_static("https://m.dcinside.com"),
    );
    let client = reqwest::Client::builder().user_agent(UA).build().unwrap();

    let response = client
        .post(COMMENT_REQUEST_URL)
        .json(&payload)
        .send()
        .await
        .expect("msg");
    let reply_html = Html::parse_document(response.text().await.unwrap().as_str());
    let comment_scraper = scraper::Selector::parse(COMMENT_SCRAPER).unwrap();
    let write_info = reply_html.select(&comment_scraper);

    for c in write_info {
        let writer: String = c
            .select(&scraper::Selector::parse("a.nick").unwrap())
            .map(|x| x.text().collect::<String>())
            .collect();
        let body: String = c
            .select(&scraper::Selector::parse("p").unwrap())
            .map(|x| x.text().collect::<String>())
            .collect();
        let date: String = c
            .select(&scraper::Selector::parse("span.date").unwrap())
            .map(|x| x.text().collect::<String>())
            .collect();
        let mut comment_type: String = c.value().attr("class").unwrap().chars().collect::<String>();
        if !comment_type.contains("add") {
            comment_type = String::from("comment");
        } else {
            comment_type = String::from("reply");
        }
        let tmp_writer_info: String = c
            .select(&scraper::Selector::parse("a").unwrap())
            .enumerate()
            .map(|(_, r)| r.value().attr("href").unwrap().chars().collect::<String>())
            .collect();
        let mut writer_info = tmp_writer_info.clone();
        if tmp_writer_info.starts_with("javascript:") {
            writer_info = String::from("");
        }
        comment_v.push(CommentInfo::new(
            writer,
            writer_info,
            body,
            date,
            comment_type,
        ))
    }
    comment_v
}
