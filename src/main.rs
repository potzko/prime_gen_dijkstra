use std::cmp::Reverse;
use std::collections::binary_heap::BinaryHeap;
use std::mem::size_of;
use std::time::Instant;
fn primes_under_n_approx(n: usize) -> usize {
    (n as f64 / (n as f64).ln()) as usize
}

fn main() {
    let bound = 50000000;
    println!("generating primes under {bound}");
    let start = Instant::now();
    let (_arr, mem) = get_primes_seg_sieve(bound);
    let end = start.elapsed();
    println!("get_primes_segmented_sieve: time = {end:?}, mem footprint = {mem} bytes");
    let start = Instant::now();
    let (_arr, mem) = get_primes_classic_sieve(bound);
    let end = start.elapsed();
    println!("get_primes_classic_sieve: time = {end:?}, mem footprint = {mem} bytes");

    let start = Instant::now();
    let (_arr, mem) = get_primes_video_vec(bound);
    let end = start.elapsed();
    println!("get_primes_video_vec: time = {end:?}, mem footprint = {mem} bytes");
    let start = Instant::now();
    let (_arr, mem) = get_primes_video_binary_heap(bound);
    let end = start.elapsed();
    println!("get_primes_video_heap: time = {end:?}, mem footprint = {mem} bytes");

    let start = Instant::now();
    let (_arr, mem) = get_primes_div(bound);
    let end = start.elapsed();
    println!("get_primes_division: time = {end:?}, mem footprint = {mem} bytes");
    let start = Instant::now();
    let (_arr, mem) = get_primes_dijksta(bound);
    let end = start.elapsed();
    println!("get_primes_dijksta: time = {end:?}, mem footprint = {mem} bytes");
}

fn get_primes_video_binary_heap(bound: usize) -> (Vec<usize>, usize) {
    let mut primes: BinaryHeap<(Reverse<usize>, usize)> =
        BinaryHeap::with_capacity(primes_under_n_approx(bound));
    let mut ret = Vec::with_capacity(primes_under_n_approx(bound));
    ret.push(2);
    primes.push((Reverse(4), 2));
    for i in 3..bound {
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
    }
    (ret, primes.len() * size_of::<(usize, usize)>())
}

fn get_primes_video_vec(bound: usize) -> (Vec<usize>, usize) {
    let mut primes = Vec::with_capacity(primes_under_n_approx(bound));
    primes.push((2, 4));
    let mut flag;
    for i in 3..bound {
        //println!("i: {i}, primes: {primes:?}");
        flag = true;
        for ii in primes.iter_mut() {
            if ii.1 == i {
                ii.1 += ii.0;
                flag = false;
            } else if ii.0 * ii.0 == ii.1 {
                break;
            }
        }
        if flag {
            primes.push((i, i * i))
        }
    }
    let len = primes.len();
    (
        primes.into_iter().map(|i| i.0).collect(),
        len * size_of::<(usize, usize)>(),
    )
}

fn get_primes_dijksta(bound: usize) -> (Vec<usize>, usize) {
    let mut power_vec =
        Vec::with_capacity(primes_under_n_approx((bound as f64).sqrt().ceil() as usize));
    power_vec.push(4);
    let mut primes: Vec<usize> = Vec::with_capacity(primes_under_n_approx(bound));
    primes.push(2);
    for i in (3..bound).step_by(2) {
        //println!("{i}, {:?}", power_vec);
        if i == *power_vec.last().unwrap() {
            power_vec.push(primes[power_vec.len()].pow(2))
        }
        let mut flag = true;
        for ii in 1..power_vec.len() {
            power_vec[ii] += if power_vec[ii] < i { primes[ii] } else { 0 };
            if power_vec[ii] == i {
                flag = false;
                break;
            }
        }

        if flag {
            primes.push(i);
            if *power_vec.last().unwrap() < i {
                power_vec.push(primes[power_vec.len()] * primes[power_vec.len()])
            }
        }
    }
    let size = size_of::<usize>() * (power_vec.len());
    (primes, size)
}

fn get_primes_div(bound: usize) -> (Vec<usize>, usize) {
    let mut ret = Vec::with_capacity(primes_under_n_approx(bound));
    ret.push(2);
    for i in (3..bound).step_by(2) {
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
    }
    (ret, 0)
}

struct SegmentedPrimeSieve {
    primes: Vec<usize>,
    segment: usize,
}

impl SegmentedPrimeSieve {
    fn new(amount: usize) -> Self {
        let mut primes = vec![2, 3, 5, 7];
        primes.reserve(primes_under_n_approx(amount));
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
    fn gen_amount(&mut self, bound: usize) -> usize {
        while self.primes[self.segment + 1] * self.primes[self.segment + 1] < bound {
            self.add_next_segment()
        }
        let start = self.primes.len();
        self.add_next_segment();
        let range_len = self.primes.len() - start;
        match self.primes[start..].binary_search(&bound) {
            Ok(num) => self.primes.len() - (range_len - num),
            Err(num) => self.primes.len() - (range_len - num),
        }
    }
}

fn get_primes_seg_sieve(bound: usize) -> (Vec<usize>, usize) {
    if bound == 0 {
        return (Vec::new(), 0);
    }
    let mut prime_get = SegmentedPrimeSieve::new(bound);
    let ind = prime_get.gen_amount(bound);
    let start = prime_get.primes[prime_get.segment].pow(2);
    let end = prime_get.primes[prime_get.segment + 1].pow(2);
    let seg = (end - start) * size_of::<bool>();
    unsafe { prime_get.primes.set_len(ind) }
    (prime_get.primes, seg)
}

struct PrimesClassicSieve {
    primes: Vec<usize>,
}

impl PrimesClassicSieve {
    fn new(bound: usize) -> Self {
        // estimate for how many primes are under any given number
        let mut primes = Vec::<usize>::with_capacity(primes_under_n_approx(bound));
        // start with 2 as prime
        primes.push(2);
        let mut arr = vec![true; bound];
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
    if amount == 0 {
        return (Vec::new(), 0);
    }
    let mem_fp = amount * size_of::<bool>();
    let prime_get = PrimesClassicSieve::new(amount);
    (prime_get.primes, mem_fp)
}
