mod constants;
mod parse_article;
use crate::parse_article::*;
use chrono;
use dcscrape::data_list::ArticleList;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let list = get_article().await;
    save_article_list(&list).await?;
    //let args = Cli::parse();
    //let contents = std::fs::read_to_string(&args.path)?;
    //write_post().await;
    Ok(())
}

async fn save_article_list(
    v_article_list: &Vec<ArticleList>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (_, gallname) = v_article_list[0].get_article_num();
    let now = chrono::offset::Local::now()
        .format("%y-%m-%d %X")
        .to_string();
    std::fs::create_dir_all(format!("data/{}/{}", gallname.clone(), now))?;

    for i in 0..v_article_list.len() {
        //let a_str_1 = format!("{:?}", v_article_list[i]).to_string();
        let strs = v_article_list[i].sort_article();
        let (id, _) = v_article_list[i].get_article_num();
        let cur = format!("data/{}/{}/{}.txt", gallname, now, id);
        let mut file = std::fs::File::create(cur)?;
        file.write_all(strs.clone().as_bytes()).unwrap();
    }
    Ok(())
}
