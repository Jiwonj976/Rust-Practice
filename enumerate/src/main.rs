enum segment {
    Speech { speaker: String, content: String },
    Narration { content: String }
} //once its end of dialogue, we do not add , after close bracket?

impl Segment {
    pub fn new_speech(speaker: &str, content: &str) -> Segment {
        Segment::Speech {
            speaker: speaker.to_string(),
            content: content.to_string(),
        }
    }
    
    pub fn new_narration(content: &str) -> Segment {
        Segment::Narration { content: content.to_string() }
    }

    pub fn run(&self) {
        println! ("--------------------------------------------");
        match self {
            Segment::Speech { speaker, content } => {
                println! ("[{}]: {}", speaker, content);
            }
            Segment::Narration { content } => {
                println!("{}", content);
            }
        }
        println!("---------------------------------------------");
        let _= std::io::stdin().read_line(&mut String::new());
    }
}
