use std::{process::exit, fs::read_to_string};

use json::parse;

/// a description of the shape of the CNC bit
/// the
pub enum ToolShape {
    /// `Flat(r)` represent a flat CNC bit of rayon `r` in meter
    Flat(f64),
    /// `Ball(r)` represent a ball CNC bit of rayon `r` in meter
    Ball(f64),
    /// `V(r, t)` represent a `V` CNC bit of rayon `r` in meter and angle `t` in radian
    V(f64, f64)// `V(r, a)` is a `V` of rayon `r` and angle `a` in radian
}

impl ToolShape {
    /// return the rayon of the CNC bit
    pub fn get_rayon(&self) -> f64 {
        match self {
            ToolShape::Flat(r) => r,
            ToolShape::Ball(r) => r,
            ToolShape::V(r, _) => r
        }.clone()
    }

    /// return the size of the CNC bit along the z-axis,
    /// usefull for debug
    pub fn get_size(&self) -> f64 {
        match self {
            ToolShape::Flat(_) => 1.0,
            ToolShape::Ball(r) => r.clone(),
            ToolShape::V(r, t) => r.clone() * f64::tan(t.clone())
        }
    }
}

/// configuration structure,
/// deduced from the JSON input to the program
pub struct Config {
    /// shape of the tool used by the CNC
    pub tool_shape : ToolShape,

    /// vertial speed of the CNC in `m / s`
    pub vectical_speed : f64,

    /// horizontal work speed of the CNC in `m / s`
    pub horizontal_work_speed : f64,

    /// horizontal fly speed of the CNC in `m / s`
    pub horizontal_fly_speed : f64,

    /// height (along z-axis) of the CNC wick in the flight phases
    pub fly_z : f64,

    /// depth in `m` (minimum value of z)
    pub depth : f64,

    /// width in `m` (maximum value of x)
    pub width : f64,

    /// height in `m` (maximum value of y)
    pub height : f64,

    /// if true, the height map is normalized using the
    /// the deepest point
    pub normalizing : bool,

}

pub fn help() -> String {
r#"
the arguments must have this form
    <path to this program> -config <path to your json configuration> -hmap <path to the height map>
the input JSON must have the following format:
{
    "tool shape" : {
        "shape" : "flat",
        "rayon" : 1.2e-2
    },
    "flight height" : 1e-3,
    "vertical speed" : 1e-3,
    "horizontal work speed" : 1e-3,
    "horizontal fly speed" : 1e-2,
    "depth" : 1.5e-3,
    "width" : 1e-2,
    "height": 2e-2,
    "normalizing" : "false"
}

with
- "tool shape" a description of the tool with two or three inputs:
    . "shape": the shape of the CNC bit, it can be "flat", "ball" or "v"
    . "rayon": the rayon of the CNC bit as float in `m`
    . "angle": the angle of the CNC bit if "shape" map to "v" as float in radian
- "vertical speed" is the vertical speed of the CNC bit as float in `m / s`
- "horizontal fly speed" is the horizontal speed of the CNC bit above the object to be engraved as float in `m / s`
- "horizontal work speed" is the horizontal speed of the CNC bit insides the object to be engraved as float in `m / s`
- "deep" is the maximum engraving depth as float in `m`
- "width" is the size along the x-axis of the engraved object as float in `m`
- "height" is the size along the y-axis of the engraved object as float in `m`
- "normalizing", if "true" then the depth is normalized using the maximal depth in the input image
- "flight height" :  height (along z-axis) of the CNC wick in the flight phases
"#.to_string()
}

pub fn get_path(args: &Vec<String>) -> (String, String) {
    let mut i = 1;

    let mut config_file : Option<String> = None;
    let mut hmap_file : Option<String> = None;


    while i < args.len() {
        if args[i] == "-config" {
            if i+1 >= args.len() {
                panic!("unexpected entry, must contain a file to the configuration after \"-config\"");
            }
            config_file = Some(args[i+1].clone());
            i += 2;
        } else if args[i] == "-hmap" {
            if i+1 >= args.len() {
                panic!("unexpected entry, must contain a file to the hmap after \"-hmap\"");
            }
            hmap_file = Some(args[i+1].clone());
            i += 2;
        } else if args[i] == "-help" || args[i] == "-h" {
            println!("{}", help());
            exit(0);
        } else {
            panic!("input not recognized {}", args[i]);
        }

    }
    if let Some(config_file) = config_file {
        if let Some(hmap_file) = hmap_file {
            (config_file, hmap_file)
        }
        else {
            panic!("unexpected entry, must contain a hmap file");
        }
    }
    else {
        panic!("unexpected entry, must contain a configuration file");
    }
}


impl Config {
    pub fn new(path:&str) -> Result<Self, String> {
        if let Ok(content) = read_to_string(path) {
            if let Ok(content) = parse(&content) {
                Self::new_from_json_obj(content, path)
            } else {
                Err(format!("unable to parse the file `{}`", path))
            }
        } else {
            Err(format!("unable to open the file `{}`", path))
        }
    }

    pub fn new_from_json_obj(object:json::JsonValue, path:&str) -> Result<Self, String> {
        let tool_shape : ToolShape = {
            let shape = &object["tool shape"]["shape"];
            let rayon = if let Some(rayon) = object["tool shape"]["rayon"].as_f64() {
                rayon
            } else {
                return Err(format!("doesn't find a valid CNC bit rayon in the file `{}`", path));
            };
            if shape == "flat" {
                ToolShape::Flat(rayon)
            } else if shape == "ball" {
                ToolShape::Ball(rayon)
            } else if shape == "v" {
                let angle = if let Some(angle) = object["tool shape"]["angle"].as_f64() {
                    angle
                } else {
                    return Err(format!("doesn't find a valid CNC bit angle in the file `{}`", path));
                };
                ToolShape::V(rayon, angle)
            } else {
                return Err(format!("doesn't find a valid CNC bit shape in the file `{}`", path));
            }
        };

        let find_f64 = |name:&str| -> Result<f64, String> {
            if let Some(data) = object[name].as_f64() {
                Ok(data)
            } else {
                Err(format!("dont find a valid {} in the file `{}`", name, path))
            }
        };

        let normalizing = if object["normalizing"] == "true" || object["normalizing"] == "false" {
            object["normalizing"] == true
        } else {
            return Err(format!("don't find a valid normalization parameter in the file `{}`", path));
        };


        Ok(Config{
            tool_shape,
            vectical_speed:find_f64("vertical speed")?,
            horizontal_fly_speed:find_f64("horizontal fly speed")?,
            horizontal_work_speed:find_f64("horizontal work speed")?,
            fly_z:find_f64("flight height")?,
            normalizing,
            depth:find_f64("depth")?,
            width:find_f64("width")?,
            height:find_f64("height")?
        })

    }


}
