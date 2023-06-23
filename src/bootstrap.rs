use rand::seq::SliceRandom;
use std::time::Instant;

/*creating a resampling function for generic datatype.
This function takes a reference to a vector of generic datatype and returns a vector of generic datatype.
*/
pub fn random_sample_with_replacement(sample: &[i64], size: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let mut resampled = Vec::with_capacity(size);

    for _ in 0..size {
        let item = sample.choose(&mut rng).unwrap().clone();
        resampled.push(item);
    }

    resampled
}

//generating bootstrapping sample groundtruth using simple random sampling with replacement

pub fn bootstrap_sums(data: &[i64], num_resamples: usize, sample_fraction: f64) -> (Vec<i64>, f64) {
    let mut bootstrap_sums = Vec::with_capacity(num_resamples);

    // Start the timer
    let start_time = Instant::now();

    for _ in 0..num_resamples {
        let resampled_data = random_sample_with_replacement(&data, data.len());
        let sum: i64 = resampled_data.iter().sum();
        bootstrap_sums.push((sum as f64 / sample_fraction) as i64);
    }

    // Calculate the elapsed time
    let elapsed_time = start_time.elapsed().as_secs_f64();

    (bootstrap_sums, elapsed_time)
}

//calculating mean of bootstrapping ground truth sample
pub fn calculate_mean(bootstrap_sums: &[i64]) -> f64 {
    let sum: i64 = bootstrap_sums.iter().sum();
    sum as f64 / bootstrap_sums.len() as f64
}

//calculating standard deviation of bootstrapping ground truth sample
pub fn calculate_std_error(bootstrap_sums: &[i64], mean: f64) -> f64 {
    let count = bootstrap_sums.len() as f64;
    let variance: f64 = bootstrap_sums
        .iter()
        .map(|&value| {
            let diff = value as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / count
        - 1.0;

    variance.sqrt()
}
