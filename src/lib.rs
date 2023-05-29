pub mod data_list {
    use regex::Regex;

    #[derive(Debug)]
    pub struct ArticleList {
        gallname: String,    //done
        id: u32,             //done
        title: String,       //done
        time: String,        //done
        writer: String,      //done
        writer_info: String, //done
        view: u32,           //done
        body: String,        //done
        comment_cnt: u32,    //done
        comment: Vec<CommentInfo>,
        recommend: u32, //done
    }
    impl ArticleList {
        pub fn new(gallname: String, id: u32) -> ArticleList {
            ArticleList {
                gallname: gallname,
                id: id,
                title: String::new(),
                time: String::new(),
                writer: String::new(),
                writer_info: String::new(),
                view: 0,
                body: String::new(),
                comment_cnt: 0,
                comment: Vec::new(),
                recommend: 0,
            }
        }
        pub fn get_article_num(&self) -> (u32, String) {
            (self.id, self.gallname.clone())
        }
        pub fn update_list(
            &mut self,
            body: String,
            title: String,
            writer: String,
            time: String,
            writer_info: String,
            recommend: u32,
            comment_cnt: u32,
            comment: Vec<CommentInfo>,
            view: u32,
        ) {
            self.body = body;
            self.title = title;
            self.writer = writer;
            self.time = time;
            self.writer_info = writer_info;
            self.recommend = recommend;
            self.comment_cnt = comment_cnt;
            self.comment = comment;
            self.view = view;
        }
        pub fn sort_article(&self) -> String {
            let regex = Regex::new(r"\[([^\[\]]+)\]").unwrap();
            let stripped_body = html2text::from_read(self.body.as_bytes(), 150);
            let stripped_body = regex.replace_all(&stripped_body, "[IMAGE]");
            let mut text_output = format!(
                "작성자:{} [{}]\n{}\n{}\t조회: {}\t추천: {}\t댓글: {}\n\n{}\n\n[댓글]\n",
                self.writer,
                self.writer_info,
                self.title,
                self.time,
                self.view,
                self.recommend,
                self.comment_cnt,
                stripped_body
            );
            for i in 0..self.comment.len() {
                let cmt = &self.comment[i];
                let tmp_writer_info = cmt.writer_info.rsplit_once("/");
                let mut splitted_writer_info = String::new();
                match tmp_writer_info {
                    Some((_, r)) => splitted_writer_info = r.to_string(),
                    None => (),
                }
                let comment_list = format!(
                    "{} [{}]: {} |{}|{}|\n",
                    cmt.writer, splitted_writer_info, cmt.body, cmt.date, cmt.comment_type
                );
                text_output.push_str(&comment_list);
            }
            let raw_text = format!("\nRAW\n{}", self.body);
            text_output.push_str(&raw_text);

            text_output
        }
    }
    #[derive(Debug)]
    pub struct CommentInfo {
        writer: String,
        writer_info: String,
        body: String,
        date: String,
        comment_type: String,
    }
    impl CommentInfo {
        pub fn new(
            writer: String,
            writer_info: String,
            body: String,
            date: String,
            comment_type: String,
        ) -> CommentInfo {
            CommentInfo {
                writer: writer,
                writer_info: writer_info,
                body: body,
                date: date,
                comment_type: comment_type,
            }
        }
    }
}
