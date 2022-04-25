fn main() {
    println!("Hello, world!");
}

pub fn chop(search_value: i32, array: Vec<i32>) -> i32 {
    if array.is_empty() { return -1; }
    let (mut begin, mut end) = (0, array.len() - 1);
    while begin <= end {
        if begin == end && array[begin] != search_value {
            return -1;
        }
        let middle: usize = (begin + end) / 2;
        if search_value == array[middle] {
            return middle as i32;
        }
        if search_value < array[middle] {
            end = middle - 1;
        }
        else if search_value > array[middle] {
            begin = middle + 1;
        }
    };
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        assert_eq!(-1, chop(3, [].to_vec()))
    }

    #[test]
    fn test_single_element_array_not_found() {
        assert_eq!(-1, chop(3, [1].to_vec()))
    }

    #[test]
    fn test_single_element_array_found() {
        assert_eq!(0, chop(3, [3].to_vec()))
    }

    #[test]
    fn test_multiple_elements_array_found() {
        assert_eq!(0, chop(1, [1, 3, 5].to_vec()))
    }

    #[test]
    fn test_even_number_of_elements_found() {
        assert_eq!(3, chop(7, [1, 3, 5, 7, 9, 11, 13, 15].to_vec()))
    }

    #[test]
    fn test_even_number_of_elements_not_found() {
        assert_eq!(-1, chop(12, [1, 3, 5, 7, 9, 11, 13, 15].to_vec()))
    }

    #[test]
    fn test_odd_number_of_elements_found() {
        assert_eq!(3, chop(7, [1, 3, 5, 7, 9, 11, 13].to_vec()))
    }

    #[test]
    fn test_odd_number_of_elements_not_found() {
        assert_eq!(-1, chop(8, [1, 3, 5, 7, 9, 11, 13].to_vec()))
    }
}