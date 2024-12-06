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
    rules: Vec<Rule>,
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
            .map(|update| self.expected_order(update))
            .map(|update| update[update.len() / 2])
            .sum()
    }

    fn expected_order(&self, update: &[u32]) -> Vec<u32> {
        let mut unsorted_pages: Vec<u32> = update.to_vec();
        let mut sorted_pages = Vec::with_capacity(update.len());

        let mut applicable_rules: Vec<&Rule> = self
            .rules
            .iter()
            .filter(|rule| update.contains(&rule.antecedent) && update.contains(&rule.posterior))
            .collect();

        while !unsorted_pages.is_empty() {
            if let Some(unconstrained_page) = unsorted_pages
                .iter()
                .find(|&&page| !applicable_rules.iter().any(|rule| rule.antecedent == page))
            {
                let unconstrained_page = *unconstrained_page;

                sorted_pages.push(unconstrained_page);

                unsorted_pages.remove(unsorted_pages
                    .iter()
                    .position(|&page| page == unconstrained_page)
                    .unwrap());

                applicable_rules.retain(|rule| rule.posterior != unconstrained_page);
            } else {
                panic!("Unresolvable constraints")
            }
        }

        sorted_pages.reverse();
        sorted_pages
    }

    fn has_correct_order(&self, update: &[u32]) -> bool {
        update == self.expected_order(update)
    }
}

impl FromStr for ManualUpdater {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((rules_block, updates_block)) = s.split_once("\n\n") {
            let rules = rules_block
                .lines()
                .map(Rule::from_str)
                .collect::<Result<Vec<_>, _>>()?;

            let updates = updates_block
                .lines()
                .map(|line| {
                    line.split(',')
                        .map(|page| page.parse::<u32>())
                        .collect::<Result<Vec<u32>, _>>()
                })
                .collect::<Result<Vec<Vec<u32>>, _>>()?;

            Ok(ManualUpdater { rules, updates })
        } else {
            Err("Could not parse constraints/updates".into())
        }
    }
}

struct Rule {
    antecedent: u32,
    posterior: u32,
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((antecedent, posterior)) = s.split_once('|') {
            let antecedent = antecedent.parse::<u32>()?;
            let posterior = posterior.parse::<u32>()?;

            Ok(Rule {
                antecedent,
                posterior,
            })
        } else {
            Err("Could not parse rule".into())
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
    fn test_expected_order() {
        let manual_updater = ManualUpdater::from_str(TEST_RULES_AND_UPDATES).unwrap();

        assert_eq!(
            vec![97, 75, 47, 61, 53],
            manual_updater.expected_order(&[75, 97, 47, 61, 53])
        );
        assert_eq!(
            vec![61, 29, 13],
            manual_updater.expected_order(&[61, 13, 29])
        );
        assert_eq!(
            vec![97, 75, 47, 29, 13],
            manual_updater.expected_order(&[97, 13, 75, 29, 47])
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
