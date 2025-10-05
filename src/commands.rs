use tabled::{
    Table,
    settings::{
        Alignment, Color, Style,
        object::{Columns, Rows},
    },
};

use crate::class::{Class, Lesson};

pub struct Classes(pub Vec<Class>);

impl Classes {
    fn find(&self, name: &str) -> Option<&Class> {
        self.0.iter().find(|x| x.name == name)
    }

    fn find_mut(&mut self, name: &str) -> Option<&mut Class> {
        self.0.iter_mut().find(|x| x.name == name)
    }

    pub fn list_classes(&self) {
        let mut table = Table::new(&self.0);
        table.with(Style::modern());
        table.modify(Rows::first(), "Classes");

        println!("{table}");
    }

    pub fn list_lessons(&self, name: String) {
        match self.find(&name) {
            None => println!("No classes found with this name"),
            Some(class) => print_lessons(&class.lessons),
        };
    }

    pub fn new(&mut self, name: String) {
        self.0.push(Class::new(name));

        self.list_classes();
    }

    pub fn add(&mut self, name: String, lesson_name: String) {
        match self.find_mut(&name) {
            None => println!("No classes found with this name"),
            Some(class) => class.lessons.push(Lesson::new(lesson_name)),
        };

        self.list_lessons(name);
    }

    pub fn attended(&mut self, name: String, lesson_id: usize) {
        match self.find_mut(&name) {
            None => println!("No classes found with this name"),
            Some(class) => class.lessons[lesson_id].attended = true,
        };

        self.list_lessons(name);
    }

    pub fn read(&mut self, name: String, lesson_id: usize) {
        match self.find_mut(&name) {
            None => println!("No classes found with this name"),
            Some(class) => class.lessons[lesson_id].read = true,
        };

        self.list_lessons(name);
    }

    pub fn processed(&mut self, name: String, lesson_id: usize) {
        match self.find_mut(&name) {
            None => println!("No classes found with this name"),
            Some(class) => class.lessons[lesson_id].processed = true,
        };

        self.list_lessons(name);
    }
}

fn print_lessons(lessons: &Vec<Lesson>) {
    let mut table = Table::builder(lessons).index().build();
    table
        .with(Style::modern())
        .modify(Rows::first(), Color::BOLD)
        .modify(Columns::first(), Alignment::center())
        .modify(Columns::new(1..), Alignment::right());

    println!("{table}");
}
