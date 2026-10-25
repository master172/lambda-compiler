pub struct ProgramASTNode {
    pub statements: Vec<StatementASTNode>,
}

pub struct StatementASTNode {
    pub statement: Statemets,
}

pub enum Statemets {
    Definition(DefinitionASTNode),
    StatementExpression(ExpressionASTNode),
}

pub struct DefinitionASTNode {
    pub identifier: IdentifierASTNode,
    pub expression: ExpressionASTNode,
}

pub struct ExpressionASTNode {
    pub expression: Expressions,
}

pub struct IdentifierASTNode {
    pub name: String,
}

pub enum Expressions {
    Abstraction(AbstractionASTNode),
    Application(ApplicationASTNode),
}

pub struct AbstractionASTNode {
    pub identifier: IdentifierASTNode,
    pub expression: Box<Expressions>,
}

pub struct ApplicationASTNode {
    pub left: Box<AtomicASTNode>,
    pub right: Vec<Box<AtomicASTNode>>,
}

pub struct AtomicASTNode {
    pub atom: Atom,
}
pub enum Atom {
    Identifier(IdentifierASTNode),
    AtomicExpression(ExpressionASTNode),
}

pub enum ParserErrors {
    EOFReachedWithoutParsing,
    InvalidStatmentInitalizer,
    InvalidDefinition,
    InvalidExpression,
    ExpectedAssignmentToken,
}
