use rand::{seq::SliceRandom, thread_rng};

/*creating a resampling function for generic datatype.
This function takes a reference to a vector of generic datatype and returns a vector of generic datatype.
*/
pub fn resampling<T: Clone>(data: &Vec<T>) -> Vec<T> {
    let mut rng = thread_rng();
    data.choose_multiple(&mut rng, data.len())
        .cloned()
        .collect()
}

//Function to
