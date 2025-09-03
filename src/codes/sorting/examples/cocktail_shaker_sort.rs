//! 鸡尾酒排序

/**
鸡尾酒排序可以理解成冒泡排序的双向版本。先从左到右遍历一遍，再从右到左遍历一遍，直到没有元素需要交换为止。

### 分析

从左到右遍历时，最大的元素会被交换到最右边；从右到左遍历时，最小的元素会被交换到最左边。
每次双向遍历后，未排序部分的范围都会缩小。

时间复杂度：O(n^2)，空间复杂度：O(1)。

### 优化

引入一个变量标记当前循环是否发生元素交换，如果没有发生交换，说明数组已经有序，可以提前结束排序。
 */
fn cocktail_shaker_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    
    if len == 0 {
        return;
    }

    let mut bottom = 0;
    let mut top = len - 1;

    loop {
        let mut swapped = false;
        for i in bottom..top {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        top -= 1;
        for j in ((bottom + 1)..=top).rev() {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                swapped = true;
            }
        }
        bottom += 1;

        if !swapped {
            break;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut arr = vec![5, 2, 1, 3, 4, 6];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn empty() {
        let mut arr = Vec::<i32>::new();
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn one_element() {
        let mut arr = vec![1];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        cocktail_shaker_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }
}
