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
            Ok(val) => search(val, &settings),
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
                    let val : f64 = val;
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
    r : usize,
    plates : Vec<Vec<usize>>,
    first : bool,
    acc : f64,
    min : usize,
    max : usize,
    page_size : usize,
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

fn search(divisions : usize, settings : &Settings){
    let mut finds : Vec<PrintData> = Vec::new();
    if let Some(find) = find_without_plate(divisions, settings.r){ //looks to see if you can find a solution without using a plate
        println!("Found Exact matches");
        println!("┌{0:─<5}┬{0:─<20}┬{0:─<5}┬{0:─<20}┬{0:─<20}┐","");
        find.print();
        println!("└{0:─<5}┴{0:─<20}┴{0:─<5}┴{0:─<20}┴{0:─<20}┘","");
        return;
    }

    for x in settings.min..settings.max{//search eatch x work lap computation
        if x != 1 && !no_loops_onx(divisions, x){//if x will result in an infinit loop
            continue;
        }
        if x > divisions/2{//break if xis more the half the divisions for this is just a mirror of the fist half
            break;
        }
        for plate in settings.plates.iter(){//check eatch plate for solution
            for &hole in plate {
                if let Some(find) = find_single_hole(divisions, settings.r, hole, x, settings.acc){
                    finds.push(find);
                }
                for &sec_hole in plate{
                    if let Some(find) = find_dubble_hole_forward(divisions, settings.r, hole, sec_hole, x, settings.acc){
                        finds.push(find);
                    }
                    if let Some(find) = find_dubble_hole_backward(divisions, settings.r, hole, sec_hole, x, settings.acc){
                        finds.push(find);
                    }
                }
            }
        }

    }

    let find_count = finds.len();
    if find_count < 1{
        println!("No Matches found for setting : {:#?}", settings);
        println!("Note: try lowering the accuracy or expanding work laps");
        return;
    }
    println!("Found {} possible matches", find_count);
    println!("┌{0:─<5}┬{0:─<20}┬{0:─<5}┬{0:─<20}┬{0:─<20}┐","");
    for (i,found) in PrintData::sort(finds).iter().enumerate(){
        if i > settings.page_size {
            println!("└{0:─<5}┴{0:─<20}┴{0:─<5}┴{0:─<20}┴{0:─<20}┘","");
            println!("Stopped at Page Limit - Found {} More;\nUse 'p' to change page size and see them", find_count - settings.page_size);
            return;
        }
        found.print();
        if settings.first{
            break;
        }
    }
    println!("└{0:─<5}┴{0:─<20}┴{0:─<5}┴{0:─<20}┴{0:─<20}┘","");
    
}



fn find_without_plate(divisions : usize, ratio : usize) -> Option<PrintData>{
    if ratio as f64 / divisions as f64 % 1.0 == 0.0{
        return Some(
            PrintData{
                target : divisions,
                pnod : divisions as f64,
                laps : 1,
                movement : (ratio / divisions).to_string(),
                complexity : 0,
            }
        );
    }
    None
}

fn find_single_hole(divisions : usize, ratio : usize, hole_count : usize, x : usize, acc : f64) -> Option<PrintData>{
    let step = ratio as f64 / divisions as f64 * x as f64;
    let mut full = (step - (1.0 / hole_count as f64) * x as f64).floor();
    let mut holes = (step - full) * hole_count as f64;
    if holes > hole_count as f64{
        full += 1.0;
        holes -= hole_count as f64;
    }
    let find = PrintData{
        target : divisions,
        pnod : ratio as f64 / (full + (holes.round() / hole_count as f64)) * x as f64,
        laps : x,
        movement : if full == 0.0 {format!("{}/{}", holes.round(), hole_count)}else {format!("{} & {}/{}", full, holes.round(), hole_count)},
        complexity : x,
    };
    if find.error() <= acc {
        return Some(find);
    }
    None
}

