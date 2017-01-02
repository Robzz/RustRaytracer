pub fn filter_nones<T>(v: Vec<Option<T>>) -> Vec<T> {
    v.into_iter().filter_map(|i| i).collect::<Vec<T>>()
}
