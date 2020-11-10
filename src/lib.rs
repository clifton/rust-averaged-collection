use num_traits::ToPrimitive;

/// A collection that retains an average value of its elements
pub struct AveragedCollection<T: ToPrimitive + Copy> {
    values: Vec<T>,
    average: f64,
}

impl<T> AveragedCollection<T>
where
    T: ToPrimitive + Copy,
{
    /// adds a new element and updates the average
    ///
    /// # Example
    ///
    /// ```
    /// use averaged_collection::AveragedCollection;
    /// let mut avg_collection = AveragedCollection::new();
    /// avg_collection.add(1);
    /// avg_collection.add(2);
    /// assert_eq!(avg_collection.average(), 1.5);
    /// ```
    ///
    /// ```
    /// use averaged_collection::AveragedCollection;
    /// use assert_approx_eq::assert_approx_eq;
    /// let mut avg_collection = AveragedCollection::new();
    /// avg_collection.add(1.2);
    /// avg_collection.add(1.4);
    /// avg_collection.add(1.6);
    /// assert_approx_eq!(avg_collection.average(), 1.4);
    /// ```
    pub fn new() -> AveragedCollection<T> {
        AveragedCollection {
            values: Vec::new(),
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: T) {
        self.values.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<T> {
        match self.values.pop() {
            Some(value) => Some(value),
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let mut total: f64 = 0.0;
        for value in &self.values {
            total += ToPrimitive::to_f64(value).unwrap();
        }
        self.average = total as f64 / self.values.len() as f64;
    }
}
