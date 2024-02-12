use std::cmp::Reverse;
use std::collections::binary_heap::BinaryHeap;
use std::mem::size_of;
use std::time::Instant;
fn main() {
    let amount = 1000000;
    println!("generating {amount} primes");
    let start = Instant::now();
    let (arr, mem) = get_primes_seg_sieve(amount);
    let end = start.elapsed();
    println!(
        "get_primes_segmented_sieve: time = {end:?}, last prime = {:?}, mem footprint = {mem} bytes",
        arr.last()
    );
    let start = Instant::now();
    let (arr, mem) = get_primes_classic_sieve(amount);
    let end = start.elapsed();
    println!(
        "get_primes_classic_sieve: time = {end:?}, last prime = {:?}, mem footprint = {mem} bytes",
        arr.last()
    );
    let start = Instant::now();
    let (arr, mem) = get_primes_dijksta_vec(amount);
    let end = start.elapsed();
    println!(
        "get_primes_dijksta_vec: time = {end:?}, last prime = {:?}, mem footprint = {mem} bytes",
        arr.last()
    );
    let start = Instant::now();
    let (arr, mem) = get_primes_dijksta_binary_heap(amount);
    let end = start.elapsed();
    println!(
        "get_primes_dijksta_heap: time = {end:?}, last prime = {:?}, mem footprint = {mem} bytes",
        arr.last()
    );
    let start = Instant::now();
    let (arr, mem) = get_primes_div(amount);
    let end = start.elapsed();
    println!(
        "get_primes_division classic divisions: time = {end:?}, last prime = {:?}, mem footprint = {mem} bytes",
        arr.last()
    );
}

fn get_primes_dijksta_binary_heap(amount: usize) -> (Vec<usize>, usize) {
    let mut primes: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::with_capacity(amount);
    let mut ret = Vec::with_capacity(amount);
    ret.push(2);
    primes.push((Reverse(4), 2));
    let mut i = 3;
    while primes.len() < amount {
        //println!("{i}, {primes:?}");
        let factor = primes.peek().unwrap();
        if factor.0 .0 > i {
            primes.push((Reverse(i * i), i));
            ret.push(i);
        } else {
            let mut a = primes.pop().unwrap();
            a.0 .0 += a.1;
            primes.push(a);
            while primes.peek().unwrap().0 .0 == i {
                let mut a = primes.pop().unwrap();
                a.0 .0 += a.1;
                primes.push(a)
            }
        }
        i += 1;
    }
    let len = ret.len();
    (
        ret,
        len * size_of::<usize>() + primes.len() * size_of::<(usize, usize)>(),
    )
}

fn get_primes_dijksta_vec(amount: usize) -> (Vec<usize>, usize) {
    let mut primes = Vec::with_capacity(amount);
    primes.push((2, 4));
    let mut i = 3;
    let mut flag;
    while primes.len() < amount {
        //println!("i: {i}, primes: {primes:?}");
        flag = true;
        for ii in 0..primes.len() {
            let (prime, next_apperence) = primes[ii];
            if next_apperence == i {
                primes[ii].1 += primes[ii].0;
                flag = false;
            } else if prime * prime == next_apperence {
                break;
            }
        }
        if flag {
            primes.push((i, i * i))
        }
        i += 1;
    }
    let len = primes.len();
    (
        primes.into_iter().map(|i| i.0).collect(),
        len * size_of::<(usize, usize)>(),
    )
}

fn get_primes_div(amount: usize) -> (Vec<usize>, usize) {
    let mut ret = Vec::with_capacity(amount);
    ret.push(2);
    let mut i = 3;
    while ret.len() < amount {
        let mut is_prime = true;
        for ii in &ret {
            if ii * ii > i {
                break;
            }
            if i % ii == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            ret.push(i);
        }
        i += 2
    }
    let len = ret.len();
    (ret, len * size_of::<usize>())
}

struct SegmentedPrimeSieve {
    primes: Vec<usize>,
    segment: usize,
}

impl SegmentedPrimeSieve {
    fn new(amount: usize) -> Self {
        let mut primes = vec![2, 3, 5, 7];
        primes.reserve(amount);
        SegmentedPrimeSieve { primes, segment: 1 }
    }

    fn add_next_segment(&mut self) {
        let start = self.primes[self.segment];
        let end = self.primes[self.segment + 1];
        let seg_start = start * start;
        let seg_end = end * end;
        let mut seg = vec![false; seg_end - seg_start];

        for i in 1..=self.segment {
            let prime_num = self.primes[i];
            let mut ind = seg_start + (prime_num - (seg_start % prime_num)) % prime_num;
            while ind - seg_start < seg.len() {
                seg[ind - seg_start] = true;
                ind += prime_num;
            }
        }

        for ind in (seg_start..seg_end).step_by(2) {
            if !seg[ind - seg_start] {
                self.primes.push(ind);
            }
        }
        self.segment += 1;
    }
    fn gen_amount(&mut self, amount: usize) {
        while self.primes.len() < amount {
            self.add_next_segment()
        }
    }
}

fn get_primes_seg_sieve(amount: usize) -> (Vec<usize>, usize) {
    let mut prime_get = SegmentedPrimeSieve::new(amount);
    prime_get.gen_amount(amount);
    unsafe { prime_get.primes.set_len(amount) }
    let start = prime_get.primes[prime_get.segment];
    let end = prime_get.primes[prime_get.segment + 1];
    let seg_start = start * start;
    let seg_end = end * end;
    let seg = (seg_end - seg_start) * size_of::<bool>();
    let len = prime_get.primes.len();
    (prime_get.primes, len * size_of::<usize>() + seg)
}

struct PrimesClassicSieve {
    primes: Vec<usize>,
}

impl PrimesClassicSieve {
    fn new(amount: usize) -> Self {
        // estimate for how many primes are under any given number
        let mut primes = Vec::<usize>::with_capacity(1 + amount / (amount as f64).ln() as usize);
        // start with 2 as prime
        primes.push(2);
        let mut arr = vec![true; amount];
        arr[0] = false;
        arr[1] = false;
        // sieve of eratosthenes
        for i in (3..(arr.len() as f64).sqrt() as usize).step_by(2) {
            if arr[i] {
                for ind in (i * 2..arr.len()).step_by(i) {
                    arr[ind] = false;
                }
            }
        }
        for i in (1..arr.len()).step_by(2) {
            if arr[i] {
                primes.push(i);
            }
        }
        PrimesClassicSieve { primes }
    }
}

fn get_primes_classic_sieve(amount: usize) -> (Vec<usize>, usize) {
    let mut size = amount;
    let mut mem_fp = size * 8;

    let mut prime_get = PrimesClassicSieve::new(size);
    while prime_get.primes.len() < amount {
        size *= 2;
        prime_get = PrimesClassicSieve::new(size);
        mem_fp = size * size_of::<bool>();
    }
    unsafe { prime_get.primes.set_len(amount) }
    let len = prime_get.primes.len();
    (prime_get.primes, len * size_of::<usize>() + mem_fp)
}
