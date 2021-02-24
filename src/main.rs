use std::{fs, io};
use serde::{Serialize, Deserialize};
use ron::de::from_str;
use ron::ser::to_string;

fn main() {
    println!("Dividing Head Calculator     A. Kay 2020 Version 1.0.0");
    let mut settings : Settings;
    if fs::metadata("setup.ron").is_ok(){
        let ronstring = fs::read_to_string("setup.ron")
        .expect("setting file not found");
        settings = from_str(&ronstring[..]).expect("failed to extract stetting from ron");
    }else{
        settings = Settings::new();
        first_time_load(&mut settings);
        save_settings(&settings);
    }
    println!("Enter ? for help");
    loop {
        println!("Enter A Num Or Com Char");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed To Get Input");
    let input = input.trim();
    match input.chars().next().unwrap() {
        'a' => add(&input, &mut settings),
        'r' => remove(&input, &mut settings),
        'c' => clear(),
        'R' => update_r(&input, &mut settings),
        'l' => list(&settings),
        'f' => settings.first = !settings.first,
        'A' => change_acc(&input, &mut settings),
        'm' => change_min(&input, &mut settings),
        'M' => change_max(&input, &mut settings),
        's' => println!("Settings are {:#?}", settings),
        'p' => change_page_size(&input, &mut settings),
        '?' => help(),
        '0' | '1' | '2' | '3'| '4'| '5'| '6'| '7'| '8'| '9' => {match input.trim().parse() {
            Ok(val) => find(val, &settings),
            Err(_) => {println!("{} is not a number",input); continue;}
        };},
        _ => println!("Enter Num or a VALID Com Char"),
    };
}
}


fn add(input : &str,settings : &mut Settings){
    let input_words = input.split(' ');
    let mut to_add = Vec::new();
    for word in input_words{
        match word.chars().next().unwrap() {
            '0' | '1' | '2' | '3'| '4'| '5'| '6'| '7'| '8'| '9' => {match word.parse() {
                Ok(val) => to_add.push(val),
                Err(_) => {println!("{} contains more than just numbers, skipping it.",word);
                continue;}
            };},
            _ => continue,
        };
    }
    if to_add.len() < 1{
        println!("Failed to find values to set");
        return;
    }
    settings.plates.push(to_add);
    save_settings(&settings);
}

fn remove(input: &str, settings: &mut Settings) {
    if settings.plates.len() == 1{
        println!("Only one plate; can not remove last plate!");
    }
    let input_words = input.split(' ');
    for word in input_words{
        match word.chars().next().unwrap() {
            '0' | '1' | '2' | '3'| '4'| '5'| '6'| '7'| '8'| '9' => {match word.parse() {
                Ok(val) => {
                    if settings.plates.len()>val {
                    settings.plates.remove(val);}
                    else{println!("{} is not a valid plate ID use 'l' command to list all plates and there ID", val)}},
                Err(_) => {println!("{} contains more than just numbers, skipping it.",word);
                continue;}
            };},
            _ => continue,
        };
    }
    save_settings(&settings);
}

fn clear(){
    todo!();
}

fn change_page_size(input: &str, settings: &mut Settings) {
    let input_words = input.split(' ');
    for word in input_words{
        match word.chars().next().unwrap() {
            '0' | '1' | '2' | '3'| '4'| '5'| '6'| '7'| '8'| '9' => {match word.parse() {
                Ok(val) => {
                    settings.page_size = val;
                    println!("Set Min to {}",val);
                    save_settings(&settings);
                    return;},
                Err(_) => {println!("{} contains more than just numbers, Trying Next.",word);
                continue;}
            };},
            _ => continue,
        };
    }
    println!("Failed to find value to set");
}

fn change_max(input: &str, settings: &mut Settings) {
    let input_words = input.split(' ');
    for word in input_words{
        match word.chars().next().unwrap() {
            '0' | '1' | '2' | '3'| '4'| '5'| '6'| '7'| '8'| '9' => {match word.parse() {
                Ok(val) => {
                    settings.max = val;
                    println!("Set Min to {}",val);
                    save_settings(&settings);
                    return;},
                Err(_) => {println!("{} contains more than just numbers, Trying Next.",word);
                continue;}
            };},
            _ => continue,
        };
    }
    println!("Failed to find value to set");
}

fn change_min(input: &str, settings: &mut Settings) {
    let input_words = input.split(' ');
    for word in input_words{
        match word.chars().next().unwrap() {
            '0' | '1' | '2' | '3'| '4'| '5'| '6'| '7'| '8'| '9' => {match word.parse() {
                Ok(val) => {
                    settings.min = val;
                    println!("Set Min to {}",val);
                    save_settings(&settings);
                    return;},
                Err(_) => {println!("{} contains more than just numbers, Trying Next.",word);
                continue;}
            };},
            _ => continue,
        };
    }
    println!("Failed to find value to set");
}

