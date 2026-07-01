#[derive(Debug)]
pub struct Url{
    pub base: String,
    pub dns: String,
    pub another_url: String
}
impl Url{
    pub fn new(str_url: &'static str) -> Url{
        let base= "https://".to_string(); // 
        let dns= "x.com".to_string();
        let another_url= str_url[13..].to_string();

        Url { base, dns, another_url }
    }

    pub fn get_url(&self) -> String{
        format!("{}{}{}", self.base, self.dns, self.another_url)
    }
}


//  x 포스팅 데이터 스트럭쳐 -> url로 데이터 형성
pub struct XPosting {
    pub url: Url,
}

// 에러 핸들링
impl XPosting {
    pub fn new(str_url: &'static str) -> Self {
        let url= Url::new(str_url);
        Self { url }
    }
}

// x포스팅들을 벡터화 함
pub struct XPostingList {
    pub postings: Box<Vec<XPosting>>,
}
impl XPostingList {
    pub fn new() -> Self{
        let postings: Box<Vec<XPosting>>= Box::from(Vec::new());
        return Self { postings };
    }
    pub fn add_x_post(&mut self, x_post: XPosting) -> &Box<Vec<XPosting>> {
        let _= &self.postings.push(x_post);
        
        &self.postings
    }
}