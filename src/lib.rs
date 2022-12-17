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
    size_of_files: u64
}

impl Folder{
    pub fn create_folder() -> Folder{
       return Folder {size_of_files: 0};
    }
    pub fn add_file(&mut self,file_size: u64){
        self.size_of_files += file_size;
    }

    pub fn get_size(&self)->u64{
        return self.size_of_files;
    }

}
#[derive(Debug, Clone)]
pub struct Monkey{
    pub items: Vec<u64>,
    pub value: u64,
    pub function: fn(x: u64, y: u64) -> u64,
    pub test: Test
}
#[derive(PartialEq)]
pub enum Operator {
    Add,
    Multi
}
#[derive(PartialEq)]
pub enum OtherOperator {
    Value,
    Me
}

impl Monkey{
    pub fn inspect(&self, worry_level: u64)-> u64{
        return (self.function)(worry_level,self.value);
    }
    pub fn get_target(&self, worry_level: u64)-> i32{
        return self.test.get_target(worry_level);
    }
    pub fn clear_items(&mut self){
        self.items.clear();
    }
    pub fn add_item(&mut self, item: u64){
        self.items.push(item);
    }
}
#[derive(Debug, Clone)]
pub struct Test{
    pub divide_by: u64,
    pub if_true: i32,
    pub if_false: i32
}

impl Test{
    fn get_target(&self, input: u64) -> i32{
        if input % self.divide_by == 0{
            return self.if_true;
        }
        return self.if_false;
    }
}