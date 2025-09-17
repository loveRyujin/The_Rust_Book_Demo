/// 检查字符是否为元音字母
/// 支持英文和部分带重音符号的字符
fn is_vowel(c: char) -> bool {
    match c.to_lowercase().next().unwrap_or(c) {
        'a' | 'e' | 'i' | 'o' | 'u' |
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' |
        'è' | 'é' | 'ê' | 'ë' |
        'ì' | 'í' | 'î' | 'ï' |
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' |
        'ù' | 'ú' | 'û' | 'ü' => true,
        _ => false,
    }
}

/// 检查字符是否为字母
fn is_letter(c: char) -> bool {
    c.is_alphabetic()
}

/// 将单个单词转换为 Pig Latin
/// 
/// 规则：
/// 1. 如果单词以元音开头，在末尾加上 "hay"
/// 2. 如果单词以辅音开头，将第一个辅音移到末尾并加上 "ay"
/// 3. 如果单词以多个辅音开头，将所有连续的辅音移到末尾并加上 "ay"
/// 4. 保持原始的大小写模式
fn word_to_pig_latin(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }
    
    // 将单词转换为字符向量，正确处理 UTF-8
    let chars: Vec<char> = word.chars().collect();
    
    // 检查是否包含字母
    if !chars.iter().any(|&c| is_letter(c)) {
        return word.to_string(); // 不包含字母则直接返回
    }
    
    // 找到第一个字母的位置
    let first_letter_pos = chars.iter().position(|&c| is_letter(c));
    if first_letter_pos.is_none() {
        return word.to_string();
    }
    
    let first_letter_pos = first_letter_pos.unwrap();
    let first_letter = chars[first_letter_pos];
    
    // 分离前缀（非字母部分）
    let prefix: String = chars[..first_letter_pos].iter().collect();
    let word_part: String = chars[first_letter_pos..].iter().collect();
    
    // 检查第一个字母是否为元音
    if is_vowel(first_letter) {
        // 元音开头：加上 "hay"
        return format!("{}{}-hay", prefix, word_part);
    }
    
    // 辅音开头：找到所有连续的辅音
    let word_chars: Vec<char> = word_part.chars().collect();
    let mut consonant_count = 0;
    
    for (i, &c) in word_chars.iter().enumerate() {
        if is_letter(c) && !is_vowel(c) {
            consonant_count += 1;
            // 特殊处理 "qu" 组合
            if c.to_lowercase().next() == Some('q') && 
               i + 1 < word_chars.len() && 
               word_chars[i + 1].to_lowercase().next() == Some('u') {
                consonant_count += 1; // 将 'u' 也算作辅音聚类的一部分
                break; // qu 后面就停止
            }
        } else if is_letter(c) && is_vowel(c) {
            break; // 遇到元音停止
        }
        // 遇到非字母字符时，如果还没有找到元音，继续计数
    }
    
    if consonant_count == 0 {
        // 没有辅音，按元音处理
        return format!("{}{}-hay", prefix, word_part);
    }
    
    // 分离辅音和剩余部分
    let consonants: String = word_chars[..consonant_count].iter().collect();
    let remaining: String = word_chars[consonant_count..].iter().collect();
    
    // 处理大小写：如果原始单词首字母大写，新单词首字母也要大写
    let result_word = if first_letter.is_uppercase() && !remaining.is_empty() {
        let mut remaining_chars: Vec<char> = remaining.chars().collect();
        remaining_chars[0] = remaining_chars[0].to_uppercase().next().unwrap_or(remaining_chars[0]);
        let capitalized_remaining: String = remaining_chars.iter().collect();
        let lowercase_consonants = consonants.to_lowercase();
        format!("{}{}-{}ay", prefix, capitalized_remaining, lowercase_consonants)
    } else {
        format!("{}{}-{}ay", prefix, remaining, consonants.to_lowercase())
    };
    
    result_word
}

