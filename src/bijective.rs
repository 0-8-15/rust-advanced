use slint::{Model, ModelTracker};

pub struct BijectiveModel<M, F, R> {
    wrapped_model: M,
    forward_function: F,
    reverse_function: R,
}

impl<M, F, R, T, U> Model for BijectiveModel<M, F, R>
where
    M: 'static,
    F: 'static,
    R: 'static,
    F: Fn(T) -> U,
    R: Fn(U, T) -> T,
    M: Model<Data = T>,
{
    type Data = U;

    fn row_count(&self) -> usize {
        self.wrapped_model.row_count()
    }

    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.wrapped_model.row_data(row).map(|x| (self.forward_function)(x))
    }

    fn set_row_data(&self, row: usize, data: Self::Data) {
        if let Some(new) = self.wrapped_model.row_data(row).map(|old| (self.reverse_function)(data, old)) {
            self.wrapped_model.set_row_data(row, new)
        }
    }

    fn model_tracker(&self) -> &dyn ModelTracker {
        self.wrapped_model.model_tracker()
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl<M, F, R, T, U> BijectiveModel<M, F, R>
where
    M: 'static,
    F: 'static,
    R: 'static,
    F: Fn(T) -> U,
    R: Fn(U, T) -> T,
    M: Model<Data = T>,
{
    /// Creates a new BijectiveModel based on the given `wrapped_model` and `map_function`.
    /// Alternatively you can use [`ModelExt::map`] on your Model.
    pub fn new(wrapped_model: M, forward_function: F, reverse_function: R) -> Self {
        Self { wrapped_model, forward_function, reverse_function }
    }

    /// Returns a reference to the inner model
    pub fn source_model(&self) -> &M {
        &self.wrapped_model
    }
}

#[test]
fn test_bijective_model() {
    use slint::VecModel;
    let wrapped_rc = Rc::new(VecModel::from(vec![1, 2, 3]));
    let map = BijectiveModel::new(wrapped_rc.clone(), |x| x.to_string(), |s, o| s.parse().unwrap_or_else(|_| o));

    wrapped_rc.set_row_data(2, 42);
    wrapped_rc.push(4);

    assert_eq!(map.row_data(2).unwrap(), "42");
    assert_eq!(map.row_data(3).unwrap(), "4");
    assert_eq!(map.row_data(1).unwrap(), "2");

    map.set_row_data(3, "23".to_string());
    assert_eq!(wrapped_rc.row_data(3).unwrap(), 23);

}