fn change_acc(input: &str, settings: &mut Settings) {
    let input_words = input.split(' ');
    for word in input_words{
        match word.chars().next().unwrap() {
            '0' | '1' | '2' | '3'| '4'| '5'| '6'| '7'| '8'| '9' => {match word.parse() {
                Ok(val) => {
                    let val : f32 = val;
                    settings.acc = val.abs();
                    println!("Set Accuracy to {}",settings.acc);
                    save_settings(&settings);
                    return;},
                Err(_) => {println!("{} contains more than just numbers, Trying Next.",word);
                continue;}
            };},
            _ => continue,
        };
    }
    println!("Failed to fine value to set");
}

fn list(settings: &Settings) {
    println!("You Have {} plates loaded they are", settings.plates.len());
    for (i, plate) in settings.plates.iter().enumerate(){
        println!("ID:{} = {:?}", i,plate);
    }
}

fn update_r(input: &str, settings: &mut Settings) {
    let input_words = input.split(' ');
    for word in input_words{
        match word.chars().next().unwrap() {
            '0' | '1' | '2' | '3'| '4'| '5'| '6'| '7'| '8'| '9' => {match word.parse() {
                Ok(val) => {
                    settings.r = val;
                    println!("Set R to {}",val);
                    save_settings(&settings);
                    return;},
                Err(_) => {println!("{} contains more than just numbers, Trying Next.",word);
                continue;}
            };},
            _ => continue,
        };
    }
    println!("Failed to find value to set");
}

#[derive(Serialize, Deserialize, Debug)]
struct Settings{
    r : u32,
    plates : Vec<Vec<u32>>,
    first : bool,
    acc : f32,
    min : u8,
    max : u8,
    page_size : u8,
}

impl Settings {
    const fn new() -> Settings{
        Settings{
            r : 0,
            acc: 0.01,
            min : 1,
            max : 10,
            page_size : 100,
            first : false,
            plates : Vec::new()
        }
    }
}

