use crate::utils::context::TestContext;
use crate::utils::process_transaction_assert_success;
use crate::utils::program::{create_test_account, create_user, Program};
use anchor_lang::AccountDeserialize;
use blackhat::processor::TestAccountV1;
use lighthouse::types::{Assertion, DataValue, Operator};
use solana_program::pubkey::Pubkey;
use solana_program_test::tokio;
use solana_sdk::transaction::Transaction;
use solana_sdk::{signature::Keypair, signer::EncodableKeypair};
use std::{collections::HashSet, hash::Hash};

fn distribute<T: Eq + PartialEq + PartialOrd + Ord + Clone + Hash>(elements: &Vec<T>) -> Vec<T> {
    let unique_letters: HashSet<_> = elements.iter().collect();
    let modulo_divisor = unique_letters.len();
    let mut result: Vec<T> = Vec::new();
    let mut occurrences = vec![0; modulo_divisor];
    let mut element_indices: Vec<_> = unique_letters.into_iter().enumerate().collect();

    // Sort the unique letters to maintain consistent order
    element_indices.sort_by(|a, b| a.1.cmp(b.1));

    while result.len() < elements.len() {
        for (index, element) in &element_indices {
            let index = *index;

            let count = elements.iter().filter(|x| x == element).count();
            if occurrences[index] < count {
                result.push((*element).clone());
                occurrences[index] += 1;
            }
        }
    }

    result
}

fn find_smallest_pattern<T: Eq + PartialEq + PartialOrd + Ord + Clone + Hash + std::fmt::Debug>(
    element: Vec<T>,
) -> Option<Vec<T>> {
    let unique_elements = element.iter().collect::<HashSet<_>>();
    let unique_char_size = unique_elements.len();

    let mut pattern = Vec::new();
    let mut seen_elements = HashSet::new();
    let mut index_of_end_pattern = 0;

    for i in 0..element.len() {
        let letter = &element[i];

        if !seen_elements.contains(&letter) {
            pattern.push(letter.clone());
            seen_elements.insert(letter);
        }

        if pattern.len() == unique_char_size {
            index_of_end_pattern = i + 1;
            break;
        }
    }

    let mut deduped_elements = pattern.clone();

    let mut i = index_of_end_pattern;

    println!("Pattern: {:?}", pattern);

    while i < element.len() {
        let end_range = usize::min(element.len(), i + pattern.len());
        let remaining_slice = &element[i..end_range];

        if pattern.starts_with(remaining_slice) {
            i += remaining_slice.len()
        } else {
            deduped_elements.append(&mut element[i..end_range].to_vec());

            // deduped_elements = element.clone();
            break;
        }
    }

    if deduped_elements.len() == element.len() {
        return None;
    }

    Some(deduped_elements)
}

