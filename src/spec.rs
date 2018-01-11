use blocks::BlockPointer;


#[derive(Debug, PartialEq)]
pub enum Spec {
    Pointer(SpecPointer),
    Range((SpecPointer, SpecPointer)),
}

impl Spec {
    #[inline]
    pub fn parse(spec: &str) -> Result<Spec, ()> {
        match spec.find("..") {
            Some(idx) => {
                let (a, b) = spec.split_at(idx);
                let a = SpecPointer::parse(a)?;
                let b = SpecPointer::parse(&b[2..])?;
                Ok(Spec::Range((a, b)))
            },
            None => {
                let pointer = SpecPointer::parse(spec)?;
                Ok(Spec::Pointer(pointer))
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SpecPointer {
    Block(BlockPointer),
    Parent((Box<SpecPointer>, u64)),
    Session(Box<SpecPointer>),
    Head,
}

impl SpecPointer {
    pub fn parse(spec: &str) -> Result<SpecPointer, ()> {
        // TODO: ^ or @ first?

        if spec.ends_with("^") {
            let mut i = 0;
            let len = spec.len();

            for b in spec.as_bytes().iter().rev() {
                if *b != '^' as u8 {
                    break;
                }
                i += 1;
            }

            let next = SpecPointer::parse(&spec[..len-i])?;
            return Ok(SpecPointer::Parent((Box::new(next), i as u64)));
        }

        if spec.starts_with("@") {
            let next = SpecPointer::parse(&spec[1..])?;
            return Ok(SpecPointer::Session(Box::new(next)));
        }


        if spec == "HEAD" || spec == "" {
            return Ok(SpecPointer::Head);
        }

        let block = BlockPointer::from_hex(spec).unwrap();
        Ok(SpecPointer::Block(block))
    }
}