fn find_dubble_hole_forward(divisions : usize, ratio : usize, hole_count : usize,sec_hole_count : usize, x : usize, acc : f64) -> Option<PrintData>{
    if hole_count == sec_hole_count { //skip if holes are the same hole
        return None;
    }
    //do not trust any of this implicity but it works
    let step = ratio as f64 / divisions as f64 * x as f64;
    let mut full = (step - (1.0 / hole_count as f64) * x as f64).floor();
    let big_step = step - full;
    let big_holes = (big_step * hole_count as f64).floor(); //floor to make shure we allways need to go forward with the seconed hole
    let small_step = big_step - big_holes * (1.0 / hole_count as f64);//is the remander needed to be made up by the seconed hole
    
    for i in 0..hole_count{
        let i = i as f64;
        let small_holes = ((small_step  + i * (1.0 / hole_count as f64))* sec_hole_count as f64).round(); //caculate small holes by taking small step and adding i bigholes to it
        let test = ratio as f64 / (full + ((big_holes - i) / hole_count as f64) + (small_holes / sec_hole_count as f64)) * x as f64;
        if (test - divisions as f64).abs() <= acc{
            if small_holes == 0.0 || small_holes == sec_hole_count as f64{return  None;}//return if the small_holes is 0 or a full rotation; is the same as just using hole
            if big_holes - i == 0.0 || big_holes - i == hole_count as f64{return  None;} //return if the new bigholes is 0; is the same as just useing sec_hole
            let first_holes = if big_holes - i < 0.0{
                full -= 1.0;
                big_holes - i + hole_count as f64
            }else {big_holes - i};
            let find = PrintData{
                target : divisions as usize,
                pnod : test,
                laps : x,
                movement : if full == 0.0 {format!("{}/{} + {}/{}", first_holes, hole_count, small_holes, sec_hole_count)}else {format!("{} & {}/{} + {}/{}", full, first_holes, hole_count, small_holes, sec_hole_count)},
                complexity : 2 * x,
            };
            return Some(find);
        }
    }
    None
}

fn find_dubble_hole_backward(divisions : usize, ratio : usize, hole_count : usize,sec_hole_count : usize, x : usize, acc : f64) -> Option<PrintData>{
    if hole_count == sec_hole_count { //skip if holes are the same hole
        return None;
    }
    //be sceptical of this it is playing nice but is not as tested as the forward version and but is basicly a sine flip so should be fin
    //there is a bug that is some how can end up with a big holes more then 1.99 x hole_count witch results in x/X where x is bigger then X but the math is correct just doesnt look as nice
    let step = ratio as f64 / divisions as f64 * x as f64;
    let mut full = (step - (1.0 / hole_count as f64) * x as f64).floor();
    let big_step = step - full;
    let big_holes = (big_step * hole_count as f64).ceil(); //ceil to make shure we allways need to go back with the seconed hole
    let small_step = big_step - big_holes * (1.0 / hole_count as f64);//is the remander needed to be made up by the seconed hole
    
    for i in 0..hole_count{
        let i = i as f64;
        let small_holes = ((-small_step  + i * (1.0 / hole_count as f64)) * sec_hole_count as f64).round(); //caculate small holes by taking small step and adding i bigholes to it
        let test = ratio as f64 / (full + ((big_holes + i) / hole_count as f64) - (small_holes / sec_hole_count as f64)) * x as f64;
        if (test - divisions as f64).abs() <= acc{
            if small_holes == 0.0 || small_holes == sec_hole_count as f64{return  None;}//return if the small_holes is 0 or a full rotation; is the same as just using hole
            let first_holes = if big_holes + i > hole_count as f64{
                full += 1.0;
                big_holes + i - hole_count as f64
            }else {big_holes + i};
            if first_holes == 0.0 || first_holes == hole_count as f64{return  None;} //return if the new bigholes is 0 or is a full rotation; is the same as just useing sec_hole
            let find = PrintData{
                target : divisions as usize,
                pnod : test,
                laps : x,
                movement : if full == 0.0 {format!("{}/{} - {}/{}", first_holes, hole_count, small_holes, sec_hole_count)}else {format!("{} & {}/{} - {}/{}", full, first_holes, hole_count, small_holes, sec_hole_count)},
                complexity : 2 * x,
            };
            return Some(find);
        }
    }
    None
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
        let mut holes = Vec::new();
        for hole in platet{
            let hole: usize = match hole.trim().parse() {
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
        target : usize,
        movement : String,
        laps : usize,
        pnod : f64,
        complexity : usize
    }
    impl PrintData {
        fn print(&self){
            if self.error() == 0.0 {
                println!("│{:05}│{:^20}│{:^5}│{:^20}│{:^20}│",self.target, self.movement, self. laps, self.pnod, "Ext");
            }else{
                println!("│{:05}│{:^20}│{:^5}│{:^20}│{:^20}│",self.target, self.movement, self. laps, self.pnod, 0.001 / self.error() * self.target as f64 / 3.14159265359);
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
                    else if test.error() == data.error() && test.complexity < data.complexity{
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
        fn error(&self) -> f64{
            return (self.target as f64 - self.pnod).abs();
        }
    }

fn save_settings(settings : &Settings){
    let ronstring = to_string(&settings)
    .expect("Failed to make ron string");
    fs::write("setup.ron", ronstring).expect("Failed to save setup");
}

fn no_loops_onx(divisions : usize, x : usize) -> bool{
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
