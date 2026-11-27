use crate::{
    parser::definitions::{AtomicASTNode, PhiAbstractionASTNode},
    pretty_printer::definitions::PrettyPrinter,
    runtime::definitions::{PhiValid, RuntimeErrors},
    visitor::definitons::Visitor,
};

impl PhiValid for PhiAbstractionASTNode {
    fn phi_string(&self) -> Result<String, RuntimeErrors> {
        let mut pretty_printer: PrettyPrinter = PrettyPrinter {
            print_defintion: false,
        };
        let resultant = pretty_printer
            .visit_phi_abstraction(self)
            .map_err(|_| RuntimeErrors::FailedAbstractionPrint);
        resultant
    }
}

impl PhiValid for Box<AtomicASTNode> {
    fn phi_string(&self) -> Result<String, RuntimeErrors> {
        let mut pretty_printer: PrettyPrinter = PrettyPrinter {
            print_defintion: false,
        };
        let resultant = pretty_printer
            .visit_atomic(self)
            .map_err(|_| RuntimeErrors::FailedAbstractionPrint);
        resultant
    }
}