fn find(divisions : u32, settings : &Settings){
    let mut found_list :Vec<PrintData> = Vec::new();
    if settings.r as f32 / divisions as f32 % 1.0 == 0.0{
        println!("Found Exact matches");
        println!("┌─────┬────────────────────┬─────┬───────────┬──────────┐");
        PrintData{
            target : divisions,
            pnod : divisions as f32,
            laps : 1,
            movement : (settings.r / divisions).to_string(),
        }.print();
        println!("└─────┴────────────────────┴─────┴───────────┴──────────┘");
        return;
    }
    for x in settings.min..settings.max{
        if !no_loops_onx(divisions, x as u32){
            continue;
        }
        for plate in settings.plates.iter(){
            for &hole_count in plate{
                let steps = settings.r as f32 / divisions as f32;
                let steps = steps * x as f32;
                let full = (steps - (1.0 / hole_count as f32) * x as f32).floor();
                let holes = (steps - full as f32) * hole_count as f32;
                let find = PrintData{
                    target : divisions,
                    pnod : settings.r as f32 / (full as f32 + (holes.floor() / hole_count as f32)) * x as f32,
                    laps : x,
                    movement : format!("{} & {}/{}", full, holes as u32, hole_count),
                };
                let exc = find.error() == 0.0;
                if find.error() < settings.acc {
                    found_list.push(find);
                }
                if exc {continue;}
                for &sec_hole_count in plate{
                    if sec_hole_count == hole_count {
                        continue;
                    }
                    let mut small_holes = holes;
                    let big_holes = small_holes.floor();
                    small_holes -= big_holes;
                    let small_holes = small_holes / hole_count as f32;
                    let dif = (1.0 / hole_count as f32) - (1.0 / sec_hole_count as f32);
                    let mut moves = 0.0;
                    let mut target = 0.0;
                    for _ in 0..hole_count{
                        let test = settings.r as f32 / (full as f32 + ((big_holes - moves) / hole_count as f32) + (((((dif * moves)+small_holes)/(1.0/sec_hole_count as f32)) + moves).round() / sec_hole_count as f32)) * x as f32;
                        target = (test - divisions as f32).abs();
                        if target <= settings.acc {
                            break;
                        }
                        moves += 1.0;
                    }
                    let sec_holes = ((dif * moves) + small_holes).round() / (1.0 / sec_hole_count as f32);
                    {
                        let mut big_holes = big_holes - moves;
                        let mut full = full + (big_holes/hole_count as f32).floor();
                        while big_holes < 0.0 {
                            big_holes += hole_count as f32;
                            full -= 1.0;
                        }
                        let mut sec_holes = sec_holes + moves;
                        while sec_holes >= sec_hole_count as f32 {
                            sec_holes -= sec_hole_count as f32;
                            full += 1.0;
                        }
                        //println!("sec_holes = {}, moves = {}", small_holes, moves);
                        let find = PrintData{
                            target : divisions,
                            pnod : settings.r as f32 / (full + (big_holes/hole_count as f32) + (sec_holes / sec_hole_count as f32)) * x as f32,
                            laps : x,
                            movement : format!("{} & {}/{} + {}/{}", full, big_holes as u32, hole_count, sec_holes, sec_hole_count),
                        };
                        //println!("{:#?} and error = {}",find, find.error());
                        if find.error() <= settings.acc {
                            //println!("pushing {:#?}", find);
                            found_list.push(find);
                        }
                    }
                    moves = 0.0;
                    let mut small_holes = holes;
                    let big_holes = holes.floor();
                    small_holes -= big_holes;
                    small_holes /= hole_count as f32;
                    //full = (steps - (1.0 / hole_count as f32) * i as f32).floor();
                    for _ in  0..hole_count{
                        let test = settings.r as f32 / (full as f32 + (((big_holes + moves) / hole_count as f32) - (((-(small_holes - (dif * moves))/(1.0/sec_hole_count as f32)) + moves).round() / sec_hole_count as f32))) * x as f32;
                            if false {
                                let a = big_holes + moves;
                                let c = dif * moves;
                                let d = 1.0/sec_hole_count as f32;
                                let e = small_holes - c;
                                let f = -e/d;
                                let g =(f + moves).round();
                                println!("test'{test}' = settings.r as f32 / (full as f32 + (((big_holes + moves) / hole_count as f32) - (((-(small_holes - (dif * moves))/(1.0/sec_hole_count as f32)) + moves).round() / sec_hole_count as f32))) * x as f32", test = settings.r as f32 / (full as f32 + (((big_holes + moves) / hole_count as f32) - (((-(small_holes - (dif * moves))/(1.0/sec_hole_count as f32)) + moves).round() / sec_hole_count as f32))) * x as f32);
                                println!("test'{test}' = {r} / ({full} + (({a} / {hole_count}) - ({g} / {sec_hole_count}))) * {x}",a = a, hole_count=hole_count,g= g,x = x,test=test,r = settings.r,full = full, sec_hole_count = sec_hole_count);
                            }
                        target = (test-divisions as f32).abs();
                        if target <= settings.acc {
                            //println!("found target {}",target);
                            break;
                        }
                        moves += 1.0;
                    }
                    if target <= settings.acc
                    {
                        let mut sec_holes =((-(small_holes - (dif * moves))/(1.0/sec_hole_count as f32)) + moves).round();
                        let mut big_holes = big_holes + moves;
                        let mut full = full;
                        //sec_holes += moves;
                        //println!("target {} = {} & {}/{} - {}/{} * {}",target, full, big_holes, hole_count, sec_holes, sec_hole_count, x);
                        while big_holes >= hole_count as f32 {
                            big_holes -= hole_count as f32;
                            full+=1.0;
                        }
                        while sec_holes > sec_hole_count as f32 {
                            sec_holes -= sec_hole_count as f32;
                            full -= 1.0;
                        }
                        //println!("target {} = {} & {}/{} - {}/{}",target, full, big_holes, hole_count, sec_holes, sec_hole_count);
                        let find = PrintData{
                            target : divisions,
                            pnod : settings.r as f32 / (full + (big_holes/hole_count as f32) - (sec_holes / sec_hole_count as f32)) * x as f32,
                            laps : x,
                            movement : format!("{} & {}/{} - {}/{}", full, big_holes as u32, hole_count, sec_holes, sec_hole_count),
                        };
                        if find.error() <= settings.acc {
                            found_list.push(find);
                        }
                    }
                }
            }
        }
    }

    if found_list.len() < 1{
        println!("No Matches found for setting : {:#?}", settings);
        println!("Note: try lowering the accuracy or raising max work laps");
        return;
    }
    println!("Found {} possible matches", found_list.len());
    println!("┌─────┬────────────────────┬─────┬───────────┬──────────┐");
    for (i,found) in PrintData::sort(found_list).iter().enumerate(){
        if i > settings.page_size as usize {
            println!("└─────┴────────────────────┴─────┴───────────┴──────────┘");
            println!("Stopped at Page Limit - Found More;\nUse 'p' to change page size and see them");
            return;
        }
        found.print();
        if settings.first{
            break;
        }
    }
    println!("└─────┴────────────────────┴─────┴───────────┴──────────┘");
}

