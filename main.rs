use rand::Rng;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt; // for permission changing

/*
* Processes the file that the user has provided and creates a new directory and files for each year.
* This funciton take ownership of the file passed in
*/
fn process_file(file_name: String) {
    // Get a random number for the directory
    let mut rng = rand::thread_rng(); // random number generator
    let rand: String = rng.gen_range(0..100001).to_string(); // create a random number between 0 and 100000

    // Convert this number into a formatted string and make the new directory
    let mut new_dir: String = "./devorech.movies.".to_string();
    new_dir.push_str(&rand);
    println!("Created directory with the name {}", new_dir);
    fs::create_dir(&new_dir).expect("Failed to create directory.");
    // Change the permissions of the directory (0x750 -> 750 = rwxr-x---)
    let new_dir_permissions = fs::Permissions::from_mode(0o750);
    fs::set_permissions(new_dir.clone(), new_dir_permissions).expect("Unable to set directory permissions");

    // Open the file the user provided
    let mut movie_file = fs::File::open(file_name).unwrap();
    let mut mf_contents = String::new();
    movie_file.read_to_string(&mut mf_contents).expect("Unable to read from file");

    // Split the file contents into an iterator, then into an array (vector) of lines
    let mut lines: Vec<&str> = mf_contents.split("\n").collect();
    lines.remove(0); // remove the first line (header)

    // Iterate through each line in the file
    for line in lines {
        // Any line (including the last line) will be blank, so don't try to read this
        if line == ""
        {
            continue;
        }
        // movie_data now contains: [release year, movie title, languages, rating]
        let movie_data: Vec<&str> = line.split(",").collect();
        let title = movie_data[0].trim();
        let year = movie_data[1].trim();
        println!("Current movie is {}, made in {}", title, year);

        // Create a new file (or append to existing file) for the current year containing all movies
        let new_path_name: String = format!("{}/{}.txt", new_dir, year);
        let mut new_file = fs::OpenOptions::new()
            .create(true)
            .append(true) // append if the file already exists, otherwise it makes a new one
            .open(new_path_name.clone())
            .unwrap();
        new_file.write_all(title.as_bytes()).expect("Unable to write to file"); // need to write the title string as bytes rather than the string itself
                                               // Note: no need to close file after opening since rust handles this - continue as normal

        // Change the permissions for the file (0o640 -> 640 = rw-r--r--))
        let new_file_permissions = fs::Permissions::from_mode(0o640);
        fs::set_permissions(new_path_name, new_file_permissions)
            .expect("Unable to set file permissions");
    }
}

/*
* Attempts to find the user's specified file name. The file must be a .csv file.
* If no file is found or if the user enters a file that does not end in .csv, the
* program will print out an error message.
*/
fn find_user_file() -> String {
    // Prompt the user for the file name
    println!("\nEnter the file name: ");
    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read line");
    let file_name: String = file_name.trim().parse().expect("Please enter a filename");

    // Check to see if the file exists in the current directory:
    // 1. Open the current directory
    let path = String::from("./src");
    let curr_dir = fs::read_dir(path).expect("Unable to read directory");

    // 2. Go through all the entries
    for file in curr_dir {
        // Get the current file name and convert it to a string
        // (converts from a Result type -> PathBuf type -> Str type -> option type -> String type)
        let file: String = file
            .unwrap()
            .path()
            .to_str()
            .unwrap()
            .to_string();

        // Check if the file found and the user's file name are the same
        println!("file name is {}", file);
        if file_name == file.replace("./src/", "") {
            // The file exists (proceed)
            return file;
        }
    }

    // No file with a matching name has been found in the directory, print error message
    println!("\nError: No file found with the name {}", file_name);
    return "".to_string();
}

