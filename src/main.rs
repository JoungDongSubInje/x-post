//  x 포스팅 데이터 스트럭쳐 -> url로 데이터 형성
struct XPosting {
    url: String,
}
impl XPosting {
    fn new(url: String) -> Self {
        self { url }
    }
}

// x포스팅들을 벡터화 함
struct XPostingList {
    postings: Vec<XPosting>,
}
impl XPostingList {
    fn new() -> Self {
        XPostingList {
            postings: Vec::new(),
        }
    }

    fn add(&mut self, x_post: XPosting) {
        self.postings.push(x_post);
    }
}   

// 형식에 맞게 1개의 파일로 익스포트
// html -> 축약되어 나타나는 url 디스크립션을 보여주기 
// 마크다운 -> 동일하기 url 디스크립션으로 보여주는 짧은 커멘드 생성


fn main() {
    // X 포스팅 url을 매개변수로 받기
    let x_post_a= XPosting::new("https://x.com/post/1".to_string());
    let x_post_b= XPosting::new("https://x.com/post/2".to_string()); 

    // 해당 x포스팅들을 익스포트 해서 리스트 만들기 
    let mut x_posting_list = XPostingList::new();
    x_posting_list.add(x_post_a);
    x_posting_list.add(x_post_b);

    // 마크다운 혹은 html형식으로 빼내기

}