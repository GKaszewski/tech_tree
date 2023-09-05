# Technology Tree
This is my implementation of a technology tree in Rust. It is based on the [Civ 6 tech tree](https://civ6.gamepedia.com/Technology_(Civ6)).
The sole purpose of this project is to have a ready-to-use technology tree for my other projects.

## Features
- Add/Remove technologies dynamically
- Check if a technology is unlockable based on prerequisites
- Unlock technologies
- Serialize and deserialize the tech tree to and from a file
- Print out the tech tree in a hierarchical manner

## Installation
Add the following to your `Cargo.toml` file:
```toml
[dependencies]
tech_tree = { git = "https://github.com/GKaszewski/tech_tree.git" }
```

## Quick Start
tech tree in txt file
```txt
pottery;Pottery;Basic pottery techniques.;And:;5
mining;Mining;Basics of mining.;And:;5
irrigation;Irrigation;Advanced irrigation techniques.;And:pottery;10
writing;Writing;Basics of writing.;And:pottery;10
masonry;Masonry;Basics of masonry.;And:mining;10
education;Education;Advanced education techniques.;And:writing;20
```

main.rs
```rust
use std::collections::HashSet;
use tech_tree::TechnologyTree;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    match TechnologyTree::load_from_file("tech_tree.txt") {
        Ok(tech_tree) => {
            let mut unlocked_techs = HashSet::new();
            tech_tree.print_tech_tree(&mut unlocked_techs, 0);
        }
        Err(e) => {
            println!("Failed to load tech tree: {}", e);
        }
    }
    Ok(())
}
```

## Testing
Run `cargo test` to run the tests.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap
- [x] Add/Remove technologies dynamically
- [x] Check if a technology is unlockable based on prerequisites
- [x] Unlock technologies
- [x] Serialize and deserialize the tech tree to and from a file
- [x] Print out the tech tree in a hierarchical manner
- [x] Add unit tests
- [ ] Add serde support
- [ ] Add a GUI
- [ ] Add a CLI