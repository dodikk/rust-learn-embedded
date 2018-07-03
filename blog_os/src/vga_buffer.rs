

// silence code style enforcing
// no other people on the team. no project support expected.
//
#![allow(non_snake_case)]

#![deny(warnings)]

use core::fmt;

extern crate volatile;
use self::volatile::Volatile;

extern crate spin;
use ::spin::Mutex;


// for unit testing
// allows standard library for asserts and other good things
// (which are unavailable on bare metal)
//
#[cfg(test)]
extern crate std;



#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color 
{
    Black      =  0,
    Blue       =  1,
    Green      =  2,
    Cyan       =  3,
    Red        =  4,
    Magenta    =  5,
    Brown      =  6,
    LightGray  =  7,
    DarkGray   =  8,
    LightBlue  =  9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,

} // enum Color


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode 
{
    fn new(
    	foreground: Color, 
    	background: Color) 
    -> ColorCode 
    {
    	// assuming little endian
    	//
    	let rawValue: u8 = 
			(background as u8) << 4 
		  | (foreground as u8);

        ColorCode(rawValue)

    } // fn new()

} // impl ColorCode



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar 
{
    ascii_character: u8       ,
    color_code     : ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH : usize = 80;

struct Buffer 
{
    chars: 
    [  
    	[
    		Volatile<ScreenChar>; 
    		BUFFER_WIDTH
    	]; 

    	BUFFER_HEIGHT 
    ]
}



pub struct Writer 
{
    column_position:              usize    ,
    color_code     :              ColorCode,
    buffer         : &'static mut Buffer   ,
}



impl Writer 
{
    pub fn write_byte(&mut self, byte: u8) 
    {
        match byte 
        {
            b'\n' => self.new_line(),
            
            byte => 
            {
                if self.column_position >= BUFFER_WIDTH 
                {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                

                let outColouredChar = 
                ScreenChar 
                {
                    ascii_character: byte      ,
                    color_code     : color_code,
                };

                self.buffer.chars[row][col].write(outColouredChar);
                self.column_position += 1;

            } // byte

        } // match

    } // fn write_byte

	pub fn write_string(&mut self, s: &str) 
	{
        for byte in s.bytes() 
        {
            match byte 
            {
                
                // printable ASCII byte or newline
                0x20...0x7e | b'\n' => self.write_byte(byte),
                

                // not part of printable ASCII range
                _ => self.write_byte(0xfe),


            } // match

        } // for

    } // fn write_string

    fn new_line(&mut self) 
	{
		for row in 1..BUFFER_HEIGHT 
		{
            for col in 0..BUFFER_WIDTH 
            {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);

            } // for column

        } // for row


        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;

	} // fn new_line()


    fn clear_row(&mut self, row: usize) 
    {
        let blank = ScreenChar 
        {
            ascii_character: b' ',
            color_code     : self.color_code,
        };
        
        for col in 0..BUFFER_WIDTH 
        {
            self.buffer.chars[row][col].write(blank);

        } // for

    } // fn clear_row()

} // impl Writer 


/*pub*/ impl fmt::Write for Writer 
{
    fn write_str(
    	&mut self, 
    	s: &str) 
    -> fmt::Result 
    {
        self.write_string(s);
        Ok(())
    }
} // Writer + fmt::Write


lazy_static! 
{
    pub static ref WRITER: Mutex<Writer> = Mutex::new(
    Writer
    {
        column_position: 0,
        color_code     : ColorCode::new(
        					Color::Yellow, 
        					Color::Black ) ,
        buffer         : unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}


macro_rules! print 
{
    ($($arg:tt)*) => ($crate::vga_buffer::print(format_args!($($arg)*)));
}

macro_rules! println 
{
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) 
{
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

// pub fn print_something() 
// {
// 	let yellowOnBlack = 
// 		ColorCode::new(
// 			Color::Yellow, 
// 			Color::Black );

//     let mut writer = Writer 
//     {
//         column_position: 0            ,
//         color_code     : yellowOnBlack,
//         buffer         : unsafe { &mut *(0xb8000 as *mut Buffer) },
//     };

//     writer.write_byte(b'H');
//     writer.write_string("ello ");
//     writer.write_string("Wörld!");
//     write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();

// }


#[cfg(test)]
mod vga_buffer_logic_test 
{
    use super::*;



    fn construct_writer() -> Writer 
    {
		use vga_buffer::std::boxed::Box;

	    let blueOnMagenta = 
	    ColorCode::new(
	    	Color::Blue   , 
	    	Color::Magenta);

        let buffer = construct_buffer();
        Writer 
        {
            column_position: 0                          ,
            color_code     : blueOnMagenta              ,
            buffer         : Box::leak(Box::new(buffer)),
        }

    } // fn construct_writer()

	fn construct_buffer() -> Buffer 
	{
	    use array_init::array_init;

	    Buffer 
	    {
	        chars: array_init(|_| array_init(|_| Volatile::new(empty_char()))),


            // chars: [[Volatile::new(empty_char()); BUFFER_WIDTH]; BUFFER_HEIGHT],

	    }
	} // fn construct_buffer()

	fn empty_char() -> ScreenChar 
	{
		let greenOnBrown = 
			ColorCode::new(
				Color::Green , 
				Color::Brown );


    	ScreenChar 
    	{
        	ascii_character: b' '        ,
        	color_code     : greenOnBrown,
 	    }

	} // fn empty_char()

    #[test]
    fn write_byte() 
    {
        let mut writer = construct_writer();
        writer.write_byte(b'X');
        writer.write_byte(b'Y');

        for (i, row) in writer.buffer.chars.iter().enumerate() 
        {
            for (j, screen_char) in row.iter().enumerate() 
            {
                let screen_char = screen_char.read();
                
                let isFirstSymbolOfFirstTextLine: bool = 
                	(i == BUFFER_HEIGHT - 1) 
                 && (j == 0);

                let isFirstSymbolOfSecondTextLine: bool = 
                	(i == BUFFER_HEIGHT - 1)
                 && (j == 1 );


                if isFirstSymbolOfFirstTextLine
                {
                    assert_eq!(screen_char.ascii_character, b'X');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } 
                else if isFirstSymbolOfSecondTextLine
                {
                    assert_eq!(screen_char.ascii_character, b'Y');
                    assert_eq!(screen_char.color_code, writer.color_code);
                } 
                else 
                {
                    assert_eq!(screen_char, empty_char());
                }
            
            } // for -- column

        } // for -- row
    } // test -- fn write_byte() 

} // mod tests



