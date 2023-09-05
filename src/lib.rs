use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs, io,
};

#[derive(Debug, Clone)]
pub enum Prerequisites {
    And(HashSet<String>),
    Or(HashSet<String>),
}

#[derive(Debug, Clone)]
pub struct Technology {
    id: String,
    name: String,
    description: String,
    prerequisites: Prerequisites,
    cost: u32,
}

#[derive(Debug)]
pub struct TechnologyTree {
    technologies: HashMap<String, Technology>,
}

#[derive(Eq, PartialEq)]
struct Node {
    tech_id: String,
    cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TechnologyTree {
    pub fn new() -> Self {
        Self {
            technologies: HashMap::new(),
        }
    }

    pub fn add_technology(&mut self, technology: Technology) {
        self.technologies.insert(technology.id.clone(), technology);
    }

    pub fn remove_technology(&mut self, technology_id: &str) -> Result<(), String> {
        for tech in self.technologies.values() {
            match &tech.prerequisites {
                Prerequisites::And(prereqs) => {
                    if prereqs.contains(technology_id) {
                        return Err(format!(
                            "Technology {} is a prerequisite for {}",
                            technology_id, tech.id
                        ));
                    }
                }
                Prerequisites::Or(prereqs) => {
                    if prereqs.contains(technology_id) {
                        return Err(format!(
                            "Technology {} is a prerequisite for {}",
                            technology_id, tech.id
                        ));
                    }
                }
            }
        }
        self.technologies.remove(technology_id);
        Ok(())
    }

    pub fn is_unlockable(
        &self,
        tech_id: &str,
        unlocked: &HashSet<String>,
        science_points: u32,
    ) -> bool {
        if let Some(tech) = self.technologies.get(tech_id) {
            match &tech.prerequisites {
                Prerequisites::And(prereqs) => {
                    if prereqs.is_subset(unlocked) && tech.cost <= science_points {
                        return true;
                    }
                }
                Prerequisites::Or(prereqs) => {
                    if prereqs.intersection(unlocked).count() > 0 && tech.cost <= science_points {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn unlock_technology(
        &self,
        tech_id: &str,
        unlocked: &mut HashSet<String>,
        science_points: u32,
    ) -> bool {
        if self.is_unlockable(tech_id, unlocked, science_points) {
            unlocked.insert(tech_id.to_string());
            return true;
        }
        false
    }

    pub fn get_unlockable_technologies(
        &self,
        unlocked: &HashSet<String>,
        science_points: u32,
    ) -> Vec<String> {
        self.technologies
            .keys()
            .filter(|&tech_id| self.is_unlockable(tech_id, unlocked, science_points))
            .cloned()
            .collect()
    }

    pub fn get_technology_path(
        &self,
        target: &str,
        unlocked: &HashSet<String>,
        science_points: u32,
    ) -> Option<Vec<String>> {
        let mut heap = BinaryHeap::new();
        let mut parent: HashMap<String, String> = HashMap::new();
        let mut visited: HashSet<String> = HashSet::new();

        for tech in unlocked {
            heap.push(Node {
                tech_id: tech.clone(),
                cost: 0,
            });
        }

        while let Some(current) = heap.pop() {
            let current_tech = current.tech_id;
            let current_cost = -current.cost;

            if visited.contains(&current_tech) {
                continue;
            }

            visited.insert(current_tech.clone());

            if &current_tech == target {
                let mut path = Vec::new();
                let mut node = &current_tech;
                while let Some(p) = parent.get(node) {
                    path.push(p.clone());
                    node = p;
                }
                path.reverse();
                return Some(path);
            }

            for (neighbor_id, neighbor) in &self.technologies {
                if !unlocked.contains(neighbor_id)
                    && self.is_unlockable(neighbor_id, unlocked, science_points)
                    && !visited.contains(neighbor_id)
                {
                    parent.insert(neighbor_id.clone(), current_tech.clone());
                    heap.push(Node {
                        tech_id: neighbor_id.clone(),
                        cost: -(current_cost as i32 + neighbor.cost as i32),
                    });
                }
            }
        }

        None
    }

    pub fn print_tech_tree(&self, unlocked: &mut HashSet<String>, indent: usize) {
        let roots: Vec<String> = self
            .technologies
            .iter()
            .filter(|(_, tech)| match &tech.prerequisites {
                Prerequisites::And(prereqs) => prereqs.is_empty() || prereqs.is_subset(unlocked),
                Prerequisites::Or(prereqs) => prereqs.is_empty() || !prereqs.is_disjoint(unlocked),
            })
            .map(|(id, _)| id.clone())
            .collect();

        for root in roots {
            self.print_tech_branch(&root, unlocked, indent);
        }
    }

    fn print_tech_branch(&self, tech_id: &str, unlocked: &mut HashSet<String>, indent: usize) {
        if let Some(tech) = self.technologies.get(tech_id) {
            println!(
                "{}- {} (Cost: {})",
                " ".repeat(indent),
                tech.name,
                tech.cost
            );

            unlocked.insert(tech_id.to_string());

            for (neighbor_id, _) in &self.technologies {
                // Check if this neighbor is a child of the current technology.
                let is_child = match &self.technologies[neighbor_id].prerequisites {
                    Prerequisites::And(prereqs) => prereqs.contains(tech_id),
                    Prerequisites::Or(prereqs) => prereqs.contains(tech_id),
                };

                // If it is a child and it is unlockable, print it and go deeper.
                if is_child && self.is_unlockable(neighbor_id, unlocked, u32::MAX) {
                    self.print_tech_branch(neighbor_id, unlocked, indent + 4);
                }
            }

            // Remove the current technology from the unlocked set before returning.
            unlocked.remove(tech_id);
        }
    }

    pub fn serialize(&self) -> String {
        let mut serialized_data = Vec::new();

        for (tech_id, tech) in &self.technologies {
            let prereqs = match &tech.prerequisites {
                Prerequisites::And(set) => format!(
                    "And:{}",
                    set.iter().cloned().collect::<Vec<String>>().join(",")
                ),
                Prerequisites::Or(set) => format!(
                    "Or:{}",
                    set.iter().cloned().collect::<Vec<String>>().join(",")
                ),
            };

            serialized_data.push(format!(
                "{};{};{};{};{}",
                tech_id, tech.name, tech.description, prereqs, tech.cost
            ));
        }

        serialized_data.join("\n")
    }

    pub fn deserialize(data: &str) -> Self {
        let mut technologies = HashMap::new();

        for line in data.lines() {
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() == 5 {
                let (tech_id, name, description, prereqs, cost) =
                    (parts[0], parts[1], parts[2], parts[3], parts[4]);
                let prereq_parts: Vec<&str> = prereqs.split(':').collect();
                let prereq_set: HashSet<String> = prereq_parts[1]
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect();

                let prerequisites = match prereq_parts[0] {
                    "And" => Prerequisites::And(prereq_set),
                    "Or" => Prerequisites::Or(prereq_set),
                    _ => continue,
                };

                let technology = Technology {
                    id: tech_id.to_string(),
                    name: name.to_string(),
                    description: description.to_string(),
                    prerequisites,
                    cost: cost.parse::<u32>().unwrap_or(0),
                };
                println!("Loaded technology: {:?}", technology);
                technologies.insert(tech_id.to_string(), technology);
            }
        }

        TechnologyTree { technologies }
    }

    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let data = fs::read_to_string(filename)?;
        let tech_tree = TechnologyTree::deserialize(&data);
        println!("Loaded tech tree from {}", filename);
        Ok(tech_tree)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_technology() {
        let mut tech_tree = TechnologyTree::new();
        let tech = Technology {
            id: "pottery".to_string(),
            name: "Pottery".to_string(),
            description: "Basic pottery techniques.".to_string(),
            prerequisites: Prerequisites::And(HashSet::new()),
            cost: 5,
        };

        tech_tree.add_technology(tech.clone());

        assert!(tech_tree.technologies.contains_key(&tech.id));
    }

    #[test]
    fn test_is_unlockable() {
        let mut tech_tree = TechnologyTree::new();
        let mut unlocked = HashSet::new();
        unlocked.insert("pottery".to_string());

        let tech = Technology {
            id: "writing".to_string(),
            name: "Writing".to_string(),
            description: "Basics of writing.".to_string(),
            prerequisites: Prerequisites::And(unlocked.clone()),
            cost: 10,
        };

        tech_tree.add_technology(tech.clone());

        assert!(tech_tree.is_unlockable(&tech.id, &unlocked, 15));
    }

    #[test]
    fn test_remove_technology() {
        let mut tech_tree = TechnologyTree::new();
        let tech_id = "pottery".to_string();
        let tech = Technology {
            id: tech_id.clone(),
            name: "Pottery".to_string(),
            description: "Basic pottery techniques.".to_string(),
            prerequisites: Prerequisites::And(HashSet::new()),
            cost: 5,
        };
        tech_tree.add_technology(tech);

        assert!(tech_tree.remove_technology(&tech_id).is_ok());
        assert!(!tech_tree.technologies.contains_key(&tech_id));
    }

    #[test]
    fn test_remove_technology_with_dependency() {
        let mut tech_tree = TechnologyTree::new();
        let mut prereq = HashSet::new();
        prereq.insert("pottery".to_string());

        let tech1 = Technology {
            id: "pottery".to_string(),
            name: "Pottery".to_string(),
            description: "Basic pottery techniques.".to_string(),
            prerequisites: Prerequisites::And(HashSet::new()),
            cost: 5,
        };

        let tech2 = Technology {
            id: "irrigation".to_string(),
            name: "Irrigation".to_string(),
            description: "Advanced irrigation techniques.".to_string(),
            prerequisites: Prerequisites::And(prereq.clone()),
            cost: 10,
        };

        tech_tree.add_technology(tech1);
        tech_tree.add_technology(tech2);

        assert!(tech_tree.remove_technology("pottery").is_err());
    }

    #[test]
    fn test_unlock_technology() {
        let mut tech_tree = TechnologyTree::new();
        let mut unlocked = HashSet::new();
        unlocked.insert("pottery".to_string());

        let tech = Technology {
            id: "writing".to_string(),
            name: "Writing".to_string(),
            description: "Basics of writing.".to_string(),
            prerequisites: Prerequisites::And(unlocked.clone()),
            cost: 10,
        };

        tech_tree.add_technology(tech.clone());

        assert!(tech_tree.unlock_technology(&tech.id, &mut unlocked, 15));
    }
}
