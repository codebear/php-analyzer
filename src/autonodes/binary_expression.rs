use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::dynamic_variable_name::DynamicVariableNameNode;
use crate::autonodes::member_access_expression::MemberAccessExpressionNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::nullsafe_member_access_expression::NullsafeMemberAccessExpressionNode;
use crate::autonodes::qualified_name::QualifiedNameNode;
use crate::autonodes::scoped_property_access_expression::ScopedPropertyAccessExpressionNode;
use crate::autonodes::subscript_expression::SubscriptExpressionNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::types::union::DiscreteType;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use std::ffi::OsStr;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum BinaryExpressionOperator {
    NotEqual(&'static str, Range),
    NotIdentical(&'static str, Range),
    Mod(&'static str, Range),
    BinaryAnd(&'static str, Range),
    BooleanAnd(&'static str, Range),
    Mult(&'static str, Range),
    Add(&'static str, Range),
    Sub(&'static str, Range),
    Concat(&'static str, Range),
    Div(&'static str, Range),
    LessThan(&'static str, Range),
    LeftShift(&'static str, Range),
    LessThanOrEqual(&'static str, Range),
    Spaceship(&'static str, Range),
    Equal(&'static str, Range),
    Identical(&'static str, Range),
    GreaterThan(&'static str, Range),
    GreaterThanOrEqual(&'static str, Range),
    RightShift(&'static str, Range),
    BinaryXor(&'static str, Range),
    And(&'static str, Range),
    Instanceof(&'static str, Range),
    Or(&'static str, Range),
    Xor(&'static str, Range),
    BinaryOr(&'static str, Range),
    BooleanOr(&'static str, Range),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl BinaryExpressionOperator {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                BinaryExpressionOperator::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => BinaryExpressionOperator::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => BinaryExpressionOperator::Error(Box::new(ErrorNode::parse(node, source)?)),
            "!=" => BinaryExpressionOperator::NotEqual("!=", node.range()),
            "!==" => BinaryExpressionOperator::NotIdentical("!==", node.range()),
            "%" => BinaryExpressionOperator::Mod("%", node.range()),
            "&" => BinaryExpressionOperator::BinaryAnd("&", node.range()),
            "&&" => BinaryExpressionOperator::BooleanAnd("&&", node.range()),
            "*" => BinaryExpressionOperator::Mult("*", node.range()),
            "+" => BinaryExpressionOperator::Add("+", node.range()),
            "-" => BinaryExpressionOperator::Sub("-", node.range()),
            "." => BinaryExpressionOperator::Concat(".", node.range()),
            "/" => BinaryExpressionOperator::Div("/", node.range()),
            "<" => BinaryExpressionOperator::LessThan("<", node.range()),
            "<<" => BinaryExpressionOperator::LeftShift("<<", node.range()),
            "<=" => BinaryExpressionOperator::LessThanOrEqual("<=", node.range()),
            "<=>" => BinaryExpressionOperator::Spaceship("<=>", node.range()),
            "<>" => BinaryExpressionOperator::NotEqual("<>", node.range()),
            "==" => BinaryExpressionOperator::Equal("==", node.range()),
            "===" => BinaryExpressionOperator::Identical("===", node.range()),
            ">" => BinaryExpressionOperator::GreaterThan(">", node.range()),
            ">=" => BinaryExpressionOperator::GreaterThanOrEqual(">=", node.range()),
            ">>" => BinaryExpressionOperator::RightShift(">>", node.range()),
            "^" => BinaryExpressionOperator::BinaryXor("^", node.range()),
            "and" => BinaryExpressionOperator::And("and", node.range()),
            "instanceof" => BinaryExpressionOperator::Instanceof("instanceof", node.range()),
            "or" => BinaryExpressionOperator::Or("or", node.range()),
            "xor" => BinaryExpressionOperator::Xor("xor", node.range()),
            "|" => BinaryExpressionOperator::BinaryOr("|", node.range()),
            "||" => BinaryExpressionOperator::BooleanOr("||", node.range()),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => {
                BinaryExpressionOperator::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => BinaryExpressionOperator::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => BinaryExpressionOperator::Error(Box::new(ErrorNode::parse(node, source)?)),
            "!=" => BinaryExpressionOperator::NotEqual("!=", node.range()),
            "!==" => BinaryExpressionOperator::NotIdentical("!==", node.range()),
            "%" => BinaryExpressionOperator::Mod("%", node.range()),
            "&" => BinaryExpressionOperator::BinaryAnd("&", node.range()),
            "&&" => BinaryExpressionOperator::BooleanAnd("&&", node.range()),
            "*" => BinaryExpressionOperator::Mult("*", node.range()),
            "+" => BinaryExpressionOperator::Add("+", node.range()),
            "-" => BinaryExpressionOperator::Sub("-", node.range()),
            "." => BinaryExpressionOperator::Concat(".", node.range()),
            "/" => BinaryExpressionOperator::Div("/", node.range()),
            "<" => BinaryExpressionOperator::LessThan("<", node.range()),
            "<<" => BinaryExpressionOperator::LeftShift("<<", node.range()),
            "<=" => BinaryExpressionOperator::LessThanOrEqual("<=", node.range()),
            "<=>" => BinaryExpressionOperator::Spaceship("<=>", node.range()),
            "<>" => BinaryExpressionOperator::NotEqual("<>", node.range()),
            "==" => BinaryExpressionOperator::Equal("==", node.range()),
            "===" => BinaryExpressionOperator::Identical("===", node.range()),
            ">" => BinaryExpressionOperator::GreaterThan(">", node.range()),
            ">=" => BinaryExpressionOperator::GreaterThanOrEqual(">=", node.range()),
            ">>" => BinaryExpressionOperator::RightShift(">>", node.range()),
            "^" => BinaryExpressionOperator::BinaryXor("^", node.range()),
            "and" => BinaryExpressionOperator::And("and", node.range()),
            "instanceof" => BinaryExpressionOperator::Instanceof("instanceof", node.range()),
            "or" => BinaryExpressionOperator::Or("or", node.range()),
            "xor" => BinaryExpressionOperator::Xor("xor", node.range()),
            "|" => BinaryExpressionOperator::BinaryOr("|", node.range()),
            "||" => BinaryExpressionOperator::BooleanOr("||", node.range()),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        self.as_any().kind()
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match self {
            BinaryExpressionOperator::Comment(x) => x.get_utype(state, emitter),
            BinaryExpressionOperator::TextInterpolation(x) => x.get_utype(state, emitter),
            BinaryExpressionOperator::Error(x) => x.get_utype(state, emitter),
            BinaryExpressionOperator::NotEqual(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::NotIdentical(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::Mod(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::BinaryAnd(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::BooleanAnd(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::Mult(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::Add(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::Sub(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::Concat(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::Div(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::LessThan(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::LeftShift(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::LessThanOrEqual(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::Spaceship(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::Equal(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::Identical(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::GreaterThan(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::GreaterThanOrEqual(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::RightShift(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::BinaryXor(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::And(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::Instanceof(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::Or(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::Xor(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::BinaryOr(_, _) => Some(DiscreteType::String.into()),
            BinaryExpressionOperator::BooleanOr(_, _) => Some(DiscreteType::String.into()),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            BinaryExpressionOperator::Comment(x) => x.get_php_value(state, emitter),
            BinaryExpressionOperator::TextInterpolation(x) => x.get_php_value(state, emitter),
            BinaryExpressionOperator::Error(x) => x.get_php_value(state, emitter),
            BinaryExpressionOperator::NotEqual(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::NotIdentical(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::Mod(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::BinaryAnd(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::BooleanAnd(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::Mult(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::Add(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::Sub(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::Concat(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::Div(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::LessThan(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::LeftShift(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::LessThanOrEqual(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::Spaceship(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::Equal(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::Identical(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::GreaterThan(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::GreaterThanOrEqual(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::RightShift(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::BinaryXor(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::And(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::Instanceof(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::Or(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::Xor(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::BinaryOr(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            BinaryExpressionOperator::BooleanOr(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            BinaryExpressionOperator::Comment(x) => x.read_from(state, emitter),
            BinaryExpressionOperator::TextInterpolation(x) => x.read_from(state, emitter),
            BinaryExpressionOperator::Error(x) => x.read_from(state, emitter),
            BinaryExpressionOperator::NotEqual(_, _) => (),
            BinaryExpressionOperator::NotIdentical(_, _) => (),
            BinaryExpressionOperator::Mod(_, _) => (),
            BinaryExpressionOperator::BinaryAnd(_, _) => (),
            BinaryExpressionOperator::BooleanAnd(_, _) => (),
            BinaryExpressionOperator::Mult(_, _) => (),
            BinaryExpressionOperator::Add(_, _) => (),
            BinaryExpressionOperator::Sub(_, _) => (),
            BinaryExpressionOperator::Concat(_, _) => (),
            BinaryExpressionOperator::Div(_, _) => (),
            BinaryExpressionOperator::LessThan(_, _) => (),
            BinaryExpressionOperator::LeftShift(_, _) => (),
            BinaryExpressionOperator::LessThanOrEqual(_, _) => (),
            BinaryExpressionOperator::Spaceship(_, _) => (),
            BinaryExpressionOperator::Equal(_, _) => (),
            BinaryExpressionOperator::Identical(_, _) => (),
            BinaryExpressionOperator::GreaterThan(_, _) => (),
            BinaryExpressionOperator::GreaterThanOrEqual(_, _) => (),
            BinaryExpressionOperator::RightShift(_, _) => (),
            BinaryExpressionOperator::BinaryXor(_, _) => (),
            BinaryExpressionOperator::And(_, _) => (),
            BinaryExpressionOperator::Instanceof(_, _) => (),
            BinaryExpressionOperator::Or(_, _) => (),
            BinaryExpressionOperator::Xor(_, _) => (),
            BinaryExpressionOperator::BinaryOr(_, _) => (),
            BinaryExpressionOperator::BooleanOr(_, _) => (),
        }
    }
}

impl NodeAccess for BinaryExpressionOperator {
    fn brief_desc(&self) -> String {
        match self {
            BinaryExpressionOperator::Comment(x) => {
                format!("BinaryExpressionOperator::comment({})", x.brief_desc())
            }
            BinaryExpressionOperator::TextInterpolation(x) => format!(
                "BinaryExpressionOperator::text_interpolation({})",
                x.brief_desc()
            ),
            BinaryExpressionOperator::Error(x) => {
                format!("BinaryExpressionOperator::ERROR({})", x.brief_desc())
            }
            BinaryExpressionOperator::NotEqual(a, _) => a.to_string(),
            BinaryExpressionOperator::NotIdentical(a, _) => a.to_string(),
            BinaryExpressionOperator::Mod(a, _) => a.to_string(),
            BinaryExpressionOperator::BinaryAnd(a, _) => a.to_string(),
            BinaryExpressionOperator::BooleanAnd(a, _) => a.to_string(),
            BinaryExpressionOperator::Mult(a, _) => a.to_string(),
            BinaryExpressionOperator::Add(a, _) => a.to_string(),
            BinaryExpressionOperator::Sub(a, _) => a.to_string(),
            BinaryExpressionOperator::Concat(a, _) => a.to_string(),
            BinaryExpressionOperator::Div(a, _) => a.to_string(),
            BinaryExpressionOperator::LessThan(a, _) => a.to_string(),
            BinaryExpressionOperator::LeftShift(a, _) => a.to_string(),
            BinaryExpressionOperator::LessThanOrEqual(a, _) => a.to_string(),
            BinaryExpressionOperator::Spaceship(a, _) => a.to_string(),
            BinaryExpressionOperator::Equal(a, _) => a.to_string(),
            BinaryExpressionOperator::Identical(a, _) => a.to_string(),
            BinaryExpressionOperator::GreaterThan(a, _) => a.to_string(),
            BinaryExpressionOperator::GreaterThanOrEqual(a, _) => a.to_string(),
            BinaryExpressionOperator::RightShift(a, _) => a.to_string(),
            BinaryExpressionOperator::BinaryXor(a, _) => a.to_string(),
            BinaryExpressionOperator::And(a, _) => a.to_string(),
            BinaryExpressionOperator::Instanceof(a, _) => a.to_string(),
            BinaryExpressionOperator::Or(a, _) => a.to_string(),
            BinaryExpressionOperator::Xor(a, _) => a.to_string(),
            BinaryExpressionOperator::BinaryOr(a, _) => a.to_string(),
            BinaryExpressionOperator::BooleanOr(a, _) => a.to_string(),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            BinaryExpressionOperator::Comment(x) => x.as_any(),
            BinaryExpressionOperator::TextInterpolation(x) => x.as_any(),
            BinaryExpressionOperator::Error(x) => x.as_any(),
            BinaryExpressionOperator::NotEqual(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::NotIdentical(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::Mod(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::BinaryAnd(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::BooleanAnd(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::Mult(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::Add(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::Sub(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::Concat(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::Div(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::LessThan(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::LeftShift(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::LessThanOrEqual(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::Spaceship(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::Equal(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::Identical(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::GreaterThan(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::GreaterThanOrEqual(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::RightShift(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::BinaryXor(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::And(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::Instanceof(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::Or(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::Xor(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::BinaryOr(a, b) => AnyNodeRef::StaticExpr(a, *b),
            BinaryExpressionOperator::BooleanOr(a, b) => AnyNodeRef::StaticExpr(a, *b),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            BinaryExpressionOperator::Comment(x) => x.children_any(),
            BinaryExpressionOperator::TextInterpolation(x) => x.children_any(),
            BinaryExpressionOperator::Error(x) => x.children_any(),
            BinaryExpressionOperator::NotEqual(_, _) => todo!("Crap"),
            BinaryExpressionOperator::NotIdentical(_, _) => todo!("Crap"),
            BinaryExpressionOperator::Mod(_, _) => todo!("Crap"),
            BinaryExpressionOperator::BinaryAnd(_, _) => todo!("Crap"),
            BinaryExpressionOperator::BooleanAnd(_, _) => todo!("Crap"),
            BinaryExpressionOperator::Mult(_, _) => todo!("Crap"),
            BinaryExpressionOperator::Add(_, _) => todo!("Crap"),
            BinaryExpressionOperator::Sub(_, _) => todo!("Crap"),
            BinaryExpressionOperator::Concat(_, _) => todo!("Crap"),
            BinaryExpressionOperator::Div(_, _) => todo!("Crap"),
            BinaryExpressionOperator::LessThan(_, _) => todo!("Crap"),
            BinaryExpressionOperator::LeftShift(_, _) => todo!("Crap"),
            BinaryExpressionOperator::LessThanOrEqual(_, _) => todo!("Crap"),
            BinaryExpressionOperator::Spaceship(_, _) => todo!("Crap"),
            BinaryExpressionOperator::Equal(_, _) => todo!("Crap"),
            BinaryExpressionOperator::Identical(_, _) => todo!("Crap"),
            BinaryExpressionOperator::GreaterThan(_, _) => todo!("Crap"),
            BinaryExpressionOperator::GreaterThanOrEqual(_, _) => todo!("Crap"),
            BinaryExpressionOperator::RightShift(_, _) => todo!("Crap"),
            BinaryExpressionOperator::BinaryXor(_, _) => todo!("Crap"),
            BinaryExpressionOperator::And(_, _) => todo!("Crap"),
            BinaryExpressionOperator::Instanceof(_, _) => todo!("Crap"),
            BinaryExpressionOperator::Or(_, _) => todo!("Crap"),
            BinaryExpressionOperator::Xor(_, _) => todo!("Crap"),
            BinaryExpressionOperator::BinaryOr(_, _) => todo!("Crap"),
            BinaryExpressionOperator::BooleanOr(_, _) => todo!("Crap"),
        }
    }

    fn range(&self) -> Range {
        match self {
            BinaryExpressionOperator::Comment(x) => x.range(),
            BinaryExpressionOperator::TextInterpolation(x) => x.range(),
            BinaryExpressionOperator::Error(x) => x.range(),
            BinaryExpressionOperator::NotEqual(_, r) => *r,
            BinaryExpressionOperator::NotIdentical(_, r) => *r,
            BinaryExpressionOperator::Mod(_, r) => *r,
            BinaryExpressionOperator::BinaryAnd(_, r) => *r,
            BinaryExpressionOperator::BooleanAnd(_, r) => *r,
            BinaryExpressionOperator::Mult(_, r) => *r,
            BinaryExpressionOperator::Add(_, r) => *r,
            BinaryExpressionOperator::Sub(_, r) => *r,
            BinaryExpressionOperator::Concat(_, r) => *r,
            BinaryExpressionOperator::Div(_, r) => *r,
            BinaryExpressionOperator::LessThan(_, r) => *r,
            BinaryExpressionOperator::LeftShift(_, r) => *r,
            BinaryExpressionOperator::LessThanOrEqual(_, r) => *r,
            BinaryExpressionOperator::Spaceship(_, r) => *r,
            BinaryExpressionOperator::Equal(_, r) => *r,
            BinaryExpressionOperator::Identical(_, r) => *r,
            BinaryExpressionOperator::GreaterThan(_, r) => *r,
            BinaryExpressionOperator::GreaterThanOrEqual(_, r) => *r,
            BinaryExpressionOperator::RightShift(_, r) => *r,
            BinaryExpressionOperator::BinaryXor(_, r) => *r,
            BinaryExpressionOperator::And(_, r) => *r,
            BinaryExpressionOperator::Instanceof(_, r) => *r,
            BinaryExpressionOperator::Or(_, r) => *r,
            BinaryExpressionOperator::Xor(_, r) => *r,
            BinaryExpressionOperator::BinaryOr(_, r) => *r,
            BinaryExpressionOperator::BooleanOr(_, r) => *r,
        }
    }
}
#[derive(Debug, Clone)]
pub enum BinaryExpressionRight {
    _Expression(Box<_ExpressionNode>),
    DynamicVariableName(Box<DynamicVariableNameNode>),
    MemberAccessExpression(Box<MemberAccessExpressionNode>),
    Name(Box<NameNode>),
    NullsafeMemberAccessExpression(Box<NullsafeMemberAccessExpressionNode>),
    QualifiedName(Box<QualifiedNameNode>),
    ScopedPropertyAccessExpression(Box<ScopedPropertyAccessExpressionNode>),
    SubscriptExpression(Box<SubscriptExpressionNode>),
    VariableName(Box<VariableNameNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl BinaryExpressionRight {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                BinaryExpressionRight::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => BinaryExpressionRight::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => BinaryExpressionRight::Error(Box::new(ErrorNode::parse(node, source)?)),
            "dynamic_variable_name" => BinaryExpressionRight::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "member_access_expression" => BinaryExpressionRight::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "name" => BinaryExpressionRight::Name(Box::new(NameNode::parse(node, source)?)),
            "nullsafe_member_access_expression" => {
                BinaryExpressionRight::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => BinaryExpressionRight::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                BinaryExpressionRight::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => BinaryExpressionRight::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => BinaryExpressionRight::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| BinaryExpressionRight::_Expression(y))
                {
                    x
                } else {
                    return Err(ParseError::new(
                        node.range(),
                        format!("Parse error, unexpected node-type: {}", node.kind()),
                    ));
                }
            }
        })
    }

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => {
                BinaryExpressionRight::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => BinaryExpressionRight::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => BinaryExpressionRight::Error(Box::new(ErrorNode::parse(node, source)?)),
            "dynamic_variable_name" => BinaryExpressionRight::DynamicVariableName(Box::new(
                DynamicVariableNameNode::parse(node, source)?,
            )),
            "member_access_expression" => BinaryExpressionRight::MemberAccessExpression(Box::new(
                MemberAccessExpressionNode::parse(node, source)?,
            )),
            "name" => BinaryExpressionRight::Name(Box::new(NameNode::parse(node, source)?)),
            "nullsafe_member_access_expression" => {
                BinaryExpressionRight::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => BinaryExpressionRight::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                BinaryExpressionRight::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => BinaryExpressionRight::SubscriptExpression(Box::new(
                SubscriptExpressionNode::parse(node, source)?,
            )),
            "variable_name" => BinaryExpressionRight::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

            _ => {
                return Ok(
                    if let Some(x) = _ExpressionNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| BinaryExpressionRight::_Expression(y))
                    {
                        Some(x)
                    } else {
                        None
                    },
                )
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        self.as_any().kind()
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn get_utype(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<UnionType> {
        match self {
            BinaryExpressionRight::Comment(x) => x.get_utype(state, emitter),
            BinaryExpressionRight::TextInterpolation(x) => x.get_utype(state, emitter),
            BinaryExpressionRight::Error(x) => x.get_utype(state, emitter),
            BinaryExpressionRight::_Expression(x) => x.get_utype(state, emitter),
            BinaryExpressionRight::DynamicVariableName(x) => x.get_utype(state, emitter),
            BinaryExpressionRight::MemberAccessExpression(x) => x.get_utype(state, emitter),
            BinaryExpressionRight::Name(x) => x.get_utype(state, emitter),
            BinaryExpressionRight::NullsafeMemberAccessExpression(x) => x.get_utype(state, emitter),
            BinaryExpressionRight::QualifiedName(x) => x.get_utype(state, emitter),
            BinaryExpressionRight::ScopedPropertyAccessExpression(x) => x.get_utype(state, emitter),
            BinaryExpressionRight::SubscriptExpression(x) => x.get_utype(state, emitter),
            BinaryExpressionRight::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            BinaryExpressionRight::Comment(x) => x.get_php_value(state, emitter),
            BinaryExpressionRight::TextInterpolation(x) => x.get_php_value(state, emitter),
            BinaryExpressionRight::Error(x) => x.get_php_value(state, emitter),
            BinaryExpressionRight::_Expression(x) => x.get_php_value(state, emitter),
            BinaryExpressionRight::DynamicVariableName(x) => x.get_php_value(state, emitter),
            BinaryExpressionRight::MemberAccessExpression(x) => x.get_php_value(state, emitter),
            BinaryExpressionRight::Name(x) => x.get_php_value(state, emitter),
            BinaryExpressionRight::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            BinaryExpressionRight::QualifiedName(x) => x.get_php_value(state, emitter),
            BinaryExpressionRight::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            BinaryExpressionRight::SubscriptExpression(x) => x.get_php_value(state, emitter),
            BinaryExpressionRight::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            BinaryExpressionRight::Comment(x) => x.read_from(state, emitter),
            BinaryExpressionRight::TextInterpolation(x) => x.read_from(state, emitter),
            BinaryExpressionRight::Error(x) => x.read_from(state, emitter),
            BinaryExpressionRight::_Expression(x) => x.read_from(state, emitter),
            BinaryExpressionRight::DynamicVariableName(x) => x.read_from(state, emitter),
            BinaryExpressionRight::MemberAccessExpression(x) => x.read_from(state, emitter),
            BinaryExpressionRight::Name(x) => x.read_from(state, emitter),
            BinaryExpressionRight::NullsafeMemberAccessExpression(x) => x.read_from(state, emitter),
            BinaryExpressionRight::QualifiedName(x) => x.read_from(state, emitter),
            BinaryExpressionRight::ScopedPropertyAccessExpression(x) => x.read_from(state, emitter),
            BinaryExpressionRight::SubscriptExpression(x) => x.read_from(state, emitter),
            BinaryExpressionRight::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for BinaryExpressionRight {
    fn brief_desc(&self) -> String {
        match self {
            BinaryExpressionRight::Comment(x) => {
                format!("BinaryExpressionRight::comment({})", x.brief_desc())
            }
            BinaryExpressionRight::TextInterpolation(x) => format!(
                "BinaryExpressionRight::text_interpolation({})",
                x.brief_desc()
            ),
            BinaryExpressionRight::Error(x) => {
                format!("BinaryExpressionRight::ERROR({})", x.brief_desc())
            }
            BinaryExpressionRight::_Expression(x) => {
                format!("BinaryExpressionRight::_expression({})", x.brief_desc())
            }
            BinaryExpressionRight::DynamicVariableName(x) => format!(
                "BinaryExpressionRight::dynamic_variable_name({})",
                x.brief_desc()
            ),
            BinaryExpressionRight::MemberAccessExpression(x) => format!(
                "BinaryExpressionRight::member_access_expression({})",
                x.brief_desc()
            ),
            BinaryExpressionRight::Name(x) => {
                format!("BinaryExpressionRight::name({})", x.brief_desc())
            }
            BinaryExpressionRight::NullsafeMemberAccessExpression(x) => format!(
                "BinaryExpressionRight::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            BinaryExpressionRight::QualifiedName(x) => {
                format!("BinaryExpressionRight::qualified_name({})", x.brief_desc())
            }
            BinaryExpressionRight::ScopedPropertyAccessExpression(x) => format!(
                "BinaryExpressionRight::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            BinaryExpressionRight::SubscriptExpression(x) => format!(
                "BinaryExpressionRight::subscript_expression({})",
                x.brief_desc()
            ),
            BinaryExpressionRight::VariableName(x) => {
                format!("BinaryExpressionRight::variable_name({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            BinaryExpressionRight::Comment(x) => x.as_any(),
            BinaryExpressionRight::TextInterpolation(x) => x.as_any(),
            BinaryExpressionRight::Error(x) => x.as_any(),
            BinaryExpressionRight::_Expression(x) => x.as_any(),
            BinaryExpressionRight::DynamicVariableName(x) => x.as_any(),
            BinaryExpressionRight::MemberAccessExpression(x) => x.as_any(),
            BinaryExpressionRight::Name(x) => x.as_any(),
            BinaryExpressionRight::NullsafeMemberAccessExpression(x) => x.as_any(),
            BinaryExpressionRight::QualifiedName(x) => x.as_any(),
            BinaryExpressionRight::ScopedPropertyAccessExpression(x) => x.as_any(),
            BinaryExpressionRight::SubscriptExpression(x) => x.as_any(),
            BinaryExpressionRight::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            BinaryExpressionRight::Comment(x) => x.children_any(),
            BinaryExpressionRight::TextInterpolation(x) => x.children_any(),
            BinaryExpressionRight::Error(x) => x.children_any(),
            BinaryExpressionRight::_Expression(x) => x.children_any(),
            BinaryExpressionRight::DynamicVariableName(x) => x.children_any(),
            BinaryExpressionRight::MemberAccessExpression(x) => x.children_any(),
            BinaryExpressionRight::Name(x) => x.children_any(),
            BinaryExpressionRight::NullsafeMemberAccessExpression(x) => x.children_any(),
            BinaryExpressionRight::QualifiedName(x) => x.children_any(),
            BinaryExpressionRight::ScopedPropertyAccessExpression(x) => x.children_any(),
            BinaryExpressionRight::SubscriptExpression(x) => x.children_any(),
            BinaryExpressionRight::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            BinaryExpressionRight::Comment(x) => x.range(),
            BinaryExpressionRight::TextInterpolation(x) => x.range(),
            BinaryExpressionRight::Error(x) => x.range(),
            BinaryExpressionRight::_Expression(x) => x.range(),
            BinaryExpressionRight::DynamicVariableName(x) => x.range(),
            BinaryExpressionRight::MemberAccessExpression(x) => x.range(),
            BinaryExpressionRight::Name(x) => x.range(),
            BinaryExpressionRight::NullsafeMemberAccessExpression(x) => x.range(),
            BinaryExpressionRight::QualifiedName(x) => x.range(),
            BinaryExpressionRight::ScopedPropertyAccessExpression(x) => x.range(),
            BinaryExpressionRight::SubscriptExpression(x) => x.range(),
            BinaryExpressionRight::VariableName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct BinaryExpressionNode {
    pub range: Range,
    pub left: Option<_ExpressionNode>,
    pub operator: Option<Box<BinaryExpressionOperator>>,
    pub right: Option<Box<BinaryExpressionRight>>,
    pub children: Vec<Box<_ExpressionNode>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl BinaryExpressionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "binary_expression" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [binary_expression] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let left: Option<_ExpressionNode> = node
            .children_by_field_name("left", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| _ExpressionNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        let operator: Option<Box<BinaryExpressionOperator>> = node
            .children_by_field_name("operator", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode2| BinaryExpressionOperator::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
        let right: Option<Box<BinaryExpressionRight>> = node
            .children_by_field_name("right", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode2| BinaryExpressionRight::parse(chnode2, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .map(|z| Box::new(z))
            .next()
            .into();
        Ok(Self {
            range,
            left,
            operator,
            right,
            children: _ExpressionNode::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| !skip_nodes.contains(&node.id()))
                    .filter(|node| node.kind() != "comment"),
                source,
            )?,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment")
                    .filter(|node| !skip_nodes.contains(&node.id())),
                source,
            )?,
        })
    }

    pub fn parse_vec<'a, I>(children: I, source: &Vec<u8>) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            if child.kind() == "comment" {
                continue;
            }
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
    }

    pub fn kind(&self) -> &'static str {
        "binary_expression"
    }
}

impl NodeAccess for BinaryExpressionNode {
    fn brief_desc(&self) -> String {
        "BinaryExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::BinaryExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.left {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.operator {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.right {
            child_vec.push(x.as_any());
        }
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
