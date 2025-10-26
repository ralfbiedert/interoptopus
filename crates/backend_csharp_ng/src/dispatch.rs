use crate::model::{FunctionId, TypeId};

type DispatchFn = Box<dyn FnMut(Item) -> String>;

pub struct Dispatch {
    dispatch: DispatchFn,
}

impl Dispatch {
    pub fn custom(f: impl FnMut(Item) -> String + 'static) -> Self {
        Self { dispatch: Box::new(f) }
    }

    pub fn single_file() -> Self {
        Self::custom(|x| match x {
            _ => "Interop.cs".to_string(),
        })
    }

    pub fn classify(&mut self, item: Item) -> String {
        (self.dispatch)(item)
    }
}

impl Default for Dispatch {
    fn default() -> Self {
        Self::single_file()
    }
}

pub enum Item {
    Type(TypeId),
    Function(FunctionId),
}
