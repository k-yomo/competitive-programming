
fn modpow(x: u128, e: u128, m: u128) -> u128 {
    let mut x = x;
    let mut e = e;
    let mut res = 1;
    while e > 0 {
        if e & 1 != 0 {
            res *= x;
            res %= m;
        }
        x *= x;
        x %= m;
        e >>= 1;
    }
    return res;
}
