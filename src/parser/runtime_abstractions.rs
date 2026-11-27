use crate::parser::definitions::{
    AbstractionASTNode, Expressions, IdentifierASTNode, PhiAbstractionASTNode,
};

pub trait AbstractionReducable {
    fn identifier(&self) -> &IdentifierASTNode;
    fn identifier_mut(&mut self) -> &mut IdentifierASTNode;

    fn expression(&self) -> &Expressions;
    fn expression_mut(&mut self) -> &mut Expressions;
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
