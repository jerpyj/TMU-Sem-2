#![allow(non_snake_case,non_camel_case_types,dead_code)]

fn segment(start: &(i32,i32), end: &(i32,i32), lines: &[u32]) -> String
{
	//get the absolute total length of all the lines as a signed 32 bit integer
	let absoluteLength: i32 = lines.iter().map(|&x| x as i32).sum();

	//create a stack with a vector containing the start x and y, the index of line, the path taken, and the absolute total length of all the lines
    let mut stack = vec![(start.0, start.1, 0, String::new(), absoluteLength)];

	//Create a while loop with patern matching to sort through possible directions
	while let Some((x, y, idx, path, newLength)) = stack.pop() {
		
		// if the end of the lines has been reached
		if idx == lines.len() {
			if  (x,y) == *end{
				return path;
			}
			continue; // if not, ignore
		}

		
		let totalLength = (end.0 - x).abs() + (end.1 - y).abs();
		//calculate total length left

		//Check if length is even possible
		if totalLength > newLength {
			continue;
		}
		
		//set len as the current line length
		let len = lines[idx] as i32;

		//new length to represent total distance required left
		let newerLength = newLength - len;

		//Possible directions left
		let directions = [
			('U', x, y + len),
			('D', x, y - len),
			('L', x - len, y),
			('R', x + len, y)
		];
		
		//loop for possible paths
		for (dirChar, movedX, movedY) in directions{
			let mut movedPath = path.clone();
			movedPath.push(dirChar);
			stack.push((movedX, movedY, idx + 1, movedPath, newerLength));

		}

	}
    String::new() //Return an empty string
}
    
#[cfg(test)]
#[path = "tests.rs"]
mod tests;

