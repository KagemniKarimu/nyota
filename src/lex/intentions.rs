use prefrontal::ClassDefinition;

pub fn user_intentions() -> Vec<ClassDefinition> {
    vec![
        model(),
        api(),
        greeting(),
        farewell(),
        help(),
        thanks(),
        about(),
        unknown(),
        insult(),
        compliment(),
    ]
}

fn model() -> ClassDefinition {
    ClassDefinition::new(
        "model",
        "Questions about the LLM or underlying AI logic of Nyota",
    )
    .with_examples(vec![
        "LLM model",
        "LLM model selection",
        "switch AI models",
        "change LLM model",
        "context window",
    ])
}

fn api() -> ClassDefinition {
    ClassDefinition::new(
        "api",
        "Questions about the API or how Nyota interacts with external services",
    )
    .with_examples(vec![
        "API provider",
        "API configuration",
        "change API provider",
        "switch API provider",
        "API URL",
    ])
}

fn greeting() -> ClassDefinition {
    ClassDefinition::new("greeting", "Intents related to greetings and salutations.").with_examples(
        vec![
            "yo nyota",
            "How are you?",
            "Hello!",
            "Hi there!",
            "Good morning!",
            "Good evening!",
            "Hey!",
            "Greetings Nyota",
            "Hello there.",
        ],
    )
}

fn farewell() -> ClassDefinition {
    ClassDefinition::new(
        "farewell",
        "Intents related to saying goodbye or ending conversations.",
    )
    .with_examples(vec![
        "Goodbye",
        "See you later",
        "Take care!",
        "Bye for now!",
        "I have to go.",
        "Bye.",
        "g2g",
        "cya",
    ])
}

fn help() -> ClassDefinition {
    ClassDefinition::new("help", "Intents related to seeking assistance or guidance.")
        .with_examples(vec![
            "Can you help me?",
            "I need assistance.",
            "Would you be able to help?",
            "I have a question.",
            "Help me out, please.",
        ])
}

fn thanks() -> ClassDefinition {
    ClassDefinition::new("thanks", "Intents related to expressing gratitude.").with_examples(vec![
        "Thank you!",
        "Grateful for what you did.",
        "I appreciate it!",
        "Thanks a lot!",
        "You're a lifesaver!",
        "Thanks for your help!",
    ])
}

fn about() -> ClassDefinition {
    ClassDefinition::new(
        "about",
        "Intents related to inquiring about Nyota or the program.",
    )
    .with_examples(vec![
        "Tell me about yourself.",
        "What can you do?",
        "Give me some information about Nyota.",
        "What is your purpose?",
        "Who created you?",
    ])
}

fn unknown() -> ClassDefinition {
    ClassDefinition::new(
        "unknown",
        "Intents for handling unknown or unrecognized inputs.",
    )
    .with_examples(vec![
        "blslajsdkdshlfshasdjads",
        "09303808130nxonsasasko",
        "I don't understand.",
        "What does that mean?",
        "Can you clarify?",
        "Say that again.",
        "I don't know what you're talking about.",
    ])
}

fn insult() -> ClassDefinition {
    ClassDefinition::new(
        "insult",
        "Intents related to negative or insulting messages.",
    )
    .with_examples(vec![
        "You're useless.",
        "I don't like you.",
        "This is terrible!",
        "What a bad idea!",
        "You're not very helpful.",
        "I hate you.",
    ])
}

fn compliment() -> ClassDefinition {
    ClassDefinition::new(
        "compliment",
        "Intents related to positive or complimentary messages.",
    )
    .with_examples(vec![
        "You're amazing!",
        "I really like you!",
        "Great job!",
        "You're the best!",
        "I appreciate everything you do!",
        "I love you",
    ])
}
