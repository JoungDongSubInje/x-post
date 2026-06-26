# A sort of koreaOfs project

이 컴포넌트는 코리아 오브의 X포스팅 코어 관련 컴포넌트입니다.

```rust

use x_post::KoreaXPosting;

fn main(){

        // static base) https://
        // static base) x.com
        // needed) /SpaceX/status/2065415377165726146
    let post_query= "/SpaceX/status/2065415377165726146";
    let v_korea_x_posts= KoreaXPosting::push_x_posing(post_query);
}

```
