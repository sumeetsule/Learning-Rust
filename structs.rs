/* 
struct Students {
    id:i32,
    name:String,
    course:String
}

fn main(){
    let s:Students = Students{
        id:10,
        name:String::from("Rachel"),
        course:String::from("DB")
    };

    println!("{},{},{}",s.id,s.name,s.course);

}*/


// Lets find out where the objects inside Struct are created.

/*
#[allow(dead_code)]
struct Students {
    id: i32,
    name: String,
    course: String,
}

fn main() {
    let s: Students = Students {
        id: 10,
        name: String::from("Rachel"),
        course: String::from("DB"),
    };

    println!(
        "id value -> {} \
    \naddress of s.id -> {:p} \
    \nAddress of s -> {:p} \
    \nAddress of s.name -> {:p} \
    \nAddress of s.course -> {:p} \
    \ns.name pointing to -> {:p} \
    \ns.course pointing to -> {:p}",
        s.id,
        &s.id,
        &s,
        &s.name,
        &s.course,
        s.name.as_ptr(),
        s.course.as_ptr()
    );
} */



// Struct Stack or Heap
/*
#[derive(Clone,Debug)]
struct Students {
    id: i32
}

fn main() {
    let s: Students = Students {
        id: 10
    };

    let s4 = s.clone();
    
    println!("{}",s.id);
    println!("{:?}",s4);
    
}*/




// Struct inside Vector
/*
#[derive(Debug, Clone)]
struct Students {
    id: i32,
    name: String,
    course: String,
}

fn main() {
    let s: Students = Students {
        id: 10,
        name: String::from("Rachel"),
        course: String::from("DB"),
    };

    let s1: Students = Students {
        id: 11,
        name: String::from("Monica"),
        course: String::from("DB"),
    };

    let s2: Students = Students {
        id: 12,
        name: String::from("Phoebe"),
        course: String::from("DB"),
    };

    let mut s_vec: Vec<Students> = Vec::new();

    s_vec.push(s);
    s_vec.push(s1);
    s_vec.push(s2);

    for v in s_vec.iter() {
         println!("{},{},{}", v.id, v.name, v.course);
    }
}*/


#[derive(Debug, Clone)]
struct Students {
    id: i32,
    name: String,
    course: String,
}

impl Students {
    
    // either use self or self: &Self it all means the same
    //fn get_student_details(self){
    
    fn get_student_details(self: &Self){
        println!("{}", self.id);
        println!("{}", self.name);
        println!("{}", self.course);
    }

}

fn main() {
    let s: Students = Students {
        id: 10,
        name: String::from("Rachel"),
        course: String::from("DB"),
    };
    
    let s1: Students = Students {
        id: 11,
        name: String::from("Monica"),
        course: String::from("DB"),
    };
    
    s.get_student_details();
    s1.get_student_details();
}    