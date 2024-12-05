use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let manual_updater = {
            let mut string = String::new();
            File::open(path)?.read_to_string(&mut string)?;

            ManualUpdater::from_str(&string)?
        };

        println!(
            "Sum of middle pages from correctly-ordered updates: {}",
            manual_updater.middle_page_sum_from_correct_updates()
        );

        println!(
            "Sum of middle pages from repaired, incorrectly-ordered updates: {}",
            manual_updater.middle_page_sum_from_repaired_incorrect_updates()
        );

        Ok(())
    } else {
        Err("Usage: day05 INPUT_FILE_PATH".into())
    }
}

struct ManualUpdater {
    constraints: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
}

impl ManualUpdater {
    pub fn middle_page_sum_from_correct_updates(&self) -> u32 {
        self.updates
            .iter()
            .filter(|update| self.has_correct_order(update))
            .map(|update| update[update.len() / 2])
            .sum()
    }

    pub fn middle_page_sum_from_repaired_incorrect_updates(&self) -> u32 {
        self.updates
            .iter()
            .filter(|update| !self.has_correct_order(update))
            .map(|update| self.repair_update(update))
            .map(|update| update[update.len() / 2])
            .sum()
    }

    fn has_correct_order(&self, update: &[u32]) -> bool {
        for i in 1..update.len() {
            if let Some(constraints) = self.constraints.get(&update[i]) {
                if update[0..i].iter().any(|page| constraints.contains(page)) {
                    return false;
                }
            }
        }

        true
    }

    fn repair_update(&self, update: &[u32]) -> Vec<u32> {
        let mut unresolved_constraints = HashMap::new();

        update.iter().for_each(|page| {
            if let Some(constraints) = self.constraints.get(page) {
                unresolved_constraints.insert(
                    *page,
                    constraints
                        .iter()
                        .filter(|page| update.contains(page))
                        .to_owned()
                        .collect(),
                );
            } else {
                unresolved_constraints.insert(*page, Vec::new());
            }
        });

        let mut repaired_update = Vec::with_capacity(update.len());

        while !unresolved_constraints.is_empty() {
            // One page in the update should have no constraints, and we can push it into the
            // repaired order
            if let Some(page) = unresolved_constraints
                .iter()
                .filter(|(_, constraints)| constraints.is_empty())
                .map(|(page, _)| *page)
                .next()
            {
                repaired_update.push(page);
                unresolved_constraints.remove(&page);

                unresolved_constraints
                    .iter_mut()
                    .for_each(|(_, constraints)| {
                        if let Some(i) = constraints
                            .iter()
                            .position(|&&constraint| constraint == page)
                        {
                            constraints.remove(i);
                        }
                    });
            } else {
                panic!("Unresolvable constraints");
            }
        }

        repaired_update.reverse();

        repaired_update
    }
}

impl FromStr for ManualUpdater {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((constraints_block, updates_block)) = s.split_once("\n\n") {
            let mut constraints = HashMap::new();

            for line in constraints_block.lines() {
                if let Some((page, before)) = line.split_once('|') {
                    let page = page.parse::<u32>()?;
                    let before = before.parse::<u32>()?;

                    constraints.entry(page).or_insert(Vec::new()).push(before);
                }
            }

            let updates = updates_block
                .lines()
                .map(|line| {
                    line.split(',')
                        .map(|page| page.parse::<u32>())
                        .collect::<Result<Vec<u32>, _>>()
                })
                .collect::<Result<Vec<Vec<u32>>, _>>()?;

            Ok(ManualUpdater {
                constraints,
                updates,
            })
        } else {
            Err("Could not parse constraints/updates".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_RULES_AND_UPDATES: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13
        
        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn test_has_correct_order() {
        let manual_updater = ManualUpdater::from_str(TEST_RULES_AND_UPDATES).unwrap();

        assert!(manual_updater.has_correct_order(&manual_updater.updates[0]));
        assert!(manual_updater.has_correct_order(&manual_updater.updates[1]));
        assert!(manual_updater.has_correct_order(&manual_updater.updates[2]));
        assert!(!manual_updater.has_correct_order(&manual_updater.updates[3]));
        assert!(!manual_updater.has_correct_order(&manual_updater.updates[4]));
        assert!(!manual_updater.has_correct_order(&manual_updater.updates[5]));
    }

    #[test]
    fn test_middle_page_sum_from_correct_updates() {
        let manual_updater = ManualUpdater::from_str(TEST_RULES_AND_UPDATES).unwrap();
        assert_eq!(143, manual_updater.middle_page_sum_from_correct_updates());
    }

    #[test]
    fn test_repair_update() {
        let manual_updater = ManualUpdater::from_str(TEST_RULES_AND_UPDATES).unwrap();

        assert_eq!(
            vec![97, 75, 47, 61, 53],
            manual_updater.repair_update(&[75, 97, 47, 61, 53])
        );
        assert_eq!(
            vec![61, 29, 13],
            manual_updater.repair_update(&[61, 13, 29])
        );
        assert_eq!(
            vec![97, 75, 47, 29, 13],
            manual_updater.repair_update(&[97, 13, 75, 29, 47])
        );
    }

    #[test]
    fn test_middle_page_sum_from_repaired_incorrect_updates() {
        let manual_updater = ManualUpdater::from_str(TEST_RULES_AND_UPDATES).unwrap();
        assert_eq!(
            123,
            manual_updater.middle_page_sum_from_repaired_incorrect_updates()
        );
    }
}