/// 将整个字符串转换为 Pig Latin
/// 保持单词之间的空格和标点符号
fn to_pig_latin(text: &str) -> String {
    // 使用正则表达式可能更好，但这里用简单的分割方法
    // 按空格分割，但保持其他标点符号
    let words: Vec<&str> = text.split_whitespace().collect();
    
    words
        .iter()
        .map(|word| {
            // 处理单词中的标点符号
            let mut result = String::new();
            let mut current_word = String::new();
            
            for c in word.chars() {
                if c.is_alphabetic() || c == '\'' {
                    // 字母、撇号属于单词的一部分
                    current_word.push(c);
                } else if c == '-' {
                    // 连字符：先处理当前单词，然后加连字符
                    if !current_word.is_empty() {
                        result.push_str(&word_to_pig_latin(&current_word));
                        current_word.clear();
                    }
                    result.push(c);
                } else {
                    // 遇到其他标点符号，处理当前单词
                    if !current_word.is_empty() {
                        result.push_str(&word_to_pig_latin(&current_word));
                        current_word.clear();
                    }
                    result.push(c);
                }
            }
            
            // 处理最后的单词部分
            if !current_word.is_empty() {
                result.push_str(&word_to_pig_latin(&current_word));
            }
            
            if result.is_empty() {
                word.to_string()
            } else {
                result
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    println!("=== Pig Latin 转换器 ===\n");
    
    // 测试用例
    let test_cases = vec![
        "first",
        "apple", 
        "school",
        "hello world",
        "The quick brown fox",
        "I love programming",
        "café résumé", // UTF-8 测试
        "hello, world!",
        "don't stop",
        "well-known",
        "123 numbers",
        "émigré naïve", // 更多 UTF-8 测试
    ];
    
    for test in test_cases {
        let result = to_pig_latin(test);
        println!("原文: \"{}\"", test);
        println!("转换: \"{}\"", result);
        println!();
    }
    
    // 交互式测试
    println!("=== 交互式测试 ===");
    println!("输入一些文本来测试转换（输入空行退出）:");
    
    loop {
        use std::io::{self, Write};
        
        print!("请输入: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    break;
                }
                
                let result = to_pig_latin(input);
                println!("转换结果: \"{}\"", result);
                println!();
            }
            Err(error) => {
                println!("读取输入时出错: {}", error);
                break;
            }
        }
    }
    
    println!("再见！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_vowel() {
        assert!(is_vowel('a'));
        assert!(is_vowel('E'));
        assert!(is_vowel('i'));
        assert!(is_vowel('O'));
        assert!(is_vowel('u'));
        assert!(!is_vowel('b'));
        assert!(!is_vowel('Z'));
    }

    #[test]
    fn test_word_to_pig_latin_vowel_start() {
        assert_eq!(word_to_pig_latin("apple"), "apple-hay");
        assert_eq!(word_to_pig_latin("eat"), "eat-hay");
        assert_eq!(word_to_pig_latin("Apple"), "Apple-hay");
    }

    #[test]
    fn test_word_to_pig_latin_consonant_start() {
        assert_eq!(word_to_pig_latin("first"), "irst-fay");
        assert_eq!(word_to_pig_latin("hello"), "ello-hay");
        assert_eq!(word_to_pig_latin("school"), "ool-schay");
        assert_eq!(word_to_pig_latin("First"), "Irst-fay");
    }

    #[test]
    fn test_word_to_pig_latin_with_punctuation() {
        assert_eq!(word_to_pig_latin("don't"), "on't-day");
    }
    
    #[test]
    fn test_hyphenated_words() {
        assert_eq!(to_pig_latin("well-known"), "ell-way-own-knay");
    }

    #[test]
    fn test_to_pig_latin_sentence() {
        assert_eq!(
            to_pig_latin("hello world"),
            "ello-hay orld-way"
        );
        assert_eq!(
            to_pig_latin("The quick brown fox"),
            "E-thay ick-quay own-bray ox-fay"
        );
    }

    #[test]
    fn test_utf8_handling() {
        // 测试 UTF-8 字符
        assert_eq!(word_to_pig_latin("café"), "afé-cay");
        assert_eq!(word_to_pig_latin("émigré"), "émigré-hay");
    }

    #[test]
    fn test_empty_and_edge_cases() {
        assert_eq!(word_to_pig_latin(""), "");
        assert_eq!(word_to_pig_latin("123"), "123");
        assert_eq!(to_pig_latin(""), "");
    }
}