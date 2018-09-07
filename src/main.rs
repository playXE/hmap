macro_rules! hmap {
    ($($k:expr => $v:expr),*) => {
        {
            use std::collections::HashMap;
            let mut temp_hmap = HashMap::new();

            $(
                temp_hmap.insert($k,$v);
            )*

            temp_hmap
        }
    };
}

fn main() {
    let map = hmap!("s" => 1);

    println!("{}",map.get("s").unwrap());
}