fn first_time_load(settings : &mut Settings){
    println!("First Time Setup Required");
    let mut input = String::new();
    println!("Enter Ratio");
    while settings.r == 0 {
        io::stdin().read_line(&mut input).expect("Failed To Get Input");
        settings.r = match input.trim().parse() {
            Ok(val) => val,
            Err(_) => {println!("Please enter a positive number"); continue;}
        };
    }
    println!("Enter Plate Numbers separated by ' '");
    while settings.plates.len() == 0 {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed To Get Input");
        let platet = input.trim().split(' ');
        let mut holes: Vec<u32> = Vec::new();
        for hole in platet{
            let hole: u32 = match hole.trim().parse() {
                Ok(val) => val,
                Err(_) => {println!("{} is not a number", hole);continue;}
            };
            holes.push(hole);
        }
        if holes.len() > 1 {
        settings.plates.push(holes);
        } 
        else {println!("Please enter at least 2 numbers separated by ' '")}
    }
}
#[derive(Debug)]
struct PrintData
    {
        target : u32,
        movement : String,
        laps : u8,
        pnod : f32
    }
    impl PrintData {
        fn print(&self){
            if self.error() == 0.0 {
                println!("│{:05}│{:^20}│{:^5}│{:^11}│{:^10}│",self.target, self.movement, self. laps, self.pnod, "Ext");
            }else{
                println!("│{:05}│{:^20}│{:^5}│{:^11}│{:^10}│",self.target, self.movement, self. laps, self.pnod, 0.001 / self.error() * self.target as f32 / 3.14159265359);
            }
        }
        fn sort(original : Vec<PrintData>) -> Vec<PrintData>{
            let mut output:Vec<PrintData> = Vec::new();
            for test in original{
                if output.len() < 1 {
                    output.push(test);
                    continue;
                }
                for (i, data) in output.iter().enumerate() {
                    if test.error() < data.error() {
                        output.insert(i, test);
                        break;
                    }
                    else if i == output.len()-1
                    {
                        output.push(test);
                        break;
                    }
                }   
            }
            return  output;
        }
        fn error(&self) -> f32{
            return (self.target as f32 - self.pnod).abs();
        }
    }

fn save_settings(settings : &Settings){
    let ronstring = to_string(&settings)
    .expect("Failed to make ron string");
    fs::write("setup.ron", ronstring).expect("Failed to save setup");
}

fn no_loops_onx(divisions : u32, x : u32) -> bool{
    let mut hold = divisions % x;
    if hold == 0{
        return false;
    };
    if hold > x / 2{
        hold = x - hold;
    };
    if hold == 1{
        return true;
    }
    if x as f32 / hold as f32 % 1.0 == 0.0{
        return false;
    }
    true 
}

fn help(){
    println!("    all Commands work by putting the command char in followed by values separated by spaces");
    println!("\nHow To Use");
    println!("'_'       indicates the letter you need to enter to use that command");
    println!("'I'       means you give one integer as arguments");
    println!("'#I'      means you can put any number/or value of integers as arguments");
    println!("'F'       means you give one float number as an arguments");
    println!("'#F'      means you give any number/or value of float number as an arguments");
    println!("Note#     only the first letter of the first word needs to be the com char and it will use the first/all valid args it can extract");
    println!("          'add' would also work in place of 'a'");
    println!("example  :'a' '#I' would be entered a 16 17 18");
    println!("example  :'A' 'F' would be entered A 0.001");
    println!("example  :'m' 'I' would be entered m 1");
    println!("\nCommand List");
    println!("'?'     to show this text :P");
    println!("'a' '#I'  to add a new plate");
    println!("'r' 'I'   to remove the plate at that ID");
    println!("'l'       to list all plates and there IDs");
    println!("'R' 'I'   to set ratio of crank");
    println!("'f'       to set first only mode: useful if you only want the most accurate");
    println!("'A' 'F'   to set the minimum accuracy to be displayed : use if looking for obscure value without lots of work laps");
    println!("'m' 'I'   to set the minimum work labs to do : recommended to leave at 1 unless specific work lap is desired");
    println!("'M' 'I'   to set the max worklaps to search");
    println!("'s'       displayed current settings");
    println!("'p' 'I'   to set at what point to stop displaying : after 'I' results");
    println!("'I'       to find and display all/page_size combinations to get I");
}
