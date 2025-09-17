use std::collections::HashMap;
use std::io::{self, Write};

/// 员工管理系统
/// 使用 HashMap 存储部门名称到员工列表的映射
/// 每个部门的员工列表使用 Vector 存储
#[derive(Debug)]
struct EmployeeManager {
    // key: 部门名称, value: 员工名称列表
    departments: HashMap<String, Vec<String>>,
}

impl EmployeeManager {
    /// 创建新的员工管理系统
    fn new() -> Self {
        EmployeeManager {
            departments: HashMap::new(),
        }
    }
    
    /// 向指定部门添加员工
    /// 如果部门不存在，会自动创建
    fn add_employee(&mut self, name: String, department: String) {
        let employees = self.departments.entry(department.clone()).or_insert_with(Vec::new);
        
        // 检查员工是否已存在
        if employees.contains(&name) {
            println!("员工 {} 已经在 {} 部门中了！", name, department);
        } else {
            employees.push(name.clone());
            println!("成功将 {} 添加到 {} 部门", name, department);
        }
    }
    
    /// 获取指定部门的所有员工（按字典序排列）
    fn get_department_employees(&self, department: &str) -> Option<Vec<String>> {
        if let Some(employees) = self.departments.get(department) {
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();
            Some(sorted_employees)
        } else {
            None
        }
    }
    
    /// 获取所有部门和员工（按字典序排列）
    fn get_all_employees(&self) -> HashMap<String, Vec<String>> {
        let mut result = HashMap::new();
        
        // 获取所有部门名称并排序
        let mut departments: Vec<String> = self.departments.keys().cloned().collect();
        departments.sort();
        
        // 为每个部门创建排序后的员工列表
        for department in departments {
            if let Some(employees) = self.departments.get(&department) {
                let mut sorted_employees = employees.clone();
                sorted_employees.sort();
                result.insert(department, sorted_employees);
            }
        }
        
        result
    }
    
    /// 获取所有部门名称
    fn get_departments(&self) -> Vec<String> {
        let mut departments: Vec<String> = self.departments.keys().cloned().collect();
        departments.sort();
        departments
    }
    
    /// 删除员工
    fn remove_employee(&mut self, name: &str, department: &str) -> bool {
        if let Some(employees) = self.departments.get_mut(department) {
            if let Some(pos) = employees.iter().position(|x| x == name) {
                employees.remove(pos);
                println!("成功从 {} 部门删除员工 {}", department, name);
                
                // 如果部门变空，可以选择是否删除部门
                if employees.is_empty() {
                    self.departments.remove(department);
                    println!("部门 {} 已无员工，已删除该部门", department);
                }
                return true;
            }
        }
        false
    }
}

/// 解析用户输入的命令
/// 支持格式：
/// - "Add [名字] to [部门]"
/// - "List [部门]" 或 "Show [部门]"
/// - "List all" 或 "Show all"
/// - "Remove [名字] from [部门]"
fn parse_command(input: &str) -> Option<Command> {
    let input = input.trim();
    let words: Vec<&str> = input.split_whitespace().collect();
    
    if words.is_empty() {
        return None;
    }
    
    match words[0].to_lowercase().as_str() {
        "add" => {
            // 解析 "Add Sally to Engineering" 格式
            if let Some(to_pos) = words.iter().position(|&word| word.to_lowercase() == "to") {
                if to_pos > 1 && to_pos < words.len() - 1 {
                    let name = words[1..to_pos].join(" ");
                    let department = words[to_pos + 1..].join(" ");
                    return Some(Command::AddEmployee { name, department });
                }
            }
        }
        "list" | "show" => {
            if words.len() >= 2 {
                if words[1].to_lowercase() == "all" {
                    return Some(Command::ListAll);
                } else {
                    let department = words[1..].join(" ");
                    return Some(Command::ListDepartment { department });
                }
            }
        }
        "remove" => {
            // 解析 "Remove Sally from Engineering" 格式
            if let Some(from_pos) = words.iter().position(|&word| word.to_lowercase() == "from") {
                if from_pos > 1 && from_pos < words.len() - 1 {
                    let name = words[1..from_pos].join(" ");
                    let department = words[from_pos + 1..].join(" ");
                    return Some(Command::RemoveEmployee { name, department });
                }
            }
        }
        "help" => return Some(Command::Help),
        "quit" | "exit" => return Some(Command::Quit),
        _ => {}
    }
    
    None
}

/// 命令枚举
#[derive(Debug, PartialEq)]
enum Command {
    AddEmployee { name: String, department: String },
    ListDepartment { department: String },
    ListAll,
    RemoveEmployee { name: String, department: String },
    Help,
    Quit,
}

/// 显示帮助信息
fn show_help() {
    println!("\n=== 员工管理系统帮助 ===");
    println!("支持的命令：");
    println!("  Add [姓名] to [部门]       - 向部门添加员工");
    println!("  List [部门]               - 查看指定部门的所有员工");
    println!("  List all                  - 查看所有部门的员工");
    println!("  Remove [姓名] from [部门]  - 从部门删除员工");
    println!("  Help                      - 显示此帮助信息");
    println!("  Quit 或 Exit              - 退出程序");
    println!("\n示例：");
    println!("  Add Sally to Engineering");
    println!("  Add Amir to Sales");
    println!("  List Engineering");
    println!("  List all");
    println!("  Remove Sally from Engineering");
    println!();
}

