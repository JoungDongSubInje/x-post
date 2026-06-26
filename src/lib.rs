//  x 포스팅 데이터 스트럭쳐 -> url로 데이터 형성
struct XPosting {
    url: String,
}

// 에러 핸들링
impl XPosting {
    fn new(url: String) -> Option<Self> {
        Some(XPosting { url })
    }
}

// x포스팅들을 벡터화 함
struct XPostingList {
    postings: Vec<XPosting>,
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
    pub fn push_x_posing(url: &'static str) -> XPostingList{
        let url= url.to_string();

        // 해당 x포스팅들을 익스포트 해서 리스트 만들어 데이터 넣기 
        let mut v_x_posts= XPostingList::new();
        let option_x_post= XPosting::new(url);
        
        let x_post= match option_x_post {
            Some(XPosting) => XPosting,
            None => panic!("URL input Err"), 
        };

        v_x_posts.add_x_post(x_post);
        
        v_x_posts
    }
}