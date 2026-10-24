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
    name: String,
}

pub enum Expressions {
    Abstraction(AbstractionASTNode),
    Application(ApplicationASTNode),
}

pub struct AbstractionASTNode {
    identifier: IdentifierASTNode,
    expression: Box<Expressions>,
}

pub struct ApplicationASTNode {
    left: Box<Atom>,
    right: Vec<Box<Atom>>,
}

pub enum Atom {
    Identifier(IdentifierASTNode),
    AtomicExpression(ExpressionASTNode),
}

pub enum ParserErrors {
    EOFReachedWithoutParsing,
    InvalidStatmentInitalizer,
    ParseFailed,
}
