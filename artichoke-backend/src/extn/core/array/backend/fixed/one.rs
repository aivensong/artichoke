use crate::convert::Convert;
use crate::extn::core::array::{backend, ArrayType};
use crate::extn::core::exception::RubyException;
use crate::gc::MrbGarbageCollection;
use crate::value::Value;
use crate::Artichoke;

#[derive(Debug, Clone)]
pub struct One(Value);

impl One {
    pub fn new(elem: Value) -> Self {
        Self(elem)
    }
}

impl ArrayType for One {
    fn box_clone(&self) -> Box<dyn ArrayType> {
        Box::new(self.clone())
    }

    fn gc_mark(&self, interp: &Artichoke) {
        interp.mark_value(&self.0);
    }

    fn real_children(&self) -> usize {
        1
    }

    fn len(&self) -> usize {
        1
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn get(&self, interp: &Artichoke, index: usize) -> Result<Value, Box<dyn RubyException>> {
        if index == 0 {
            Ok(self.0.clone())
        } else {
            Ok(interp.convert(None::<Value>))
        }
    }

    fn slice(
        &self,
        interp: &Artichoke,
        start: usize,
        len: usize,
    ) -> Result<Box<dyn ArrayType>, Box<dyn RubyException>> {
        let _ = interp;
        if start == 0 && len > 0 {
            Ok(self.box_clone())
        } else {
            Ok(backend::fixed::empty())
        }
    }

    fn set(
        &mut self,
        interp: &Artichoke,
        index: usize,
        elem: Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Box<dyn RubyException>> {
        let _ = interp;
        if index == 0 {
            self.0 = elem;
        } else if index == 1 {
            *realloc = Some(vec![backend::fixed::two(self.0.clone(), elem)]);
        } else {
            *realloc = Some(vec![
                self.box_clone(),
                backend::fixed::hole(index - 1),
                backend::fixed::one(elem),
            ]);
        }
        Ok(())
    }

    fn set_with_drain(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Value,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Box<dyn RubyException>> {
        let _ = interp;
        let drained = if start == 0 && drain == 0 {
            let alloc = vec![backend::fixed::two(with, self.0.clone())];
            *realloc = Some(alloc);
            0
        } else if start == 0 {
            self.0 = with;
            1
        } else if start == 1 {
            let alloc = vec![backend::fixed::two(self.0.clone(), with)];
            *realloc = Some(alloc);
            0
        } else {
            let alloc = vec![
                self.box_clone(),
                backend::fixed::hole(start - 1),
                backend::fixed::one(with),
            ];
            *realloc = Some(alloc);
            0
        };
        Ok(drained)
    }

    fn set_slice(
        &mut self,
        interp: &Artichoke,
        start: usize,
        drain: usize,
        with: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<usize, Box<dyn RubyException>> {
        let _ = interp;
        let (alloc, drained) = if start == 0 && drain == 0 {
            let alloc = vec![with, self.box_clone()];
            (alloc, 0)
        } else if start == 0 {
            let alloc = vec![with];
            (alloc, 1)
        } else if start == 1 {
            let alloc = vec![self.box_clone(), with];
            (alloc, 0)
        } else {
            let alloc = vec![self.box_clone(), backend::fixed::hole(start - 1), with];
            (alloc, 0)
        };
        *realloc = Some(alloc);
        Ok(drained)
    }

    fn concat(
        &mut self,
        interp: &Artichoke,
        other: Box<dyn ArrayType>,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<(), Box<dyn RubyException>> {
        if other.len() == 0 {
            return Ok(());
        } else if other.len() == 1 {
            *realloc = Some(vec![backend::fixed::two(
                self.0.clone(),
                other.get(interp, 0)?,
            )]);
        } else if other.len() < backend::buffer::BUFFER_INLINE_MAX {
            let mut buffer = Vec::with_capacity(1 + other.len());
            buffer.push(self.0.clone());
            for idx in 0..other.len() {
                buffer.push(other.get(interp, idx)?);
            }
            let buffer: Box<dyn ArrayType> = Box::new(backend::buffer::Buffer::from(buffer));
            *realloc = Some(vec![buffer]);
        } else {
            *realloc = Some(vec![self.box_clone(), other]);
        }
        Ok(())
    }

    fn pop(
        &mut self,
        interp: &Artichoke,
        realloc: &mut Option<Vec<Box<dyn ArrayType>>>,
    ) -> Result<Value, Box<dyn RubyException>> {
        let _ = interp;
        *realloc = Some(vec![backend::fixed::empty()]);
        Ok(self.0.clone())
    }

    fn reverse(&self, interp: &Artichoke) -> Result<Box<dyn ArrayType>, Box<dyn RubyException>> {
        let _ = interp;
        Ok(self.box_clone())
    }
}
