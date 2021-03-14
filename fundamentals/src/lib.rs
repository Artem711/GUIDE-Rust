pub struct User {
    pub name: String,
    pub surname: String,
    pub job_location: String
}

use std::fmt;

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,  "Name: {}, Surname: {}, Job Location: {}", self.name, self.surname, self.job_location)
    }
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.name, self.surname, self.job_location)
    }
}