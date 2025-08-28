//! 冒泡排序

/**
冒泡排序是一种简单的排序算法。它重复地遍历要排序的元素，比较相邻的元素并交换它们的顺序。
遍历操作会持续进行，直到没有需要交换的元素为止，这意味着数组已经排序完成。

### 分析
每次从头开始遍历数组，将较大的元素逐渐“冒泡”到末尾。

时间复杂度：O(n^2)

空间复杂度：原地交换数值，O(1)


### 优化
引入一个变量标记当前循环是否发生元素交换，如果没有发生交换，说明数组已经有序，可以提前结束排序。
*/
fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let size = arr.len();
    for i in 0..(size - 1) {
        // 标记当前循环是否发生元素交换
        let mut swapped = false;

        // 最后 i 个元素已经排序完毕
        for j in 1..(size - i) {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                swapped = true;
            }
        }

        // 如果当前循环没有发生元素交换，说明数组已经有序
        if !swapped {
            break;
        }
    }
}

fn main() {}

// Rust tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vec() {
        let mut empty_vec: Vec<String> = vec![];
        bubble_sort(&mut empty_vec);
        assert_eq!(empty_vec, Vec::<String>::new());
    }

    #[test]
    fn test_number_vec() {
        let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![7, 9, 23, 30, 44, 49, 58, 72, 73, 78]);
    }

    #[test]
    fn test_string_vec() {
        let mut vec = vec![
            String::from("Bob"),
            String::from("David"),
            String::from("Carol"),
            String::from("Alice"),
        ];
        bubble_sort(&mut vec);
        assert_eq!(
            vec,
            vec![
                String::from("Alice"),
                String::from("Bob"),
                String::from("Carol"),
                String::from("David"),
            ]
        );
    }
}
