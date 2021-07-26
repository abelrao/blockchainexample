struct SumVec {}


impl SumVec {
    // todo error
    pub fn sum(vecs: &[u32]) -> Option<u32> {
        if vecs.len() == 0 {
            return None;
        }
        let mut sum = vecs[0];
        for index in 1..vecs.len() {
            if std::u32::MAX - vecs[index] < sum {
                return None;
            }
            sum = sum + vecs[index];
        }
        return Some(sum);
    }
}


#[cfg(test)]
mod sum_test {
    use crate::sum_u32::SumVec;

    #[test]
    fn test() {
        let res = vec![1, 2, 3, 4, 5];
        println!("src vec: {:?}", res);
        if let Some(sum) = SumVec::sum(&res) {
            println!("result sum: {}", sum);
        } else {
            println!("result: {}", "have a error");
        }
    }
}