#[derive(Debug, Clone, Copy)]
pub struct Interval{
    min: u32,
    max: u32
}
impl Interval{
    pub fn from_string(string: &str) -> Interval{
        let mut split = string.split('-').into_iter();
        let left = split.next().unwrap().parse::<u32>().unwrap();
        let right = split.next().unwrap().parse::<u32>().unwrap();
        if left > right{
            println!("{}",string);
            panic!()
        }
        return Interval { min: left, max: right }
    }

    pub fn overlap_comletely(&self, other: Interval)-> bool{
        return other.min <= self.min && other.max >= self.max || self.min <= other.min && self.max >= other.max;
    }
    pub fn overlap(&self, other: Interval)-> bool{
        return self.min <= other.min && self.max >= other.min || self.min <= other.max && self.max >= other.max || self.min >= other.min && self.max <= other.max;
    }
}
#[derive(Debug, Clone,Copy)]
pub struct Folder{
    size_of_files: i64
}

impl Folder{
    pub fn create_folder() -> Folder{
       return Folder {size_of_files: 0};
    }
    pub fn add_file(&mut self,file_size: i64){
        self.size_of_files += file_size;
    }

    pub fn get_size(&self)->i64{
        return self.size_of_files;
    }

}
pub struct Monkey{
    items: Vec<i32>,
    test: Test
}

impl Monkey{
    fn new_monkey(new_item: Vec<i32>,  new_test: Test) -> Monkey{
        return Monkey { items: new_item, test: new_test }
    }

}
pub struct Test{
    divide_by: i32,
    if_true: i32,
    if_false: i32
}

impl Test{
    fn do_test(&self,input: i32) -> i32{
        if input % self.divide_by == 0{
            return self.if_true;
        }
        return self.if_false;
    }
}