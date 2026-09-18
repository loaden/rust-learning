pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self, value: i32) -> Result<i32, String> {
        if let Some(index) = self.list.iter().position(|x| *x == value) {
            self.list.remove(index);
            self.update_average();
            Ok(value)
        } else {
            Err(format!("value is not in the collection: {}", value))
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        self.average = self.list.iter().sum::<i32>() as f64 / self.list.len() as f64;
    }
}