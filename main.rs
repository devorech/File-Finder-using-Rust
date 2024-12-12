use std::fs;
use std::io;

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
            println!("Current file {} with size {}", file, file_length);

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
    } else {
        println!("\nNow processing the file {}", largest_file);
    }
    return largest_file; // will be NULL if no files match
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
            println!("largest file is {}", file);
        } else if choice == 2 {
            // Pick the smallest file - must be a .csv file and name starts with the prefix "movies_"
            //file = find_smallest_file();
            println!("TO-DO: find smallest file.")
        } else if choice == 3 {
            // Prompt user to enter the file name (any name or file type works)
            //file = findUserFile();
            println!("TO-DO: find user file.")
        } else {
            eprintln!("\nError: Enter a 1, 2 or 3\n");
            continue;
        }

        /* Only proceed if there is a file that matched the user's choice
        if (file != NULL)
        {
            processFile(file);
            free(file);
            break; // Go back to the main menu
        } */
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

/*
* Finds the smallest .csv file in the current directory whose name starts with
* the prefix "movies_". Prints an error message if no such file exists

char* findSmallestFile()
{
    // Open the current directory
    DIR* currDir = opendir(".");
    struct dirent *aDir;
    ssize_t smallestSize = -1;
    char *smallestFile = malloc(sizeof(char) * 100);

    // Go through all the entries
    while((aDir = readdir(currDir)) != NULL)
    {
        if (strstr(aDir->d_name, ".csv") != NULL && strstr(aDir->d_name, "movies_") != NULL)
        {
            // There is a .csv file starting with "movies_", check if it is the biggest file here so far
            struct stat stCurr; // metadata struct that stores the metadata for a file
            stat(aDir->d_name, &stCurr);
            if (stCurr.st_size <= smallestSize || smallestSize < 0)
            {
                // Update the smallest file and size to the current directory entry's name and size
                smallestSize = (size_t)stCurr.st_size;
                char tempBuffer[100];
                sprintf(tempBuffer, "%s", aDir->d_name);
                strcpy(smallestFile, tempBuffer);
            }
        }
    }

    // Print an error message if no file matches the requirements, otherwise return the file for processing
    if (smallestSize < 0)
    {
        // No CSV file found that matches the requirements
        printf("\nError: No CSV file found that matches the requirements (must start with movies_).\n");
    }
    else
    {
        printf("\nNow processing the file %s\n", smallestFile);
    }
    closedir(currDir); // need to do this to avoid memory leak
    return smallestFile; // will be NULL if no files match
}


* Attempts to find the user's specified file name. The file must be a .csv file.
* If no file is found or if the user enters a file that does not end in .csv, the
* program will print out an error message.

char* findUserFile()
{
    // Prompt the user for the file name
    printf("\nEnter the file name: ");
    char *fileName = malloc(sizeof(char) * 100);
    scanf("%s", fileName);

    // Check to see if the file exists in the current directory:
    // 1. Open the current directory
    DIR* currDir = opendir(".");
    struct dirent *aDir;

    // 2. Go through all the entries
    while((aDir = readdir(currDir)) != NULL)
    {
        // Check if the file found and the user's file name are the same
        if (strcmp(aDir->d_name, fileName) == 0)
        {
            // Inform the user their file exists and proceed
            printf("\nNow processing the specified file %s\n", fileName);
            closedir(currDir);
            return fileName;
        }
    }

    // No file with a matching name has been found in the directory, print error message
    printf("\nError: No file found with the name %s\n", fileName);
    closedir(currDir); // need to do this to avoid memory leak
    free(fileName);
    return NULL; // NULL = no files match
}


* Processes the file that the user has provided and creates a new directory and files for each year.

void processFile(char* fileName)
{
    // Get a random number for the directory
    srand(time(0));
    int random = rand() % 100000;

    // Convert this number into a formatted string and make the new directory
    char newDir[200];
    sprintf(newDir, "devorech.movies.%d", random); // devorech = user's ONID
    printf("Created directory with the name %s\n", newDir);
    mkdir(newDir, 0750); // 750 = rwxr-x---

    // Open the file the user provided
    FILE *movieFile = fopen(fileName, "r");
    if (movieFile == NULL)
    {
        printf("Error: Cannot open the movie.");
        return;
    }

    // Read the file line by line (first line skips the leading headers)
    char *currLine = NULL;
    size_t len = 0;
    size_t nread;
    char flBuffer[200];

    fgets(flBuffer, 200, movieFile); // skip the first line
    while ((nread = getline(&currLine, &len, movieFile)) != -1)
    {
        // Get the title and year variable
        char title[100];
        char year[100];

        // For use with strtok_r
        char *saveptr;

        // The first token is the title
        char *token = strtok_r(currLine, ",", &saveptr);
        strcpy(title, token);

        // The next token is the year
        token = strtok_r(NULL, ",", &saveptr);
        strcpy(year, token);

        // Read until the very end after this
        token = strtok_r(NULL, "\n", &saveptr);

        // Create and open the new file if it does not exist
        char pathName[100];
        sprintf(pathName, "%s/%s.txt", newDir, year);
        FILE *yearFile = fopen(pathName, "a"); // creates the file (year).txt if it does not exist, otherwise it opens and adds to it
        fprintf(yearFile, "%s\n", title);
        fflush(yearFile);
        fclose(yearFile);

        // Change permissions of the file
        chmod(pathName, 0640);
    }

    // Free any memory left over that needs to be addressed
    fclose(movieFile);
    if (currLine != NULL)
    {
        free(currLine);
    }
}


* Prompt the user what file they would like to choose

void getUserFile()
{
    // Prompt user to enter a file continually until they don't want to
    while(1)
    {
        int choice;

        // Prompt user for entering options
        printf("\nWhich file you want to process?\n");
        printf("Enter 1 to pick the largest file\n");
        printf("Enter 2 to pick the smallest file\n");
        printf("Enter 3 to specify the name of a file\n");
        printf("Enter a choice from 1 to 3: ");
        scanf("%d", &choice);

        char* file = NULL;
        if (choice == 1)
        {
            // Pick the largest file - must be a .csv file and name starts with the prefix "movies_"
            file = findLargestFile();
        }
        else if (choice == 2)
        {
            // Pick the smallest file - must be a .csv file and name starts with the prefix "movies_"
            file = findSmallestFile();
        }
        else if (choice == 3)
        {
            // Prompt user to enter the file name (any name or file type works)
            file = findUserFile();
        }
        else
        {
            printf("\nError: Enter a 1, 2 or 3\n");
            continue;
        }

        // Only proceed if there is a file that matched the user's choice
        if (file != NULL)
        {
            processFile(file);
            free(file);
            break; // Go back to the main menu
        }

    }
}

*/
