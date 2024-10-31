use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> Option<(i32, i32)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum = data[0] + data[1];
    let mut min_pair = (data[0], data[1]);

    for window in data.windows(2) {
        let sum = window[0] + window[1];
        if sum < min_sum {
            min_sum = sum;
            min_pair = (window[0], window[1]);
        }
    }

    Some(min_pair)
}

fn print_vector(data: &[i32]) {
    println!("Vector: {:?}", data);
    if let Some((a, b)) = min_adjacent_sum(data) {
        println!("Minimum adjacent sum pair: ({}, {})", a, b);
    } else {
        println!("No adjacent pairs found.");
    }
}

fn main() {
    let random_vector = gen_random_vector(20);
    print_vector(&random_vector);
    celebrate(); // Виклик функції celebrate, якщо це потрібно
}

// Припустимо, що ви маєте функцію celebrate в src/christmas.rs
pub(crate) fn celebrate() {
    println!("Celebrate function called!");
}

// Тести
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_random_vector() {
        let vec = gen_random_vector(20);
        assert_eq!(vec.len(), 20);
        for &num in &vec {
            assert!(num >= 10 && num < 100);
        }
    }

    #[test]
    fn test_min_adjacent_sum() {
        let data = vec![3, 5, 2, 1, 4];
        assert_eq!(min_adjacent_sum(&data), Some((2, 1)));
    }

    #[test]
    fn test_min_adjacent_sum_no_adjacent() {
        let data = vec![10];
        assert_eq!(min_adjacent_sum(&data), None);
    }

    #[test]
    fn test_min_adjacent_sum_identical() {
        let data = vec![1, 1, 1, 1];
        assert_eq!(min_adjacent_sum(&data), Some((1, 1)));
    }
}
