fn main() {
    println!("Hello, world!");
    let _v: Vec<i32> = Vec::new();
    // infer type of _v
    let _v = vec![1, 2, 3];

    // infer type of v from v.push(5);
    let mut v = Vec::new();
    // need mut
    // let v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v: {:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    let _third: &i32 = &v[2];
    let _third: Option<&i32> = v.get(3);

    // panic!
    // let does_not_exists = &v[100];
    let does_not_exists = v.get(100);
    println!("v.get(100): {:?}", does_not_exists);

    let mut v = vec![1, 2, 3, 4, 5];
    let _first = &v[0];
    // cannot borrow v as mutable, v is already borrowed as immutable
    // pushing new element to vector may needs reallocation
    // and copying old elements to new space, old reference can be invalid.
    // v.push(6);

    let v = vec![100, 32, 57];
    for i in v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(3.0),
    ];
    println!("row: {:?}", row);

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);

    scores.insert(String::from("Blue"), 99);
    println!("scores: {:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores: {:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // let mut count = 0;
    // count += 1;
    // println!("count: {:?}", count);

    // let mut count = &0;
    // count += 1;
    // println!("count: {:?}", count);

    let l = vec![1, 2, 3, 4, 5];
    println!("{:?} -> average: {:?}", l, average(&l));
    let l = vec![1, 2, 3, 4, 5];
    println!("{:?} -> median: {:?}", l, median(&l));
    let l = vec![1, 2, 3, 4, 5, 6];
    println!("{:?} -> median: {:?}", l, median(&l));
    let l = vec![1, 2, 3, 4, 5, 3];
    println!("{:?} -> mode: {:?}", l, mode(&l));

    println!("apple -> {:?}", str_to_pig_latin("apple"));
    println!("first -> {:?}", str_to_pig_latin("first"));

    company::main();
}

fn average(l: &Vec<i32>) -> f32 {
    let mut sum = 0.0;

    for i in l.into_iter() {
        sum += *i as f32;
    }

    return sum / l.len() as f32;
}

fn median(l: &Vec<i32>) -> f32 {
    if l.len() % 2 == 0 {
        let middle = l.len() / 2;
        let ll = &l[middle - 1..middle + 1];
        let sum = ll[0] + ll[1];
        return sum as f32 / 2.0;
    } else {
        return l[l.len() / 2] as f32;
    }
}

fn mode(l: &Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    for i in l.into_iter() {
        let mut count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let (mode, _) = map
        .iter()
        .max_by(|&x, &y| {
            let (_, v1) = x;
            let (_, v2) = y;
            return v1.cmp(v2);
        })
        .unwrap();

    return **mode;
}

fn str_to_pig_latin(s: &str) -> Option<String> {
    let mut chars = s.chars();

    match chars.next() {
        Some(a) => if a == 'a' {
            return Some(format!("{:}{:}-hay", a, chars.as_str()));
        } else {
            return Some(format!("{:}-{:}ay", chars.as_str(), a));
        },
        None => None,
    }
}

mod company {
    use std::collections::HashMap;
    type Dic = HashMap<String, Vec<String>>;

    pub fn main() {
        let mut dic = HashMap::new();
        run("Add a to 1", &mut dic);
        run("Add b to 2", &mut dic);
        run("Add c to 1", &mut dic);
        print_members("1", &dic);
        print_all_members(&dic);
    }

    fn run(s: &str, dic: &mut Dic) {
        let mut itr = s.split_whitespace();
        let verb = itr.next();

        match verb {
            Some(a) => if a == "Add" {
                let name = itr.next();
                let _ = itr.next();
                let department = itr.next();
                match department {
                    Some(d) => match name {
                        Some(n) => {
                            let members = dic.entry(String::from(d)).or_insert(vec![]);
                            members.push(String::from(n));
                        }
                        None => {}
                    },
                    None => {}
                }
            } else {
            },
            None => {}
        }
    }

    fn print_members(dep: &str, dic: &Dic) {
        match dic.get(dep) {
            Some(members) => {
                println!("In {:?} department: ", dep);
                let mut members = members.clone();
                &members.sort();
                for member in members.iter() {
                    println!("{:?}", member);
                }
            }
            None => println!("No members in {:?}", dep),
        }
    }

    fn print_all_members(dic: &Dic) {
        let mut all_members: Vec<(&String, &String)> = vec![];

        for (dep, members) in dic.iter() {
            for member in members.iter() {
                all_members.push((member, dep));
            }
        }

        &all_members.sort_by(|&a, &b| {
            let (name1, _) = a;
            let (name2, _) = b;
            return name1.cmp(name2);
        });

        for &(name, dept) in all_members.iter() {
            println!("{:?}: {:?}", name, dept);
        }
    }
}
