use simplicio::*;
use std::io::Write;
use lazy_static::lazy_static;

lazy_static! {
    static ref CARDS: std::sync::RwLock<Vec<Card>> = std::sync::RwLock::new(vec![]);
}

static BOLD: &str = "\x1b[1m";
static ITALIC: &str = "\x1b[3m";
static UNDERLINE: &str = "\x1b[4m";
static RESET: &str = "\x1b[0m";


pub fn add_card(card: Card) {
    let mut cards = CARDS.write().expect("Could not add new section.");
    cards.push(card);
}

#[derive(Clone, PartialEq, Eq)]
pub struct Card {
    pub card_no: usize,
    pub topics: Vec<String>,
    pub questions: Vec<String>,
    pub answers: Vec<String>,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{:#?}\n{:#?}\n{:#?}", 
            self.card_no, self.topics, self.questions, self.answers,
        )
    }
}

impl Card {
    pub fn new(card_no: usize, questions: Vec<String>) -> Self {
        Self {
            card_no,
            questions,
            answers: vec![],
            topics: vec![],
        }    
    }

    pub fn answer(&mut self, answer: Vec<String>) {
        self.answers = answer;
    }

    pub fn tags(&mut self, tags: Vec<String>) {
        self.topics = tags;
    }

    pub fn add_question(&mut self, question: &str) {
        self.questions.push(s!(question));
    }

    pub fn add_answer(&mut self, answer: &str) {
        self.answers.push(s!(answer));
    }

    pub fn add_topic(&mut self, topic: &str) {
        self.topics.push(s!(topic));
    }
}

pub fn read_input() -> String {
    let mut input = s!();
    loop {
        std::io::stdout().flush().expect("Could not flush buffer");
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        if !input.trim().is_empty() {
            input = input.replace("<b>", BOLD).replace(r#"<\b>"#, RESET)
                .replace("<i>", ITALIC).replace(r#"<\i>"#, RESET)
                .replace("<u>", UNDERLINE).replace(r#"<\u>"#, RESET);

            return input;
        }
    }
}

pub fn clear_terminal() {
    println!("\x1b[2J\x1b[1;1H");
    std::io::stdout().flush().expect("Could not clear terminal");
}

pub fn main_menu() {
    loop {
        clear_terminal();
        println!("== Main Menu ==");
        println!("A: Create Card");
        println!("L: List Cards");
        println!("M: Manage Cards");
        println!("I: Import");
        println!("E: Export");
        println!("Q: Quit\n");
        println!("Enter your choice: ");
    

        match read_input().to_lowercase().trim() {
            "a" => create_card(),
            "l" => list_cards(),
            "m" => manage_cards(),
            "i" => import(),
            "e" => export(),
            "q" => break,
            _ => {
                println!("Invalid option, please try again.");
                std::thread::sleep(std::time::Duration::from_secs(1));
            },
        }
    }
}

pub fn create_card() {
    let cards;
    {
        let c = CARDS.read().expect("Could not access card list");
        cards = c.clone();
    }

    clear_terminal();
    println!("== Create Card: Card {} ==", cards.len());
    println!("Enter the question one line at a time");
    println!("Enter 'DONE!' when finished");

    let mut questions = vec![];

    loop { // Question loop
        let line = read_input();

        if &line.trim().to_lowercase() == "done!" { 
            println!("\nWould you like to save this input?");
            println!("Y: Yes  || N: No");

            match read_input().to_lowercase().trim() {
                "y" => {
                    println!("The input has been saved");
                    break;
                },
                "n" => {
                    questions.clear();
                    clear_terminal();
                    println!("Enter the question one line at a time");
                    println!("Enter 'DONE!' when finished");
                    continue;
                },
                _ => {
                    println!("Invalid character, please provide a valid response");
                }
            }
        }
        questions.push(line);
    }
    
    clear_terminal();

    let mut card = Card::new(cards.len(), questions);

    println!("Would you like to add answers to this card?");
    println!("Y: Yes  || N: No");
    loop { // Answer Loop
        match read_input().to_lowercase().trim() {
            "y" => {
                println!("Enter the answer(s) one line at a time");
                println!("Enter 'DONE!' when finished");

                let mut answers = vec![];
                loop {
                    let line = read_input();

                    if &line.trim().to_lowercase() == "done!" {
                        println!("\nWould you like to save this input?");
                        println!("Y: Yes  || N: No");

                        match read_input().to_lowercase().trim() {
                            "y" => {
                                card.answers = answers;
                                println!("The input has been saved");
                                break;
                            },
                            "n" => {
                                answers.clear();
                                clear_terminal();
                                println!("Enter the answer(s) one line at a time");
                                println!("Enter 'DONE!' when finished");
                                continue;
                            },
                            _ => {
                                println!("Invalid character, please provide a valid response");
                            }
                        }
                    }
                    answers.push(line);
                }
            },
            "n" => break,
            _ => {
                println!("Invalid character, please provide a valid response");
            }
        }
    }

    clear_terminal();

    println!("Would you like to add topics to this card?");
    println!("Y: Yes  || N: No");
    loop { // Topics loop
        match read_input().to_lowercase().trim() {
            "y" => {
                println!("Enter the topics one line at a time. Enter 'DONE!' when finished");

                let mut topics = vec![];
                loop {
                    let line = read_input();

                    if &line.trim().to_lowercase() == "done!" {
                        println!("\nWould you like to save this input?");
                        println!("Y: Yes  || N: No");

                        match read_input().to_lowercase().trim() {
                            "y" => {
                                card.topics = topics;
                                println!("The input has been saved");
                                break;
                            },
                            "n" => {
                                topics.clear();
                                clear_terminal();
                                println!("Enter the topic(s) one line at a time");
                                println!("Enter 'DONE!' when finished");
                                continue;
                            },
                            _ => {
                                println!("Invalid character, please provide a valid response");
                            }
                        }
                    }
                    topics.push(line);
                }
            },
            "n" => break,
            _ => {
                println!("Invalid character, please provide a valid response");
            }
        }
    }

    add_card(card);
}

pub fn list_cards() {
    let cards = CARDS.read().expect("Could not access card list");
    for card in cards.iter() {
        println!("{}", s!(card));
    }

    println!("Enter `Q` when you are done");

    while read_input().trim().to_lowercase() != "q" { }
}

pub fn manage_cards() {
    
}

pub fn import() {
    
}

pub fn export() {
    
}

fn main() {
    main_menu();
}
