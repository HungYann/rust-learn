// 添加学生
#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    age: u32,
    grade: u32,
    class_id: u32,
}

#[derive(Debug)]
struct ClassInfo {
    id: u32,
    name: String,
    description: String,
}

struct StudentManager {
    students: Vec<Student>,
    classes: Vec<ClassInfo>,
}

impl StudentManager {
    // 创建一个新的StudentManager
    fn new() -> Self {
        StudentManager {
            students: Vec::new(),
            classes: Vec::new()
        }
    }

    // 创建一个新的学生
    fn create_student(&mut self, id: u32, name: &str, age: u32, grade: u32, class_id: u32) {
        let student = Student{
            id,
            name: name.to_string(),
            age,
            grade,
            class_id,
        };

        self.students.push(student);
    }

    // 创建一个新的班级
    fn create_class(&mut self, id: u32, name: &str, description: &str) {
        let class_info = ClassInfo{
            id,
            name: name.to_string(),
            description: description.to_string(),
        };
        self.classes.push(class_info);
    }

    // 根据id查找学生
    fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.iter().find(|s| s.id == id)
    }

    // 根据id查找班级
    fn get_class(&self, id: u32) -> Option<&ClassInfo> {
        self.classes.iter().find(|c| c.id == id)
    }

    // 更新学生信息
    fn update_student(&mut self, id: u32, name: &str, age: u32, grade: u32, class_id: u32) {
        if let Some(student) = self.students.iter_mut().find(|s| s.id == id) {
            student.name = name.to_string();
            student.age = age;
            student.grade = grade;
            student.class_id = class_id;
        }
    }

    // 更新班级信息
    fn update_class(&mut self, id: u32, name: &str, description: &str) {
        if let Some(class_info) = self.classes.iter_mut().find(|c| c.id == id) {
            class_info.name = name.to_string();
            class_info.description = description.to_string();
        }
    }

    // 根据id删除学生
    fn delete_student(&mut self, id: u32) {
        self.students.retain(|s| s.id != id);
    }

    // 根据id删除班级
    fn delete_class(&mut self, id: u32) {
        self.classes.retain(|c| c.id != id);
    }
}

fn main() {
    let mut student_manager = StudentManager::new();

    // 创建班级
    student_manager.create_class(1, "Rust课程" , "Rust学习课程");
    student_manager.create_class(2, "Move课程" , "Move学习课程");

    // 创建学生
    student_manager.create_student(1, "Lehy", 20, 90, 1);

    student_manager.create_student(2, "Bob", 21, 85, 1);

    student_manager.create_student(3, "Nancy", 20, 92, 2);

    // 查找学生和班级
    // 查找学生和班级
    if let Some(student) = student_manager.get_student(1) {
        println!("Found student: {:?}", student);
    }
    if let Some(class_info) = student_manager.get_class(1) {
        println!("Found class: {:?}", class_info);
    }

    // 更新学生和班级信息
    student_manager.update_student(2, "Bob Simith", 22, 88, 2);
    student_manager.update_class(1, "Rust课程", "Rust课程更新" );

    println!("更新数据后:");
    // 查找学生和班级
    // 查找学生和班级
    if let Some(student) = student_manager.get_student(2) {
        println!("Found student: {:?}", student);
    }
    if let Some(class_info) = student_manager.get_class(1) {
        println!("Found class: {:?}", class_info);
    }

    // 删除学生和班级
    student_manager.delete_student(2);
    student_manager.delete_class(2);
}