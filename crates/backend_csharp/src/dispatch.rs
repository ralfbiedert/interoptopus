use crate::lang::{FunctionId, TypeId};

type DispatchFn = Box<dyn FnMut(Item) -> String>;

pub struct Dispatch {
    dispatch: DispatchFn,
}

impl Dispatch {
    pub fn custom(f: impl FnMut(Item) -> String + 'static) -> Self {
        Self { dispatch: Box::new(f) }
    }

    #[must_use] 
    pub fn single_file() -> Self {
        Self::custom(|x| "Interop.cs".to_string())
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
