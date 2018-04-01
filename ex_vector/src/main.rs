use std::collections::HashMap;

fn main() {
    let mut ls = vec![3, 4, 2, 7, 9, 1, 1];

    println!("mean={}", mean(&ls));
    println!("median={}", median(&mut ls));
    println!("mode={}", mode(&ls));
}

fn mean(ls: &[usize]) -> f32 {
    let mut _sum: usize = ls.iter().sum();
    _sum *= 100;
    (_sum / ls.len()) as f32 / 100.0
}

fn median(ls: &mut [usize]) -> f32 {
    ls.sort();
    if ls.len() % 2 == 0 {
        let mid_index = ls.len() / 2 - 1;
        let _sum = ls[mid_index] + ls[mid_index + 1];
        return _sum as f32 / 2.0;
    }
    ls[ls.len() / 2] as f32
}

fn mode(ls: &[usize]) -> usize {
    let mut counter: HashMap<usize, usize> = HashMap::with_capacity(ls.len());
    for nr in ls {
        let stat = counter.entry(*nr).or_insert(0);
        *stat += 1;
    }
    let res = counter
        .iter()
        .max_by(|&(_, v1), &(_, v2)| v1.cmp(v2))
        .unwrap();
    *res.0
}
