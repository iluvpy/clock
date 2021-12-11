
pub const ZERO: [&str; 6] = [
	"  ####  ",
	" #    # ",
    " #    # ",
	" #    # ",
	" #    # ",
	"  ####  ",
];

pub const ONE: [&str; 6] = [
	"  ###  ",
	" # ##  ",
	"   ##  ",
	"   ##  ",
	"   ##  ",
	"  #### ",
];

pub const TWO: [&str; 6] = [
	"  ###   ",
	" #   #  ",
	"     #  ",
	"    #   ",
	"  ##    ",
	" #####  ",
];

pub const THREE: [&str; 6] = [
	"   ###  ", 
	"  #   # ", 
	"      # ",
	"   ###  ", 
	" #   #  ",
	"  ###   ", 
];

pub const FOUR: [&str; 6] = [
	"    ##  ",   
	"   # #  ",  
	"  #  #  ", 
	" ###### ",
	"     #  ",  
	"     #  ",  
];

pub const FIVE: [&str; 6] = [
	"   ####  ", 
	"  #      ",
	"  ####   ",  
	"     ##  ", 
	"     ##  ",
	"  ####   ", 
];

pub const SIX: [&str; 6] = [
	" ###### ",
	" #      ",
	" ###### ",
	" ##  ## ",
	" ##  ## ",
	" ###### "
];

pub const SEVEN: [&str; 6] = [
	" ###### ",
	"     #  ",
	"    #   ",
	"   #    ",
	"  #     ",
	" #      "
];

pub const EIGHT: [&str; 6] = [
	"  ###  ",  
	" #   # ",
    "  ###  ",
    "  ###  ", 
    " #   # ",
    "  ###  " 
];

pub const NINE: [&str; 6] = [
	" ##### ",  
	" #   # ", 
    " ##### ",
    "     # ",
    "     # ",
    "     # "
];


pub const NUMBERS: [[&str; 6]; 10] = [
	ZERO,
	ONE,
	TWO, 
	THREE,
	FOUR,
	FIVE,
	SIX,
	SEVEN,
	EIGHT,
	NINE
];

pub fn get_number(ch: char) -> [&'static str; 6] {
	let number = match ch {
		'0' => ZERO,
		'1' => ONE,
		'2' => TWO,
		'3' => THREE,
		'4' => FOUR,
		'5' => FIVE,
		'6' => SIX,
		'7' => SEVEN,
		'8' => EIGHT,
		'9' => NINE,
		_ => ZERO,
	};
	return number;
}

