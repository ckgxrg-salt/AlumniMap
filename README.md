# AlumniMap
A web application that displays where our classmates went after graduation.   
Written in pure Rust with actix-web & egui and  .

A departing gift to my classmates and my high school life.   

# Build
## With Nix
Just run this:   
```
$ nix build github:ckgxrg-salt/AlumniMap#alumnimap
```

## Without Nix
cd into frontend/ directory and build the frontend using trunk   
```
$ trunk build --release
```
Then cd back and build the binary with   
```
$ cargo build --bin alumnimap --release
```
The previously built frontend code will be automatically embedded.   

The result is a single binary `alumnimap`.   

# Usage
Before using, we'll need some assets, namely the icon of the schools and avatars of your classmates.   
Make an assets/ directory like this:   
```
assets/
    avatars/
        avatar01.png
        ...
    icons/
        icon01.png
        ...
```
And dump any relevant files into the directory.   

Now, refer to config/example.toml and write a configuration file.   
- `database_uri`, `listen_address`, `port` should be clear enough
- `assets_root` is path to the assets directory previously made
- `base` is a point on the map that all other points will connect to

After written the config, we can now launch `alumnimap`.   
First, we should do   
```
$ alumnimap migrate
```
to migrate the database.   

Then, use
```
$ alumnimap add [university | profile]
```
to add data to the database.   

When all data is prepared, run
```
$ alumnimap server
```
to run the server, now we can watch and enjoy.   

# API
The backend provides these routes:
- /ping: Just returns "Up and running"
- /universities: List all "universities" in the database
- /universities/{uni_id}: Returns the title of this "university"
- /profiles/{uni_id}: List all profiles to that certain "university"
- /search/universities/{search_text}: Returns all "universities" that contains the `search_text` in their titles
