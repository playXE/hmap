#[macro_export]
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
    () => {
        use std::collections::HashMap;
        HashMap::new();
    }
}