#[tokio::test]
async fn test_pattern() {
    let test_cases = vec![
        (vec!['A', 'A', 'B', 'B', 'C'], vec!['A', 'B', 'C', 'A', 'B']),
        (
            vec!['A', 'A', 'A', 'B', 'B', 'B'],
            vec!['A', 'B', 'A', 'B', 'A', 'B'],
        ),
        (vec!['A', 'B', 'C', 'D', 'E'], vec!['A', 'B', 'C', 'D', 'E']),
        (vec!['B', 'B', 'B', 'A'], vec!['A', 'B', 'B', 'B']),
    ];

    for (input, expected) in test_cases {
        let distributed_letters = distribute(&input);
        println!("{:?} ", distributed_letters);
        assert_eq!(distributed_letters, expected);
    }

    let examples: Vec<(Vec<char>, Option<Vec<char>>)> = vec![
        (vec!['A', 'B', 'A', 'B', 'A', 'B'], Some(vec!['A', 'B'])),
        (
            vec!['A', 'B', 'C', 'A', 'B', 'C'],
            Some(vec!['A', 'B', 'C']),
        ),
        (vec!['A', 'B', 'A'], Some(vec!['A', 'B'])),
        (vec!['A', 'B', 'B', 'B'], None),
        (vec!['A', 'B', 'B', 'C'], None),
        (vec!['A', 'B', 'C', 'D'], None),
        (
            vec!['A', 'A', 'A', 'A', 'B', 'B', 'C'],
            Some(vec!['A', 'B', 'C', 'A', 'B', 'A']),
        ),
        (
            vec!['A', 'B', 'A', 'B', 'A', 'B', 'C'],
            Some(vec!['A', 'B', 'C', 'A', 'B', 'A']),
        ),
    ];

    for (letters, expected) in examples {
        let distribution = distribute(&letters);
        let pattern = find_smallest_pattern(distribution.clone());
        println!(
            "Distribution: {:?} Original: {:?}, Pattern: {:?}",
            distribution, letters, pattern
        );
        assert_eq!(pattern, expected);
    }

    let input = vec![(vec!["ABC", "ABC", "CBA", "CBA"], Some(vec!["ABC", "CBA"]))];

    for (letters, expected) in input {
        let distribution = distribute(&letters);
        let pattern = find_smallest_pattern(distribution.clone());
        println!(
            "Distribution: {:?} Original: {:?}, Pattern: {:?}",
            distribution, letters, pattern
        );
        assert_eq!(pattern, expected);
    }

    let pubkey_a = Keypair::new().encodable_pubkey();
    let pubkey_b = Keypair::new().encodable_pubkey();

    let input = vec![(
        vec![pubkey_a, pubkey_b, pubkey_a, pubkey_b],
        if pubkey_a < pubkey_b {
            Some(vec![pubkey_a, pubkey_b])
        } else {
            Some(vec![pubkey_b, pubkey_a])
        },
    )];

    for (letters, expected) in input {
        let distribution = distribute(&letters);
        let pattern = find_smallest_pattern(distribution.clone());
        println!(
            "Distribution: {:?} Original: {:?}, Pattern: {:?}",
            distribution, letters, pattern
        );
        assert_eq!(pattern, expected);
    }
}

///
/// Tests all data types using the `AccountData` assertion.
///
#[tokio::test]
async fn test_dedupe() {
    let context = &mut TestContext::new().await.unwrap();
    let program = Program::new(context.client());
    let user = create_user(context).await.unwrap();

    let keypair_a = Keypair::new();

    create_test_account(context, &user, &keypair_a, true)
        .await
        .unwrap();

    let keypair_b = Keypair::new();

    create_test_account(context, &user, &keypair_b, true)
        .await
        .unwrap();

    let test_account_a = TestAccountV1::try_deserialize(
        &mut context
            .client()
            .get_account(keypair_a.encodable_pubkey())
            .await
            .unwrap()
            .unwrap()
            .data
            .as_slice(),
    )
    .unwrap();

    let test_account_b = TestAccountV1::try_deserialize(
        &mut context
            .client()
            .get_account(keypair_b.encodable_pubkey())
            .await
            .unwrap()
            .unwrap()
            .data
            .as_slice(),
    )
    .unwrap();

    let keypair_c = Keypair::new();

    create_test_account(context, &user, &keypair_c, true)
        .await
        .unwrap();

    let test_account_c = TestAccountV1::try_deserialize(
        &mut context
            .client()
            .get_account(keypair_c.encodable_pubkey())
            .await
            .unwrap()
            .unwrap()
            .data
            .as_slice(),
    )
    .unwrap();

    println!(
        "Test Account A: {:?} {}",
        test_account_a,
        keypair_a.encodable_pubkey()
    );
    println!(
        "Test Account B: {:?} {}",
        test_account_b,
        keypair_b.encodable_pubkey()
    );

    let tx = build_assertions(
        &mut program,
        &user,
        vec![
            PubkeyWithAssertion {
                pubkey: keypair_a.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_a.u8),
                ),
            },
            PubkeyWithAssertion {
                pubkey: keypair_b.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_b.u8),
                ),
            },
            PubkeyWithAssertion {
                pubkey: keypair_b.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_b.u8),
                ),
            },
            PubkeyWithAssertion {
                pubkey: keypair_b.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_b.u8),
                ),
            },
        ],
    )
    .await;

    assert_eq!(tx.clone().message.instructions[0].accounts.len(), 5);

    process_transaction_assert_success(context, Ok(tx)).await;

    let tx = build_assertions(
        &mut program,
        &user,
        vec![
            PubkeyWithAssertion {
                pubkey: keypair_a.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_a.u8),
                ),
            },
            PubkeyWithAssertion {
                pubkey: keypair_b.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_b.u8),
                ),
            },
            PubkeyWithAssertion {
                pubkey: keypair_a.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_a.u8),
                ),
            },
            PubkeyWithAssertion {
                pubkey: keypair_b.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_b.u8),
                ),
            },
            PubkeyWithAssertion {
                pubkey: keypair_a.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_a.u8),
                ),
            },
            PubkeyWithAssertion {
                pubkey: keypair_b.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_b.u8),
                ),
            },
        ],
    )
    .await;

    assert_eq!(tx.message.instructions[0].accounts.len(), 3);
    process_transaction_assert_success(context, Ok(tx)).await;

    let tx = build_assertions(
        &mut program,
        &user,
        vec![
            PubkeyWithAssertion {
                pubkey: keypair_a.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_a.u8),
                ),
            },
            PubkeyWithAssertion {
                pubkey: keypair_b.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_b.u8),
                ),
            },
            PubkeyWithAssertion {
                pubkey: keypair_a.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_a.u8),
                ),
            },
            PubkeyWithAssertion {
                pubkey: keypair_b.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_b.u8),
                ),
            },
            PubkeyWithAssertion {
                pubkey: keypair_a.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_a.u8),
                ),
            },
            PubkeyWithAssertion {
                pubkey: keypair_b.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_b.u8),
                ),
            },
            PubkeyWithAssertion {
                pubkey: keypair_c.encodable_pubkey(),
                assertion: Assertion::AccountData(
                    8,
                    Operator::Equal,
                    DataValue::U8(test_account_c.u8),
                ),
            },
        ],
    )
    .await;

    assert_eq!(tx.message.instructions[0].accounts.len(), 7);
    process_transaction_assert_success(context, Ok(tx)).await;
}

