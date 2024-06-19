use num::Num;
use rand::{thread_rng, distributions::uniform::SampleUniform, Rng};
use std::cmp::PartialOrd;

/// Generates random number of type `Integer` or `Float`
///
/// ## Params:
///
/// - **_low_** : The lowest number of the range
/// - **_high_** : The highest number of the range
///
/// ## Returns:
///
/// `T`: a Generic type of either `i32` or `f64`
///
/// ## Since:
///
/// v1.1.0
///
/// ## Usage:
///
/// - The parameters can either be `Integer` or `Float`
/// - parameter `low` and `high` must be of same type i.e. you cannot set a range of say from 1 to 10.1
/// - The second parameter `high` is inclusive i.e. a range of 1 and 10 will mean the range includes from 1 to 10
///
/// ## Example:
///
/// ```rust
/// use best_skn_utils::random::gen_random_number;
///
/// let num1: i32 = gen_random_number(1, 10);
/// let num2: f64 = gen_random_number(1.5, 7.5);
/// ```
pub fn gen_random_number<T>(low: T, high: T) -> T
where
  T: Num + SampleUniform + PartialOrd,
{
  let random_number: T = thread_rng().gen_range(low..=high);

  random_number
}
