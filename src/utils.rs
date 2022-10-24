pub(crate) fn repeat(data: Vec<f32>, take: usize) -> Vec<f32> {
    let mut result = vec![];
    for one_of_data in data.iter() {
        for _ in 0..take {
            result.push(one_of_data.clone());
        }
    }
    result
}
