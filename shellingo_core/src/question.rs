use std::{collections::HashSet, hash::Hash};

#[derive(Debug)]
pub struct Question {
   pub question: String,
   pub solutions: HashSet<String>,
   pub location: String,

   pub correct_count_round: i32,
   pub error_count_round: i32,
   pub correct_count_sum: i32,
   pub error_count_sum: i32,    
}

impl PartialEq for Question {
    fn eq(&self, other: &Self) -> bool {
        self.question == other.question 
        && self.location == other.location
    }
}

impl Eq for Question {}

impl Hash for Question {
      fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
          self.question.hash(state);
          self.location.hash(state);
      }
}
