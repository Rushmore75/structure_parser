
use std::process::Command;

fn main() {

    // run nbted command
    let output = Command::new("nbted")
        .arg("-p")
        .arg("bob.nbt")
        .output()
        .expect("Failure");

    // split at newline chars
    let newline_splits = output
        .stdout
        .as_slice()
        .split(|f| *f as char == '\n')
        .collect::<Vec<&[u8]>>();

    let mut schematic = Schematic {
        palette: Vec::new(),
        blocks: Vec::new(),
    };

    let _ = &newline_splits
        .iter()
        .enumerate()
        .for_each(|(loc, f)| {
            let line = f.iter().map(|x| *x as char).collect::<String>();
            let line_split = line.trim().split(|f| f == ' ').collect::<Vec<&str>>();
            
            // if line.contains("blocks") {
            //     let take = splits[3].trim().parse::<i32>().unwrap();
            //     &s.iter().skip(loc+1).
            // }
            if line.contains("pos") {
                let len = line_split[3].parse::<usize>().unwrap();
                let pos = &newline_splits
                    // skip down to the location of the coordinates, and take them
                    .iter()
                    .skip(loc+1)
                    .take(len)
                    // convert lines to strings
                    .map(|f| f
                        .iter()
                        .map(|x| *x as char)
                        .collect::<String>())
                    // parse out i32
                    .map(|f| f
                        .trim()
                        .parse::<i32>()
                        .unwrap())
                    .collect::<Vec<i32>>();

                print!("pos: {:?}", pos);

                let state = &newline_splits
                    .iter()
                    // skip down to state line
                    .skip(loc+1+len)
                    // take state
                    .take(1)
                    // convert to strings
                    .map(|f| f
                        .iter()
                        .map(|x| *x as char)
                        .collect::<String>())
                    .collect::<Vec<String>>()
                    // take the first (only) string
                    [0]
                    .split(|f| f == ' ')
                    .collect::<Vec<&str>>()
                    // take the 2nd element, the state
                    [2]
                    .parse::<i32>()
                    .unwrap();

                println!(" state: {}", state);

                // TODO parse palate out 

                
                schematic.blocks.push(
                    Block {
                        pos: pos.to_vec(),
                        state: *state,
                    }
                );
            }
            
            if line.contains("palette") {
                println!("Palette");
            }
        });
}

struct Schematic {
    blocks: Vec<Block>,
    palette: Vec<String>,
}

struct Block {
    pos: Vec<i32>,
    state: i32,
}