struct Parent {
    child: String,
    pub age: u32,
}

impl Parent {
    fn generate(child: String, age: u32) -> Parent {
        Parent { child, age }
    }
}

mod school {
    //struct need to be decided whether it will be public or not.
    pub struct Student {
        //Each field of struct need to decide its accessibility either.
        name: String,   //name is private.
        pub score: i32, // score is public.
    }

    impl Student {
        //Since score is private, module must provides a generator function to assign value for private field.
        pub fn generate(name: String) -> Student {
            Student { name, score: 0 }
        }

        pub fn print(&self) {
            println!("Name : {0}", self.name);
            println!("Score : {0}", self.score);
        }
    }

    //Enum also has accessibility.
    //Unlike other things, enum is public in default.
    pub enum Job {
        student,
        //enum struct are defined as public in this case.
        teacher { salary: u32, lecture: String },
        //Typed enum also defined as public.
        parent(super::Parent),
    }
}

fn main() {
    let mut alex = school::Student::generate(String::from("Alex"));
    alex.score = 10;
    // Impossible, name is not public field.
    // alex.name = String::from("Lee");
    alex.print();
    // Name : Alex
    // Score : 10

    let alex_job = school::Job::student;
    let teacher_job = school::Job::teacher {
        lecture: String::from("Physics"),
        salary: 20,
    };
    //Enum defined as public for all fields in it.

    //Even it exists by type, it is defined as public.
    let mut alex_mom = Parent::generate(String::from("Alex"), 30);
    let mut alex_mom_job = school::Job::parent(alex_mom);
    if let school::Job::parent(mut am) = alex_mom_job {
        //THis code is bit massy since it takes ownership of alex_mom_job to am.
        am.age = 40;
        am.child = String::from("No body");
        println!("{0} {1}", am.age, am.child);
        //40 No body
    }
}
