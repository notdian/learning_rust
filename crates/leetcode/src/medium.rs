#![allow(non_snake_case)]

#[test]
fn kthGrammer() {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let mut row = vec![0];
        for i in 1..n + 1 {
            let mut new_vec = vec![];
            let mut col = 0;
            for r in row.iter() {
                if r == &0 {
                    new_vec.push(0);
                    new_vec.push(1);
                } else {
                    new_vec.push(1);
                    new_vec.push(0);
                }
                col += 2;
                if (col * u32::pow(2, (n - i) as u32)) >= k as u32 {
                    println!("{}, {} breaking", i, col);
                    break;
                }
            }

            row = new_vec;
        }

        return row[(k - 1) as usize];
    }
    assert_eq!(kth_grammar(30, 434991989), 0);
}
