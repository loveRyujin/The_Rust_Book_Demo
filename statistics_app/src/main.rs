use std::collections::HashMap;

/// 计算中位数
/// 如果数组长度为奇数，返回中间的值
/// 如果数组长度为偶数，返回中间两个值的平均值
fn median(numbers: &mut Vec<i32>) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    
    // 排序数组
    numbers.sort();
    
    let len = numbers.len();
    if len % 2 == 1 {
        // 奇数个元素，返回中间的元素
        numbers[len / 2] as f64
    } else {
        // 偶数个元素，返回中间两个元素的平均值
        let mid1 = numbers[len / 2 - 1] as f64;
        let mid2 = numbers[len / 2] as f64;
        (mid1 + mid2) / 2.0
    }
}

/// 计算众数（出现次数最多的值）
/// 返回所有出现次数最多的值的向量
fn mode(numbers: &[i32]) -> Vec<i32> {
    if numbers.is_empty() {
        return vec![];
    }
    
    // 使用HashMap统计每个数字的出现次数
    let mut frequency_map = HashMap::new();
    
    for &num in numbers {
        *frequency_map.entry(num).or_insert(0) += 1;
    }
    
    // 找到最大出现次数
    let max_frequency = frequency_map.values().max().unwrap_or(&0);
    
    // 收集所有具有最大出现次数的数字
    let mut modes: Vec<i32> = frequency_map
        .iter()
        .filter(|&(_, frequency)| *frequency == *max_frequency)
        .map(|(&number, _)| number)
        .collect();
    
    // 排序以保证结果的一致性
    modes.sort();
    modes
}

/// 统计函数：返回中位数和众数
fn calculate_statistics(mut numbers: Vec<i32>) -> (f64, Vec<i32>) {
    let med = median(&mut numbers);
    let mod_values = mode(&numbers);
    (med, mod_values)
}

fn main() {
    // 测试用例1: 奇数个元素
    let test1 = vec![1, 3, 3, 6, 7, 8, 9];
    println!("测试数据1: {:?}", test1);
    let (median_result, mode_result) = calculate_statistics(test1.clone());
    println!("中位数: {}", median_result);
    println!("众数: {:?}", mode_result);
    println!();
    
    // 测试用例2: 偶数个元素
    let test2 = vec![1, 2, 4, 6, 8, 9];
    println!("测试数据2: {:?}", test2);
    let (median_result, mode_result) = calculate_statistics(test2);
    println!("中位数: {}", median_result);
    println!("众数: {:?}", mode_result);
    println!();
    
    // 测试用例3: 多个众数
    let test3 = vec![1, 1, 2, 2, 3, 4, 5];
    println!("测试数据3: {:?}", test3);
    let (median_result, mode_result) = calculate_statistics(test3);
    println!("中位数: {}", median_result);
    println!("众数: {:?}", mode_result);
    println!();
    
    // 测试用例4: 所有元素相同
    let test4 = vec![5, 5, 5, 5, 5];
    println!("测试数据4: {:?}", test4);
    let (median_result, mode_result) = calculate_statistics(test4);
    println!("中位数: {}", median_result);
    println!("众数: {:?}", mode_result);
    println!();
    
    // 测试用例5: 用户自定义输入示例
    println!("=== 自定义测试 ===");
    let custom_data = vec![10, 20, 20, 30, 40, 50, 20, 60];
    println!("自定义数据: {:?}", custom_data);
    let (median_result, mode_result) = calculate_statistics(custom_data);
    println!("中位数: {}", median_result);
    println!("众数: {:?}", mode_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_odd_length() {
        let mut numbers = vec![1, 3, 5];
        assert_eq!(median(&mut numbers), 3.0);
    }

    #[test]
    fn test_median_even_length() {
        let mut numbers = vec![1, 2, 3, 4];
        assert_eq!(median(&mut numbers), 2.5);
    }

    #[test]
    fn test_median_empty() {
        let mut numbers = vec![];
        assert_eq!(median(&mut numbers), 0.0);
    }

    #[test]
    fn test_mode_single() {
        let numbers = vec![1, 2, 3, 3, 4];
        assert_eq!(mode(&numbers), vec![3]);
    }

    #[test]
    fn test_mode_multiple() {
        let numbers = vec![1, 1, 2, 2, 3];
        assert_eq!(mode(&numbers), vec![1, 2]);
    }

    #[test]
    fn test_mode_empty() {
        let numbers = vec![];
        assert_eq!(mode(&numbers), vec![]);
    }
}