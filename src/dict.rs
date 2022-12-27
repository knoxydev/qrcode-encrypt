#[macro_export]
macro_rules! dict {
    ( $( $key:expr => $val:expr ), * ) => {
        {
            let mut dictionary = HashMap::new();
            $( dictionary.insert($key, $val); )*
            dictionary
        }
    };
}