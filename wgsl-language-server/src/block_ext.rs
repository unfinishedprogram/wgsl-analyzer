use either::Either::{Left, Right};
use naga::{Block, Span, Statement};

pub trait BlockExt {
    fn flat_span_iter(&self) -> Box<dyn Iterator<Item = (Statement, Span)>>;
}

impl BlockExt for Block {
    fn flat_span_iter(&self) -> Box<dyn Iterator<Item = (Statement, Span)>> {
        Box::new(
            // TODO: Remove expensive and probably unnecessary clone
            self.clone()
                .span_into_iter()
                .flat_map(|(stmt, span)| match stmt {
                    Statement::Block(block) => Left(block.flat_span_iter()),
                    Statement::If {
                        condition: _,
                        accept,
                        reject,
                    } => {
                        let mut accept = accept.clone();
                        let mut reject = reject.clone();
                        accept.append(&mut reject);

                        Left(accept.flat_span_iter())
                    }
                    Statement::Loop {
                        body,
                        continuing,
                        break_if: _,
                    } => {
                        let mut body = body.clone();
                        let mut continuing = continuing.clone();
                        body.append(&mut continuing);

                        Left(body.flat_span_iter())
                    }
                    Statement::Switch { selector: _, cases } => {
                        let mut flat_cases = Block::new();
                        for case in cases {
                            let mut body = case.body.clone();
                            flat_cases.append(&mut body);
                        }

                        Left(flat_cases.flat_span_iter())
                    }
                    _ => Right(std::iter::once((stmt, span))),
                }),
        )
    }
}
