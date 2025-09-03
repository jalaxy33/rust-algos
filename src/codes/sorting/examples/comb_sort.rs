//! 梳排序

/**
梳排序是一种改进的冒泡排序，开始时会使用一个较大的间隔对元素进行比较和交换，
随着排序的进行，逐渐减小间隔，最终达到相邻元素的比较。

### 分析

开始时间隔通常设为数组长度，随后以一个固定的收缩率逐渐减小（通常取 1.3），直到间隔为1。
因为编程语言中乘法比除法快，故会取递减率倒数 (0.8) 与间距相乘。

复杂度：
- 时间复杂度：本质上还是冒泡排序，O(n^2)；
- 空间复杂度：O(1)。

 */
fn comb_sort<T: Ord>(arr: &mut [T]) {
    let mut gap = arr.len();
    let shrink = 0.8;
    let mut sorted = false;

    while !sorted {
        gap = (gap as f32 * shrink) as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true;
        }

        for i in 0..arr.len() - gap {
            let j = i + gap;
            if arr[i] > arr[j] {
                arr.swap(i, j);
                sorted = false;
            }
        }
    }
}


fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descending() {
        //descending
        let mut ve1 = vec![6, 5, 4, 3, 2, 1];
        comb_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }
    }

    #[test]
    fn ascending() {
        //pre-sorted
        let mut ve2 = vec![1, 2, 3, 4, 5, 6];
        comb_sort(&mut ve2);
        for i in 0..ve2.len() - 1 {
            assert!(ve2[i] <= ve2[i + 1]);
        }
    }
}