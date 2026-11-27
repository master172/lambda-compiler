use crate::{
    parser::definitions::{
        AbstractionASTNode, ApplicationASTNode,
        Atom::AtomicExpression,
        AtomicASTNode, ExpressionASTNode,
        Expressions::{self, Application},
        IdentifierASTNode, PhiAbstractionASTNode,
    },
    semantic_analyser::definitions::SemanticTable,
    tree_evaluator::{helpers::NameGenerator, substitute_expression},
};

pub trait AbstractionReducable {
    fn identifier(&self) -> &IdentifierASTNode;
    fn identifier_mut(&mut self) -> &mut IdentifierASTNode;

    fn expression(&self) -> &Expressions;
    fn expression_mut(&mut self) -> &mut Expressions;

    fn communication_stream_id(&self) -> Option<i64> {
        None
    }

    fn beta_reduce(
        &self,
        arguments: &mut Vec<Box<AtomicASTNode>>,
        definitions: &SemanticTable,
        name_generator: &mut NameGenerator,
    ) -> Option<Expressions> {
        let argument = arguments.first()?.clone();

        let remaining_arguments = arguments[1..].to_vec();

        let mut body = self.expression().clone();

        substitute_expression(
            &mut body,
            &argument,
            definitions,
            &self.identifier().name,
            name_generator,
        );

        Some(if remaining_arguments.is_empty() {
            body
        } else {
            Application(ApplicationASTNode {
                left: Box::new(AtomicASTNode {
                    atom: AtomicExpression(ExpressionASTNode { expression: body }),
                }),
                right: remaining_arguments,
            })
        })
    }
}

impl AbstractionReducable for AbstractionASTNode {
    fn identifier(&self) -> &IdentifierASTNode {
        &self.identifier
    }
    fn identifier_mut(&mut self) -> &mut IdentifierASTNode {
        &mut self.identifier
    }

    fn expression(&self) -> &Expressions {
        &self.expression
    }

    fn expression_mut(&mut self) -> &mut Expressions {
        &mut self.expression
    }
}

impl AbstractionReducable for PhiAbstractionASTNode {
    fn identifier(&self) -> &IdentifierASTNode {
        &self.identifier
    }
    fn identifier_mut(&mut self) -> &mut IdentifierASTNode {
        &mut self.identifier
    }

    fn expression(&self) -> &Expressions {
        &self.expression
    }

    fn expression_mut(&mut self) -> &mut Expressions {
        &mut self.expression
    }

    fn communication_stream_id(&self) -> Option<i64> {
        Some(self.communication_stream_id)
    }
}

/*
impl PhiAbstractionASTNode {
    fn to_abstraction(&self) -> AbstractionASTNode {
        AbstractionASTNode {
            identifier: self.identifier.clone(),
            expression: self.expression.clone(),
        }
    }
}
*/
