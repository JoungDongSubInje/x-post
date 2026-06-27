#[derive(Debug)]
pub struct Url{
    base: String,
    dns: String,
    another_url: String
}
impl Url{
    pub fn new(str_url: &'static str) -> Url{
        let base= "https://".to_string(); // 
        let dns= "x.com".to_string();
        let another_url= str_url[13..].to_string();

        Url { base, dns, another_url }
    }
}


//  x 포스팅 데이터 스트럭쳐 -> url로 데이터 형성
pub struct XPosting {
    url: Url,
}

// 에러 핸들링
impl XPosting {
    pub fn new(str_url: &'static str) -> Option<Self> {
        let url= Url::new(str_url);
        Some(XPosting { url })
    }
}

// x포스팅들을 벡터화 함
pub struct XPostingList {
    pub postings: Vec<XPosting>,
}
impl XPostingList {
    pub fn new() -> Self{
        let postings: Vec<XPosting>= Vec::new();
        return XPostingList { postings };
    }
    pub fn add_x_post(&mut self, x_post: XPosting) {
        self.postings.push(x_post);
    }
}

pub struct KoreaXPosting;

impl KoreaXPosting{
    // X 포스팅 url을 매개변수로 받기
    pub fn push_x_posting(mut x_posting_list: XPostingList, post_query: &'static str) -> XPostingList{
        // static base) https://
        // static base) x.com
        // needed) /SpaceX/status/2065415377165726146
        let option_x_post= XPosting::new(post_query);
        
        let x_post= match option_x_post {
            Some(XPosting) => XPosting,
            None => panic!("URL input Err"), 
        };

        x_posting_list.add_x_post(x_post);
        
        x_posting_list
    }
}