/// 显示所有员工
fn display_all_employees(manager: &EmployeeManager) {
    let all_employees = manager.get_all_employees();
    
    if all_employees.is_empty() {
        println!("公司目前没有任何员工。");
        return;
    }
    
    println!("\n=== 所有部门员工列表 ===");
    for (department, employees) in all_employees {
        println!("\n{}部门:", department);
        if employees.is_empty() {
            println!("  (暂无员工)");
        } else {
            for (i, employee) in employees.iter().enumerate() {
                println!("  {}. {}", i + 1, employee);
            }
        }
    }
    println!();
}

/// 显示部门员工
fn display_department_employees(manager: &EmployeeManager, department: &str) {
    match manager.get_department_employees(department) {
        Some(employees) => {
            println!("\n{}部门的员工：", department);
            if employees.is_empty() {
                println!("  (暂无员工)");
            } else {
                for (i, employee) in employees.iter().enumerate() {
                    println!("  {}. {}", i + 1, employee);
                }
            }
            println!();
        }
        None => {
            println!("部门 \"{}\" 不存在。", department);
            let departments = manager.get_departments();
            if !departments.is_empty() {
                println!("现有部门：{}", departments.join(", "));
            }
        }
    }
}

fn main() {
    let mut manager = EmployeeManager::new();
    
    println!("🏢 欢迎使用员工管理系统！");
    println!("输入 'help' 查看帮助信息");
    
    // 添加一些示例数据
    manager.add_employee("Alice Johnson".to_string(), "Engineering".to_string());
    manager.add_employee("Bob Smith".to_string(), "Engineering".to_string());
    manager.add_employee("Carol Williams".to_string(), "Sales".to_string());
    manager.add_employee("David Brown".to_string(), "Marketing".to_string());
    
    println!("\n已预加载一些示例员工数据。输入 'list all' 查看。\n");
    
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match parse_command(&input) {
                    Some(Command::AddEmployee { name, department }) => {
                        manager.add_employee(name, department);
                    }
                    Some(Command::ListDepartment { department }) => {
                        display_department_employees(&manager, &department);
                    }
                    Some(Command::ListAll) => {
                        display_all_employees(&manager);
                    }
                    Some(Command::RemoveEmployee { name, department }) => {
                        if !manager.remove_employee(&name, &department) {
                            println!("在 {} 部门中未找到员工 {}", department, name);
                        }
                    }
                    Some(Command::Help) => {
                        show_help();
                    }
                    Some(Command::Quit) => {
                        println!("感谢使用员工管理系统！再见！");
                        break;
                    }
                    None => {
                        println!("无法理解的命令。输入 'help' 查看帮助信息。");
                    }
                }
            }
            Err(error) => {
                println!("读取输入时出错: {}", error);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_employee() {
        let mut manager = EmployeeManager::new();
        manager.add_employee("Alice".to_string(), "Engineering".to_string());
        
        let employees = manager.get_department_employees("Engineering").unwrap();
        assert_eq!(employees, vec!["Alice"]);
    }

    #[test]
    fn test_add_multiple_employees() {
        let mut manager = EmployeeManager::new();
        manager.add_employee("Alice".to_string(), "Engineering".to_string());
        manager.add_employee("Bob".to_string(), "Engineering".to_string());
        manager.add_employee("Carol".to_string(), "Sales".to_string());
        
        let engineering = manager.get_department_employees("Engineering").unwrap();
        let sales = manager.get_department_employees("Sales").unwrap();
        
        assert_eq!(engineering, vec!["Alice", "Bob"]);
        assert_eq!(sales, vec!["Carol"]);
    }

    #[test]
    fn test_alphabetical_sorting() {
        let mut manager = EmployeeManager::new();
        manager.add_employee("Zoe".to_string(), "Engineering".to_string());
        manager.add_employee("Alice".to_string(), "Engineering".to_string());
        manager.add_employee("Bob".to_string(), "Engineering".to_string());
        
        let employees = manager.get_department_employees("Engineering").unwrap();
        assert_eq!(employees, vec!["Alice", "Bob", "Zoe"]);
    }

    #[test]
    fn test_get_all_employees() {
        let mut manager = EmployeeManager::new();
        manager.add_employee("Alice".to_string(), "Engineering".to_string());
        manager.add_employee("Bob".to_string(), "Sales".to_string());
        
        let all = manager.get_all_employees();
        assert_eq!(all.len(), 2);
        assert!(all.contains_key("Engineering"));
        assert!(all.contains_key("Sales"));
    }

    #[test]
    fn test_parse_command() {
        assert_eq!(
            parse_command("Add Sally to Engineering"),
            Some(Command::AddEmployee {
                name: "Sally".to_string(),
                department: "Engineering".to_string()
            })
        );
        
        assert_eq!(
            parse_command("List Engineering"),
            Some(Command::ListDepartment {
                department: "Engineering".to_string()
            })
        );
        
        assert_eq!(parse_command("List all"), Some(Command::ListAll));
        
        assert_eq!(
            parse_command("Remove Sally from Engineering"),
            Some(Command::RemoveEmployee {
                name: "Sally".to_string(),
                department: "Engineering".to_string()
            })
        );
    }

    #[test]
    fn test_parse_multi_word_names_and_departments() {
        assert_eq!(
            parse_command("Add John Smith to Human Resources"),
            Some(Command::AddEmployee {
                name: "John Smith".to_string(),
                department: "Human Resources".to_string()
            })
        );
    }

    #[test]
    fn test_remove_employee() {
        let mut manager = EmployeeManager::new();
        manager.add_employee("Alice".to_string(), "Engineering".to_string());
        manager.add_employee("Bob".to_string(), "Engineering".to_string());
        
        assert!(manager.remove_employee("Alice", "Engineering"));
        let employees = manager.get_department_employees("Engineering").unwrap();
        assert_eq!(employees, vec!["Bob"]);
        
        assert!(!manager.remove_employee("Charlie", "Engineering"));
    }
}