pub mod emirs_space{

    use std::time::Instant;
    use rand::{Rng, thread_rng};

    // Create comb sort function that sorts a vector of integers
    fn comb_sort<T: Ord>(v: &mut [T])
    {
        let mut gap = v.len();
        let shrink = 1.3;
        let mut sorted = false;

        while !sorted {
            // Update the gap value for a next comb
            gap = (gap as f64 / shrink) as usize;
            if gap > 1 {
                sorted = false;
            } else {
                gap = 1;
                sorted = true;
            }

            // A single "comb" over the input list
            let mut i = 0;
            while i + gap < v.len() {
                if v[i] > v[i + gap] {
                    v.swap(i, i + gap);
                    sorted = false;
                }
                i += 1;
            }
        }
    }

    // Create gnome sort function that sorts a vector of integers
    fn gnome_sort<T: Ord>(v: &mut [T])
    {
        let mut i = 1;
        let mut j = 2;

        while i < v.len() {
            if v[i - 1] <= v[i] {
                i = j;
                j += 1;
            } else {
                v.swap(i - 1, i);
                if i > 1 {
                    i -= 1;
                }
            }
        }
    }

    // Create shaker sort function that sorts a vector of integers
    fn shaker_sort<T: Ord>(v: &mut [T])
    {
        let mut left = 0;
        let mut right = v.len() - 1;

        while left < right {
            for i in left..right {
                if v[i] > v[i + 1] {
                    v.swap(i, i + 1);
                }
            }
            right -= 1;

            for i in (left..right).rev() {
                if v[i] > v[i + 1] {
                    v.swap(i, i + 1);
                }
            }
            left += 1;
        }
    }

    // A function that runs an action and measures the time it takes
    fn time_it<F>(action: F) -> f64
        where F: FnOnce() -> () {
        let start = Instant::now();
        action();
        let duration = start.elapsed();
        return duration.as_secs_f64()
    }

    fn get_random_number() -> i32 {
        let mut rng = thread_rng();
        let random_number = rng.gen_range(0..8192);
        return random_number;
    }

      pub fn run_algorithms(limit : i32){

        let random_list = (0..limit).map(|_| get_random_number()).collect::<Vec<i32>>();
        // let orderedListAsc = (0..limit).collect::<Vec<i32>>();
        // let orderedListDesc = (0..limit).rev().collect::<Vec<i32>>();
        let mut random_list_copy = random_list.clone();

        println!("--- Average case ----");
        println!("Starting random list: {:?}", random_list_copy);

        let time_took_comb = time_it(|| {
            comb_sort(&mut random_list_copy);
        });
        println!("Time took to sort comb: {} seconds", time_took_comb);

        random_list_copy.clear();
        random_list.into_iter().for_each(|x| random_list_copy.push(x));

        let time_took_gnome = time_it(|| {
            gnome_sort(&mut random_list_copy);
        });
        println!("Time took to sort gnome: {} seconds", time_took_gnome);

        random_list_copy.clear();
        random_list.into_iter().for_each(|x| random_list_copy.push(x));

        let time_took_gnome = time_it(|| {
            // sort here
            gnome_sort(&mut random_list_copy);
        });
        println!("Time took to sort gnome: {} seconds", time_took_gnome);
    }

    pub fn main(){

        let mut list = vec![64, 128, 256, 512, 1024, 2048, 4096, 8192];
        let mut index = 0;
        while index < list.len() {
            run_algorithms(list[index]);
            index += 1;
        }
    }
}