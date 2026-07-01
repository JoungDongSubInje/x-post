# A sort of koreaOfs project

> 이 컴포넌트는 코리아 오브의 X포스팅 코어 관련 컴포넌트입니다.


```rust

use x_post::{ XPosting, XPostingList};

fn main(){
    // static base) https://
    // static base) x.com
    // needed) /SpaceX/status/2065415377165726146
    let str_url= "/jds_invoker/123456789";
    let x_post= XPosting::new(str_url);

    let mut x_posts= XPostingList::new();
    x_posts.add_x_post(x_post);
}

```
