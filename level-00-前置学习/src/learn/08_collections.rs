// ============================================
// 08 - 常见集合及操作 (Common Collections)
// ============================================
// 集合将多个值存放在一个数据结构中

pub fn run() {
    println!("===== 08 常见集合及操作 =====\n");

    // ==================== Vec<T> - 动态数组 ====================
    println!("--- Vec ---");

    // 创建 Vec
    let _v1: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3]; // 使用宏创建

    // 可变 Vec
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("Vec: {:?}", v);

    // 读取元素
    let third = &v[2]; // 使用索引，越界会 panic
    println!("第三个元素: {}", third);

    match v.get(2) { // 使用 get，越界返回 None
        Some(value) => println!("第三个元素: {}", value),
        None => println!("没有第三个元素"),
    }

    // 遍历 Vec
    for i in &v {
        println!("遍历: {}", i);
    }

    // 遍历并修改
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i *= 2;
    }
    println!("修改后: {:?}", v);

    // 存储不同类型的值（使用枚举）
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Vec 常用方法
    let mut v = vec![1, 2, 3];
    v.pop(); // 移除并返回最后一个元素
    v.insert(1, 10); // 在索引1处插入
    v.remove(0); // 移除索引0处的元素
    v.extend([4, 5, 6]); // 追加多个元素
    println!("操作后: {:?}", v);
    println!("长度: {}, 容量: {}", v.len(), v.capacity());
    v.clear(); // 清空
    println!("清空后: {:?}", v);

    // ==================== String - 字符串 ====================
    println!("\n--- String ---");

    // 创建 String
    let mut s = String::new();
    let _s2 = "initial contents".to_string();
    let _s3 = String::from("initial contents");

    // 追加字符串
    s.push_str("foo");
    s.push_str("bar");
    s.push('!'); // 追加单个字符
    println!("追加后: {}", s);

    // 连接字符串
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 被移动，不能再使用
    println!("连接: {}", s3);

    // 使用 format! 宏（不获取所有权）
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("format!: {}", s);
    println!("s1 仍然可用: {}", s1);

    // 字符串切片
    let s = String::from("hello");
    let h = &s[0..1]; // 注意：切片按字节，不是按字符
    println!("切片: {}", h);

    // 遍历字符
    for c in "Здравствуйте".chars() {
        println!("字符: {}", c);
    }

    // 遍历字节
    for b in "Здравствуйте".bytes() {
        println!("字节: {}", b);
    }

    // String 常用方法
    let s = String::from("Hello, World!");
    println!("包含 'World': {}", s.contains("World"));
    println!("替换: {}", s.replace("World", "Rust"));
    println!("转大写: {}", s.to_uppercase());
    println!("转小写: {}", s.to_lowercase());
    println!(" trim: '{}'", "  hello  ".trim());
    println!("是否以 'Hello' 开头: {}", s.starts_with("Hello"));
    println!("是否以 '!' 结尾: {}", s.ends_with("!"));

    // 按空白分割
    let s = String::from("hello world foo bar");
    for word in s.split_whitespace() {
        println!("单词: {}", word);
    }

    // 按特定字符分割
    let s = "hello,world,foo,bar";
    for part in s.split(',') {
        println!("部分: {}", part);
    }

    // ==================== HashMap<K, V> - 哈希映射 ====================
    println!("\n--- HashMap ---");

    use std::collections::HashMap;

    // 创建 HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 获取值
    let team_name = String::from("Blue");
    match scores.get(&team_name) {
        Some(score) => println!("{} 队的分数: {}", team_name, score),
        None => println!("未找到 {} 队", team_name),
    }

    // 遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 更新值
    scores.insert(String::from("Blue"), 25); // 覆盖
    println!("更新后: {:?}", scores);

    // 只在键不存在时插入
    scores.entry(String::from("Yellow")).or_insert(100);
    scores.entry(String::from("Green")).or_insert(100);
    println!("entry 后: {:?}", scores);

    // 基于现有值更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("单词计数: {:?}", map);

    // HashMap 常用方法
    println!("长度: {}", scores.len());
    println!("是否包含 'Blue': {}", scores.contains_key("Blue"));
    scores.remove("Blue");
    println!("移除后: {:?}", scores);

    // ==================== HashSet<T> - 哈希集合 ====================
    println!("\n--- HashSet ---");

    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(2); // 重复值不会被插入
    println!("Set: {:?}", set);

    // 集合运算
    let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let set2: HashSet<i32> = [2, 3, 4].iter().cloned().collect();

    // 交集
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    println!("交集: {:?}", intersection);

    // 并集
    let union: HashSet<_> = set1.union(&set2).collect();
    println!("并集: {:?}", union);

    // 差集
    let difference: HashSet<_> = set1.difference(&set2).collect();
    println!("差集: {:?}", difference);

    // 对称差集
    let symmetric_diff: HashSet<_> = set1.symmetric_difference(&set2).collect();
    println!("对称差集: {:?}", symmetric_diff);

    // ==================== VecDeque<T> - 双端队列 ====================
    println!("\n--- VecDeque ---");

    use std::collections::VecDeque;

    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    println!("Deque: {:?}", deque);

    if let Some(front) = deque.pop_front() {
        println!("弹出队首: {}", front);
    }
    if let Some(back) = deque.pop_back() {
        println!("弹出队尾: {}", back);
    }

    // ==================== LinkedList<T> - 链表 ====================
    println!("\n--- LinkedList ---");

    use std::collections::LinkedList;

    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(0);
    println!("LinkedList: {:?}", list);

    // ==================== BinaryHeap<T> - 二叉堆 ====================
    println!("\n--- BinaryHeap ---");

    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new();
    heap.push(3);
    heap.push(1);
    heap.push(5);
    heap.push(2);
    println!("堆顶: {:?}", heap.peek());

    while let Some(top) = heap.pop() {
        println!("弹出: {}", top);
    }

    // ==================== BTreeMap / BTreeSet ====================
    println!("\n--- BTreeMap ---");

    use std::collections::BTreeMap;

    let mut btree = BTreeMap::new();
    btree.insert(3, "c");
    btree.insert(1, "a");
    btree.insert(2, "b");

    for (key, value) in &btree {
        println!("{}: {}", key, value); // 按键排序输出
    }

    println!();
}
