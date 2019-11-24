/// In England the currency is made up of pound, £, and pence, p, and there are eight coins in
/// general circulation:
///
/// 1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
/// It is possible to make £2 in the following way:
///
/// 1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
/// How many different ways can £2 be made using any number of coins?

fn coin_sums(goal: u32) -> u32 {
    let coins = [1u32, 2, 5, 10, 20, 50, 100, 200];
    let mut ways = vec![0u32; goal as usize + 1];

    ways[0] = 1;

    for i in 0..coins.len() {
        for j in coins[i]..goal + 1 {
            ways[j as usize] += ways[(j - coins[i]) as usize];
        }
    }

    ways[goal as usize]
}

fn main() {
    println!("{}", coin_sums(200));
}
