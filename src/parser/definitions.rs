#[derive(Debug)]
pub struct ProgramASTNode {
    pub statements: Vec<StatementASTNode>,
}

#[derive(Debug)]
pub struct StatementASTNode {
    pub statement: Statemets,
}

#[derive(Debug)]
pub enum Statemets {
    Definition(DefinitionASTNode),
    StatementExpression(ExpressionASTNode),
}

#[derive(Debug)]
pub struct DefinitionASTNode {
    pub identifier: IdentifierASTNode,
    pub expression: ExpressionASTNode,
}

#[derive(Debug)]
pub struct ExpressionASTNode {
    pub expression: Expressions,
}

#[derive(Debug)]
pub struct IdentifierASTNode {
    pub name: String,
}

#[derive(Debug)]
pub enum Expressions {
    Abstraction(AbstractionASTNode),
    Application(ApplicationASTNode),
}

#[derive(Debug)]
pub struct AbstractionASTNode {
    pub identifier: IdentifierASTNode,
    pub expression: Box<Expressions>,
}

#[derive(Debug)]
pub struct ApplicationASTNode {
    pub left: Box<AtomicASTNode>,
    pub right: Vec<Box<AtomicASTNode>>,
}

#[derive(Debug)]
pub struct AtomicASTNode {
    pub atom: Atom,
}

#[derive(Debug)]
pub enum Atom {
    Identifier(IdentifierASTNode),
    AtomicExpression(ExpressionASTNode),
}

#[derive(Debug)]
pub enum ParserErrors {
    EOFReachedWithoutParsing,
    InvalidStatmentInitalizer,
    InvalidDefinition,
    InvalidExpression,
    InvalidAbstraction,
    InvalidApplication,
    InvalidAtomic,
    ExpectedDotToken,
    ExpectedSemiColon,
    ExpectedRightParenthesisToken,
    ExpectedIdentiferToken,
    ExpectedAssignmentToken,
}