/*
* Finds the smallest .csv file in the current directory whose name starts with
* the prefix "movies_". Prints an error message if no such file exists
*/
fn find_smallest_file() -> String {
    // Open the current directory
    let path = String::from("./src");
    let curr_dir = fs::read_dir(path).unwrap();
    let mut smallest_size: i32 = -1;
    let mut smallest_file = String::new();

    // Go through all the entries
    for file in curr_dir {
        // Get the current file name and convert it to a string
        // (converts from a Result type -> PathBuf type -> Str type -> option type -> String type)
        let file: String = file.unwrap().path().to_str().unwrap().to_string();
        //println!("Current file is {}", file);
        // Check and see if the file ends with (contains) .csv and that it starts with "movies"
        if file.contains(".csv") && file.contains("movies_") {
            // There is a .csv file starting with "movies_", check if it is the biggest file here so far
            let file_length = fs::metadata(file.clone()).unwrap().len() as i32;

            if file_length <= smallest_size || smallest_size < 0 {
                // Update the smallest file and size to the current directory entry's name and size
                smallest_size = file_length;
                smallest_file = file;
            }
        }
    }

    // Print an error message if no file matches the requirements, otherwise return the file for processing
    if smallest_size < 0 {
        // No CSV file found that matches the requirements
        eprintln!(
            "\nError: No CSV file found that matches the requirements (must start with movies_)."
        );
        return "".to_string();
    } else {
        return smallest_file;
    }
}

/*
* Finds the largest .csv file in the current directory whose name starts with
* the prefix "movies_". Prints an error message if no such file exists
*/
fn find_largest_file() -> String {
    // Open the current directory
    let path = String::from("./src");
    let curr_dir = fs::read_dir(path).unwrap();
    let mut largest_size: i32 = -1;
    let mut largest_file = String::new();

    // Go through all the entries
    for file in curr_dir {
        // Get the current file name and convert it to a string
        // (converts from a Result type -> PathBuf type -> Str type -> option type -> String type)
        let file: String = file.unwrap().path().to_str().unwrap().to_string();
        //println!("Current file is {}", file);
        // Check and see if the file ends with (contains) .csv and that it starts with "movies"
        if file.contains(".csv") && file.contains("movies_") {
            // There is a .csv file starting with "movies_", check if it is the biggest file here so far
            let file_length = fs::metadata(file.clone()).unwrap().len() as i32;
            //println!("Current file {} with size {}", file, file_length);

            if file_length >= largest_size || largest_size < 0 {
                // Update the largest file and size to the current directory entry's name and size
                largest_size = file_length;
                largest_file = file;
            }
        }
    }

    // Print an error message if no file matches the requirements, otherwise return the file for processing
    if largest_size < 0 {
        // No CSV file found that matches the requirements
        eprintln!(
            "\nError: No CSV file found that matches the requirements (must start with movies_)."
        );
        return "".to_string();
    } else {
        return largest_file;
    }
}

/*
* Prompt the user what file they would like to choose
*/
fn get_user_file() {
    // Prompt user to enter a file continually until they don't want to
    loop {
        let mut choice = String::new();

        // Prompt user for entering options
        println!("\nWhich file you want to process?\n");
        println!("Enter 1 to pick the largest file\n");
        println!("Enter 2 to pick the smallest file\n");
        println!("Enter 3 to specify the name of a file\n");
        println!("Enter a choice from 1 to 3: ");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        let choice: u32 = choice.trim().parse().expect("Please type a number!");

        let mut file = String::new();
        if choice == 1 {
            // Pick the largest file - must be a .csv file and name starts with the prefix "movies_"
            file = find_largest_file();
        } else if choice == 2 {
            // Pick the smallest file - must be a .csv file and name starts with the prefix "movies_"
            file = find_smallest_file();
        } else if choice == 3 {
            // Prompt user to enter the file name (any name or file type works)
            file = find_user_file();
        } else {
            eprintln!("\nError: Enter a 1, 2 or 3\n");
            continue;
        }

        // Only proceed if there is a file that matched the user's choice
        if file != "" {
            println!("\nNow processing the file {}", file);
            process_file(file);
            break; // Go back to the main menu
        }
    }
}

fn main() {
    // Prompt the user if they would like to enter a file
    loop {
        println!("\n1. Select file to process \n2. Exit the program\n");
        println!("Enter a choice 1 or 2: ");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // Code adapted from Rust "Exploration: Variables and Data Types"
        let choice: String = choice.trim().parse().expect("Please type a number!");

        if choice == "1" {
            // Prompt the user to provide a file for processing
            get_user_file()
        } else if choice == "2" {
            // End the program
            break;
        } else {
            // Use enters a number that is not 1 or 2 (re-prompt for options)
            eprintln!("\nError: Enter either a 1 or 2.\n");
        }
    }
}
