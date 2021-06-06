use std::io;

fn main() {
    let date = prompt_date();
    println!("{}", calculate_day_of_week(date));
    
}
/*
const MONTH_DAYS: [u8;12] = [31,28,31,30,31,30,31,31,30,31,30,31];

struct Month{
    first_weekday: u8,
    days_in_month: u8,
    month_number: u8
}

struct Year{
    months: Vec<Month>,
    year: u32
}

fn create_year_struct()-> Year
{
    let yearNum;
    println!("Please enter the day of the month: ");
    let mut year_input = String::new();
    io::stdin().read_line(&mut year_input).expect("Failed to read line");
    let trimmmed = year_input.trim();
    match trimmmed.parse::<u32>(){
        Ok(i) => yearNum = i,
        Err(..) => yearNum = 1
    }

    let mut month_list = Vec::with_capacity(12);
    for m in MONTH_DAYS.iter(){
        let date
        let month_temp = Month {
            first_weekday: calculate_day_of_week(
        };
        month_list.push(month_temp);
    }


    Year{
        month
    }
}
*/
//month codes for calculating weekday
const MONTH_CODE: [u8;12] = [0,3,3,6,1,4,6,2,5,0,3,5];
// Century constants for calculating weekday
const SEVENTEENTH: u8 = 4;
const EIGHTEENTH: u8 = 2;
const NINTEENTH: u8 = 0;
const TWENTYETH: u8 = 6;
const TWENTY_FIRST: u8 = 4;
const TWENTY_SECOND: u8 = 2;
const TWENTY_THIRD: u8 = 0;

// Prompt user for the date
fn prompt_date() -> (usize, usize , u32){
    let month_num;
    println!("Please enter the month as a number (Jan is 1): ");
    let mut month_num_input = String::new();
    io::stdin().read_line(&mut month_num_input).expect("Failed to read line");
    let trimmmed = month_num_input.trim();
    match trimmmed.parse::<usize>(){
        Ok(i) => month_num = i,
        Err(..) => month_num = 1
    }

    let day_num;
    println!("Please enter the day of the month: ");
    let mut day_num_input = String::new();
    io::stdin().read_line(&mut day_num_input).expect("Failed to read line");
    let trimmmed = day_num_input.trim();
    match trimmmed.parse::<usize>(){
        Ok(i) => day_num = i,
        Err(..) => day_num = 1
    }

    let year;
    println!("Please enter the day of the month: ");
    let mut year_input = String::new();
    io::stdin().read_line(&mut year_input).expect("Failed to read line");
    let trimmmed = year_input.trim();
    match trimmmed.parse::<u32>(){
        Ok(i) => year = i,
        Err(..) => year = 1
    }
    (month_num,day_num,year)
}

fn calculate_day_of_week(date: (usize,usize,u32)) -> u8{
    let (month, day, year) = date;
    let year_code = get_year_code(year);
    let month_code = get_month_code(month);
    let century_code = get_century_code(year);
    let leap_year_code = if get_is_leap_year(year) {1}else {0};
    let day: u8 = day as u8;
    ((year_code + (month_code as u32) + (century_code as u32) + (day as u32) - (leap_year_code as u32))%7) as u8
}

fn get_month_code(month: usize) -> u8{
    MONTH_CODE[month-1]
}

fn get_is_leap_year(year: u32) -> bool{
    (year % 4 == 0) & ((year % 100 != 0) || (year % 400 == 0))
}

fn get_year_code(year:u32) ->u32{
    let num = year%100;
    ((num+(num/4))%7) as u32
}


fn get_century_code(year:u32) -> u8{
    if year >= 2300{
        TWENTY_THIRD
    }
    else if year >= 2200{
        TWENTY_SECOND
    }
    else if year >= 2100{
        TWENTY_FIRST
    }
    else if year >= 2000{
        TWENTYETH
    }
    else if year >= 1900{
        NINTEENTH
    }
    else if year >= 1800{
        EIGHTEENTH
    }
    else if year >= 1700{
        SEVENTEENTH
    }
    else{
        0
    }
}