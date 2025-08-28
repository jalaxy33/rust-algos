//! 桶排序

/**
桶排序是一种针对数值的排序算法。它的基本思想是将输入数据分到几个有序的桶中，
然后对每个桶分别排序，最后再将各个桶中的数据合并起来。

桶排序的步骤如下：
1. 初始化桶：创建若干个空桶。
2. 分配元素：遍历输入数据，将每个元素放入对应的桶中。
3. 排序每个桶：对每个桶内部进行排序（可以使用其他排序算法）。
4. 合并桶：按照桶的顺序将所有桶中的元素合并起来。

### 分析：

用自然数的排序就很好理解桶排序的思路了，
首先按最高位进行分桶，最高位为1的分一堆，2的分另一堆，依此类推。
可知最高位为1的数字一定比最高位为2的数字小。
然后再在桶内部进行排序，排序完之后所有数字就自然有序了。

时间复杂度：
- 分配元素：遍历列表所有元素，O(n)
- 排序每个桶：假设每个桶元素为m，桶的数量为k，每个桶的排序复杂度为O(m log m)，整体时间复杂度为O(k * m log m)。
- 合并桶：将所有桶中的元素依次取出，时间复杂度为O(n)
- 总时间复杂度：O(n + k * m log m)

空间复杂度：需要额外的存储空间来存放桶和排序后的结果，O(n + k)

### 优化：

函数参数列表中，参数的类型是 &[usize]，表示一个无符号整数的切片，类型有最小值 0，
可以简化桶的创建。

 */
fn bucket_sort(arr: &[usize]) -> Vec<usize> {
    if arr.is_empty() {
        return vec![];
    }
    
    // 创建桶
    let max = *arr.iter().max().unwrap();
    let len = arr.len();
    let mut buckets = vec![vec![]; len + 1];

    // 分配元素
    for &x in arr {
        buckets[len * x / max].push(x);
    }

    // 排序每个桶
    for bucket in &mut buckets {
        bucket.sort();  // 使用内置的排序算法
    }

    // 合并
    let mut result = vec![];
    for bucket in buckets {
        result.extend(bucket);
    }
    result
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted(arr: &[usize]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    fn same_len(a: &[usize], b: &[usize]) -> bool {
        a.len() == b.len()
    }

    #[test]
    fn empty() {
        let arr: [usize; 0] = [];
        let res = bucket_sort(&arr);
        assert!(same_len(&arr, &res));
        assert!(is_sorted(&res));
    }

    #[test]
    fn one_element() {
        let arr: [usize; 1] = [4];
        let res = bucket_sort(&arr);
        assert!(same_len(&arr, &res));
        assert!(is_sorted(&res));
    }

    #[test]
    fn already_sorted() {
        let arr: [usize; 3] = [10, 9, 105];
        let res = bucket_sort(&arr);
        assert!(same_len(&arr, &res));
        assert!(is_sorted(&res));
    }

    #[test]
    fn basic() {
        let arr: [usize; 4] = [35, 53, 1, 0];
        let res = bucket_sort(&arr);
        assert!(same_len(&arr, &res));
        assert!(is_sorted(&res));
    }

    #[test]
    fn odd_number_of_elements() {
        let arr: Vec<usize> = vec![1, 21, 5, 11, 58];
        let res = bucket_sort(&arr);
        assert!(same_len(&arr, &res));
        assert!(is_sorted(&res));
    }

    #[test]
    fn repeated_elements() {
        let arr: Vec<usize> = vec![542, 542, 542, 542];
        let res = bucket_sort(&arr);
        assert!(same_len(&arr, &res));
        assert!(is_sorted(&res));
    }
}
