// Project: Color Blind Easy Color Name
// Author:  Joao Carvalho
// Date:    2022.10.02
//
// Description: A simple project that given a color code in hexadecimal String,
//
//              $ color_blind_easy_color_name d3c2a5
//                Slightly Dark Orangish White
//
//              or 
//
//              $ cargo run --release -- D3C2A5
//                Slightly Dark Orangish White
//
//              gives back a description of the color has a string. The
//              description is perceptually good for a color blind person.
//              The majority of color blind persons don't see only black and
//              white color, or gray shades, but normally they are more sensible
//              to one color then the others.
//
//              The reason why I did this small program is:
//              In the world population 8% of men and 0.5% of women are color
//              blind. This is roughly 300 million men and 19 million women.
//              There should exist free programs to assist them.
//
//              Also there are many different methods to do a color picker
//              depending on the operating system you are (Windows, Linux or
//              Mac) and depending, for example in Linux if you have a X or
//              Wayland windows system.
//              Many of the possible solution work with a shell script and you
//              can simply pipe it to this program and redirect the output.
//              This is a fast executing program.
//
//              The color text descriptions where extracted and reformated from
//              a section from the free accessible file 
//              https://www.hikarun.com/e/color-fr.xml
//              There you can find a free windows program for color blind persons.
//              See my formatted file for details.
// 
// License: The license of the my formatted description file is the license from
//          it's original author that is available in the link above.
//          The license of my source code is MIT Open Source License.  
//  

use std::env;

fn main() {
    // Parse color RGB hex argument.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error:\n  Please pass the color argument in RGG Hex format \n   ex: color_blind_easy_color_name F2C3A4");
        std::process::exit(-1);
    }
    let color_hex_str = &args[1];
    let color_descript_str = include_str!("color_blind_easy_color_names.txt");
    let color_vec: Vec<Color> = Color::parse_color_descriptions(& color_descript_str);
    let res = Color::search_nearest_color(& color_vec, color_hex_str);
    if res.is_err() {
        println!("Error: \n In hex format string \n ex: color_blind_easy_color_name F2C3A4 ");
        std::process::exit(-1);
    }
    let nearest_color_description = res.unwrap();
    println!("{}", nearest_color_description);
}

#[derive(Debug)]
struct MyError();

struct Color {
    red:   u8,
    green: u8,
    blue:  u8,
    text_description: String,
}

impl Color {
    fn new(red: u8, green: u8, blue: u8, text_description: String) -> Self {
        Color {
            red,
            green,
            blue,
            text_description,
        }
    }

    /// Calculate the Euclidean distance:
    ///    Sqrt( (a0 - b0)^2 + (a1 - b1)^2 + (a2 - b2)^2 ).
    /// 
    /// There are specific methods to compare perceptual color distances.
    /// But because we have 912 samples the color will be very close even 
    /// with a Euclidean distance.
    fn calc_distance(& self, red: u8, green: u8, blue: u8) -> f32 {
        f32::sqrt(   f32::powf(self.red   as f32 - red   as f32, 2_f32 )
                   + f32::powf(self.green as f32 - green as f32, 2_f32 )
                   + f32::powf(self.blue  as f32 - blue  as f32, 2_f32 ) )
    }

    fn parse_color_descriptions(color_descript_str: & str) -> Vec<Color> {
        let mut color_vec: Vec<Color> = Vec::new();

        for line in color_descript_str.lines() {
            // Remove empty lines and comment lines, that start with a "#" symbol.
            if line.trim().len() == 0 || line.get(0..1).unwrap() == "#" {
                continue;
            }
            let mut splitted_line = line.split_ascii_whitespace();
            let _       = splitted_line.next();
            
            let hex_str = splitted_line.next().unwrap();
            let red   = u8::from_str_radix(hex_str.get(0..2).unwrap(), 16).unwrap();
            let green = u8::from_str_radix(hex_str.get(2..4).unwrap(), 16).unwrap();
            let blue  = u8::from_str_radix(hex_str.get(4..6).unwrap(), 16).unwrap();
            
            let mut text_description = String::with_capacity(60);
            for word in splitted_line {
                text_description = text_description + word + " ";
            }
            // Removes the last added space.
            text_description.pop();

            color_vec.push(Color::new(red, green, blue, text_description))
        }

        color_vec
    }

    fn search_nearest_color<'a, 'b>(color_vec: & 'a Vec<Color>, hex_color_str: & 'b str) -> Result<& 'a str, MyError> {
        
        // Parse the input safely.
        let hex_color_str = hex_color_str.trim(); 
        if hex_color_str.len() != 6 {
            return Err(MyError());
        }
        let mut res = hex_color_str.get(0..2);
        if res.is_none() {
            return Err(MyError());
        }
        let red_res   = u8::from_str_radix(res.unwrap(), 16);
        if red_res.is_err() {
            return Err(MyError());
        } 
        res = hex_color_str.get(2..4);
        if res.is_none() {
            return Err(MyError());
        }
        let green_res   = u8::from_str_radix(res.unwrap(), 16);
        if green_res.is_err() {
            return Err(MyError());
        }let res = hex_color_str.get(4..6);
        if res.is_none() {
            return Err(MyError());
        }
        let blue_res   = u8::from_str_radix(res.unwrap(), 16);
        if blue_res.is_err() {
            return Err(MyError());
        }
        let (red, green, blue) = ( red_res.unwrap(), green_res.unwrap(), blue_res.unwrap() );

        // TODO: Implement a better algorithm that doesn't do this by brute force.

        // Search for the best and nearest correspondence in 3 degrees of colors.        
        let mut nearest_distance = f32::MAX; 
        let mut cur_nearest_color: & Color = & color_vec[0];
        for color in color_vec {
            let distance = color.calc_distance(red, green, blue);
            if nearest_distance > distance {
                nearest_distance = distance;
                cur_nearest_color = color;
            }
        }

        Ok(& cur_nearest_color.text_description)
    }

}
