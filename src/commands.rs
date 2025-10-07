use tabled::{
    Table,
    settings::{
        Alignment, Color, Style,
        object::{Columns, Rows},
    },
};

use crate::{
    class::{Class, Lesson},
    cli::ClassAction,
};

pub struct Classes(pub Vec<Class>);

type StringErr = Result<(), String>;

impl Classes {
    fn find(&self, name: &str) -> Result<&Class, String> {
        self.0
            .iter()
            .find(|x| x.name == name)
            .ok_or("No classes found with this name".into())
    }

    fn find_mut(&mut self, name: &str) -> Result<&mut Class, String> {
        self.0
            .iter_mut()
            .find(|x| x.name == name)
            .ok_or("No classes found with this name".into())
    }

    pub fn list_classes(&self) -> StringErr {
        let mut table = Table::new(&self.0);
        table.with(Style::modern());
        table.modify(Rows::first(), "Classes");

        println!("{table}");
        Ok(())
    }

    pub fn list_lessons(&self, name: &str) -> StringErr {
        let class = self.find(name)?;
        print_lessons(&class.lessons);

        Ok(())
    }

    pub fn new(&mut self, name: String) -> StringErr {
        self.0.push(Class::new(name));

        self.list_classes()?;
        Ok(())
    }

    pub fn add(
        &mut self,
        name: String,
        lesson_name: String,
        done: Option<String>,
        week: u32,
    ) -> StringErr {
        let class = self.find_mut(&name)?;
        let mut lesson = Lesson::new(lesson_name, week);

        let done = match done {
            None => "".to_string(),
            Some(str) => str,
        };

        if done.contains("a") {
            lesson.attended = true;
        }
        if done.contains("u") {
            lesson.understood = true;
        }
        if done.contains("p") {
            lesson.processed = true;
        }

        class.lessons.push(lesson);

        print_lessons(&class.lessons);
        Ok(())
    }

    pub fn attended(&mut self, name: String, lesson_id: usize) -> StringErr {
        let class = self.find_mut(&name)?;
        class.lessons[lesson_id].attended = true;

        print_lessons(&class.lessons);
        Ok(())
    }

    pub fn understood(&mut self, name: String, lesson_id: usize) -> StringErr {
        let class = self.find_mut(&name)?;
        class.lessons[lesson_id].understood = true;

        print_lessons(&class.lessons);
        Ok(())
    }

    pub fn processed(&mut self, name: String, lesson_id: usize) -> StringErr {
        let class = self.find_mut(&name)?;
        class.lessons[lesson_id].processed = true;

        print_lessons(&class.lessons);
        Ok(())
    }

    pub fn remove(&mut self, name: String, lesson_id: usize) -> StringErr {
        let class = self.find_mut(&name)?;
        class.lessons.remove(lesson_id);

        print_lessons(&class.lessons);
        Ok(())
    }

    pub fn edit(&mut self, name: String, action: ClassAction) -> StringErr {
        if let ClassAction::Edit {
            lesson_id_range,
            description,
            week,
        } = action
        {
            let class = self.find_mut(&name)?;
            for lesson_id in lesson_id_range {
                let lesson = class
                    .lessons
                    .get_mut(lesson_id)
                    .ok_or("No lesson with that id".to_string())?;

                if let Some(ref description) = description {
                    lesson.description = description.join(" ");
                }

                if let Some(week) = week {
                    lesson.week = week;
                }
            }
            print_lessons(&class.lessons);
        }

        Ok(())
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
