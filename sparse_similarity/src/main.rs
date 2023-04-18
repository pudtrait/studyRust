pub struct Solution;

impl Solution {
    pub fn compute_similarities(docs: Vec<Vec<i32>>) -> Vec<String> {
        let n = docs.len();
        let mut count = vec![std::collections::HashMap::<i32, i32>::new(); n];
        let mut res = vec![];

        for i in 0..n {
            let doc = &docs[i];
            for &num in doc.iter() {
                *count[i].entry(num).or_insert(0) += 1;
            }
        }

        for i in 0..n {
            for j in i+1..n {
                let mut intersect = 0;
                for (&num, &freq) in count[i].iter() {
                    if let Some(&f) = count[j].get(&num) {
                        intersect += freq.min(f);
                    }
                }
                if intersect > 0 {
                    let union = count[i].len()  + count[j].len()  - intersect as usize;
                    let sim = intersect as f64 / union as f64;
                    res.push(format!("{},{}: {:.4}", i, j, sim));
                }
            }
        }

        res
    }
}
fn main() {
    let docs = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![1, 3, 5],
        vec![2, 4, 6],
    ];
    let res = Solution::compute_similarities(docs);
    for s in res {
        println!("{}", s);
    }
}
