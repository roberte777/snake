use super::Agent;

struct Population {
    individuals: Vec<Agent>,
}
impl Population {
    pub fn new() -> Population {
        Population {
            individuals: Vec::new(),
        }
    }
    pub fn evaluate(&mut self) {
        // for individual in &mut self.individuals {
        //     individual.evaluate();
        // }
        todo!()
    }
    pub fn mutate(&mut self) {
        todo!()
    }
}
