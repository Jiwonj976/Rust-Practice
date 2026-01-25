impl Segment {
    pub fn new(speaker: &str, content: &str) -> Segment {
        Segment {
            speaker: speaker.to_string() ,
            content: content.to_string() ,
        }
    }

    pub fn run (&self) {
        println("---------------------------------") ;
        println("[{}]: {}", self.speaker, self.content);
        println("---------------------------------");
        let _ = std::io::stdin().read_line(&mut String::new());
    }
}