pub async fn build_assertions(
    program: &mut Program,
    payer: &Keypair,
    pubkey_with_assertion: Vec<PubkeyWithAssertion>,
) -> Transaction {
    let distributed_pubkeys = distribute(&pubkey_with_assertion);
    let minimum = find_smallest_pattern(distributed_pubkeys.clone());
    let accounts = minimum.clone().unwrap_or(distributed_pubkeys.clone());

    println!(
        "original {:?}",
        pubkey_with_assertion
            .clone()
            .iter()
            .map(|a| a.pubkey)
            .collect::<Vec<_>>()
    );

    println!(
        "Distributed: {:?}",
        distributed_pubkeys
            .iter()
            .map(|a| a.pubkey)
            .collect::<Vec<_>>()
    );
    println!(
        "Minimum: {:?}",
        minimum.map(|m| m.iter().map(|a| a.pubkey).collect::<Vec<_>>())
    );
    println!(
        "Accounts: {:?}",
        accounts.iter().map(|a| a.pubkey).collect::<Vec<_>>()
    );

    program
        .create_assert_multi(
            payer,
            distributed_pubkeys
                .iter()
                .map(|a| a.assertion.clone())
                .collect(),
            accounts.iter().map(|a| a.pubkey).collect(),
        )
        .to_transaction()
        .await
        .unwrap()
}

#[derive(Debug, Clone)]
pub struct PubkeyWithAssertion {
    pubkey: Pubkey,
    assertion: Assertion,
}

impl PartialEq for PubkeyWithAssertion {
    fn eq(&self, other: &Self) -> bool {
        self.pubkey.eq(&other.pubkey)
    }
}

impl Eq for PubkeyWithAssertion {}

impl PartialOrd for PubkeyWithAssertion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.pubkey.cmp(&other.pubkey))
    }
}

impl Hash for PubkeyWithAssertion {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pubkey.hash(state);
    }
}

impl Ord for PubkeyWithAssertion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pubkey.cmp(&other.pubkey)
    }
}
