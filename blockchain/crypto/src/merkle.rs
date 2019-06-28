use hash::hash;
// use trancations;


fn get_merkle(curr_trans: Vec<Transaction>) -> String {
    let mut merkle = Vec::new();

    for t in &curr_trans {
        let hash = hash(t);
        merkle.push(hash);
    }

    if merkle.len() % 2 == 1 {
        let last = merkle.last().cloned().unwrap();
        merkle.push(last);
    }

    while merkle.len() > 1 {
        let mut h1 = merkle.remove(0);
        let mut h2 = merkle.remove(0);
        h1.push_str(&mut h2);
        let nh = hash(&h1);
        merkle.push(nh);
    }
    merkle.pop().unwrap()
}
