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
use crate::autonodes::variable_name::VariableNameNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::operators::add::AddOperator;
use crate::operators::binary_and::BinaryAndOperator;
use crate::operators::binary_or::BinaryOrOperator;
use crate::operators::binary_xor::BinaryXorOperator;
use crate::operators::boolean_and::BooleanAndOperator;
use crate::operators::boolean_or::BooleanOrOperator;
use crate::operators::concat::ConcatOperator;
use crate::operators::div::DivOperator;
use crate::operators::equal::EqualOperator;
use crate::operators::exponential::ExponentialOperator;
use crate::operators::greater_than::GreaterThanOperator;
use crate::operators::greater_than_or_equal::GreaterThanOrEqualOperator;
use crate::operators::identical::IdenticalOperator;
use crate::operators::instanceof::InstanceofOperator;
use crate::operators::left_shift::LeftShiftOperator;
use crate::operators::less_than::LessThanOperator;
use crate::operators::less_than_or_equal::LessThanOrEqualOperator;
use crate::operators::logical_and::LogicalAndOperator;
use crate::operators::logical_or::LogicalOrOperator;
use crate::operators::logical_xor::LogicalXorOperator;
use crate::operators::modulus::ModOperator;
use crate::operators::mult::MultOperator;
use crate::operators::not_equal::NotEqualOperator;
use crate::operators::not_identical::NotIdenticalOperator;
use crate::operators::null_coalesce::NullCoalesceOperator;
use crate::operators::operator::Operator;
use crate::operators::right_shift::RightShiftOperator;
use crate::operators::spaceship::SpaceshipOperator;
use crate::operators::sub::SubOperator;
use crate::parser::Range;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum BinaryExpressionOperator {
    NotEqual(NotEqualOperator),
    NotIdentical(NotIdenticalOperator),
    Mod(ModOperator),
    BinaryAnd(BinaryAndOperator),
    BooleanAnd(BooleanAndOperator),
    Mult(MultOperator),
    Exponential(ExponentialOperator),
    Add(AddOperator),
    Sub(SubOperator),
    Concat(ConcatOperator),
    Div(DivOperator),
    LessThan(LessThanOperator),
    LeftShift(LeftShiftOperator),
    LessThanOrEqual(LessThanOrEqualOperator),
    Spaceship(SpaceshipOperator),
    Equal(EqualOperator),
    Identical(IdenticalOperator),
    GreaterThan(GreaterThanOperator),
    GreaterThanOrEqual(GreaterThanOrEqualOperator),
    RightShift(RightShiftOperator),
    NullCoalesce(NullCoalesceOperator),
    BinaryXor(BinaryXorOperator),
    LogicalAnd(LogicalAndOperator),
    Instanceof(InstanceofOperator),
    LogicalOr(LogicalOrOperator),
    LogicalXor(LogicalXorOperator),
    BinaryOr(BinaryOrOperator),
    BooleanOr(BooleanOrOperator),
    Extra(ExtraChild),
}

impl NodeParser for BinaryExpressionOperator {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => BinaryExpressionOperator::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => BinaryExpressionOperator::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "!=" => BinaryExpressionOperator::NotEqual(NotEqualOperator(node.range().into())),
            "!==" => {
                BinaryExpressionOperator::NotIdentical(NotIdenticalOperator(node.range().into()))
            }
            "%" => BinaryExpressionOperator::Mod(ModOperator(node.range().into())),
            "&" => BinaryExpressionOperator::BinaryAnd(BinaryAndOperator(node.range().into())),
            "&&" => BinaryExpressionOperator::BooleanAnd(BooleanAndOperator(node.range().into())),
            "*" => BinaryExpressionOperator::Mult(MultOperator(node.range().into())),
            "**" => BinaryExpressionOperator::Exponential(ExponentialOperator(node.range().into())),
            "+" => BinaryExpressionOperator::Add(AddOperator(node.range().into())),
            "-" => BinaryExpressionOperator::Sub(SubOperator(node.range().into())),
            "." => BinaryExpressionOperator::Concat(ConcatOperator(node.range().into())),
            "/" => BinaryExpressionOperator::Div(DivOperator(node.range().into())),
            "<" => BinaryExpressionOperator::LessThan(LessThanOperator(node.range().into())),
            "<<" => BinaryExpressionOperator::LeftShift(LeftShiftOperator(node.range().into())),
            "<=" => BinaryExpressionOperator::LessThanOrEqual(LessThanOrEqualOperator(
                node.range().into(),
            )),
            "<=>" => BinaryExpressionOperator::Spaceship(SpaceshipOperator(node.range().into())),
            "<>" => BinaryExpressionOperator::NotEqual(NotEqualOperator(node.range().into())),
            "==" => BinaryExpressionOperator::Equal(EqualOperator(node.range().into())),
            "===" => BinaryExpressionOperator::Identical(IdenticalOperator(node.range().into())),
            ">" => BinaryExpressionOperator::GreaterThan(GreaterThanOperator(node.range().into())),
            ">=" => BinaryExpressionOperator::GreaterThanOrEqual(GreaterThanOrEqualOperator(
                node.range().into(),
            )),
            ">>" => BinaryExpressionOperator::RightShift(RightShiftOperator(node.range().into())),
            "??" => {
                BinaryExpressionOperator::NullCoalesce(NullCoalesceOperator(node.range().into()))
            }
            "^" => BinaryExpressionOperator::BinaryXor(BinaryXorOperator(node.range().into())),
            "and" => BinaryExpressionOperator::LogicalAnd(LogicalAndOperator(node.range().into())),
            "instanceof" => {
                BinaryExpressionOperator::Instanceof(InstanceofOperator(node.range().into()))
            }
            "or" => BinaryExpressionOperator::LogicalOr(LogicalOrOperator(node.range().into())),
            "xor" => BinaryExpressionOperator::LogicalXor(LogicalXorOperator(node.range().into())),
            "|" => BinaryExpressionOperator::BinaryOr(BinaryOrOperator(node.range().into())),
            "||" => BinaryExpressionOperator::BooleanOr(BooleanOrOperator(node.range().into())),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }
}

impl BinaryExpressionOperator {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => BinaryExpressionOperator::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => BinaryExpressionOperator::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "!=" => BinaryExpressionOperator::NotEqual(NotEqualOperator(node.range().into())),
            "!==" => {
                BinaryExpressionOperator::NotIdentical(NotIdenticalOperator(node.range().into()))
            }
            "%" => BinaryExpressionOperator::Mod(ModOperator(node.range().into())),
            "&" => BinaryExpressionOperator::BinaryAnd(BinaryAndOperator(node.range().into())),
            "&&" => BinaryExpressionOperator::BooleanAnd(BooleanAndOperator(node.range().into())),
            "*" => BinaryExpressionOperator::Mult(MultOperator(node.range().into())),
            "**" => BinaryExpressionOperator::Exponential(ExponentialOperator(node.range().into())),
            "+" => BinaryExpressionOperator::Add(AddOperator(node.range().into())),
            "-" => BinaryExpressionOperator::Sub(SubOperator(node.range().into())),
            "." => BinaryExpressionOperator::Concat(ConcatOperator(node.range().into())),
            "/" => BinaryExpressionOperator::Div(DivOperator(node.range().into())),
            "<" => BinaryExpressionOperator::LessThan(LessThanOperator(node.range().into())),
            "<<" => BinaryExpressionOperator::LeftShift(LeftShiftOperator(node.range().into())),
            "<=" => BinaryExpressionOperator::LessThanOrEqual(LessThanOrEqualOperator(
                node.range().into(),
            )),
            "<=>" => BinaryExpressionOperator::Spaceship(SpaceshipOperator(node.range().into())),
            "<>" => BinaryExpressionOperator::NotEqual(NotEqualOperator(node.range().into())),
            "==" => BinaryExpressionOperator::Equal(EqualOperator(node.range().into())),
            "===" => BinaryExpressionOperator::Identical(IdenticalOperator(node.range().into())),
            ">" => BinaryExpressionOperator::GreaterThan(GreaterThanOperator(node.range().into())),
            ">=" => BinaryExpressionOperator::GreaterThanOrEqual(GreaterThanOrEqualOperator(
                node.range().into(),
            )),
            ">>" => BinaryExpressionOperator::RightShift(RightShiftOperator(node.range().into())),
            "??" => {
                BinaryExpressionOperator::NullCoalesce(NullCoalesceOperator(node.range().into()))
            }
            "^" => BinaryExpressionOperator::BinaryXor(BinaryXorOperator(node.range().into())),
            "and" => BinaryExpressionOperator::LogicalAnd(LogicalAndOperator(node.range().into())),
            "instanceof" => {
                BinaryExpressionOperator::Instanceof(InstanceofOperator(node.range().into()))
            }
            "or" => BinaryExpressionOperator::LogicalOr(LogicalOrOperator(node.range().into())),
            "xor" => BinaryExpressionOperator::LogicalXor(LogicalXorOperator(node.range().into())),
            "|" => BinaryExpressionOperator::BinaryOr(BinaryOrOperator(node.range().into())),
            "||" => BinaryExpressionOperator::BooleanOr(BooleanOrOperator(node.range().into())),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            BinaryExpressionOperator::Extra(y) => y.kind(),
            BinaryExpressionOperator::NotEqual(y) => y.kind(),
            BinaryExpressionOperator::NotIdentical(y) => y.kind(),
            BinaryExpressionOperator::Mod(y) => y.kind(),
            BinaryExpressionOperator::BinaryAnd(y) => y.kind(),
            BinaryExpressionOperator::BooleanAnd(y) => y.kind(),
            BinaryExpressionOperator::Mult(y) => y.kind(),
            BinaryExpressionOperator::Exponential(y) => y.kind(),
            BinaryExpressionOperator::Add(y) => y.kind(),
            BinaryExpressionOperator::Sub(y) => y.kind(),
            BinaryExpressionOperator::Concat(y) => y.kind(),
            BinaryExpressionOperator::Div(y) => y.kind(),
            BinaryExpressionOperator::LessThan(y) => y.kind(),
            BinaryExpressionOperator::LeftShift(y) => y.kind(),
            BinaryExpressionOperator::LessThanOrEqual(y) => y.kind(),
            BinaryExpressionOperator::Spaceship(y) => y.kind(),
            BinaryExpressionOperator::Equal(y) => y.kind(),
            BinaryExpressionOperator::Identical(y) => y.kind(),
            BinaryExpressionOperator::GreaterThan(y) => y.kind(),
            BinaryExpressionOperator::GreaterThanOrEqual(y) => y.kind(),
            BinaryExpressionOperator::RightShift(y) => y.kind(),
            BinaryExpressionOperator::NullCoalesce(y) => y.kind(),
            BinaryExpressionOperator::BinaryXor(y) => y.kind(),
            BinaryExpressionOperator::LogicalAnd(y) => y.kind(),
            BinaryExpressionOperator::Instanceof(y) => y.kind(),
            BinaryExpressionOperator::LogicalOr(y) => y.kind(),
            BinaryExpressionOperator::LogicalXor(y) => y.kind(),
            BinaryExpressionOperator::BinaryOr(y) => y.kind(),
            BinaryExpressionOperator::BooleanOr(y) => y.kind(),
        }
    }

    pub fn parse_vec<'a, I>(children: I, source: &[u8]) -> Result<Vec<Box<Self>>, ParseError>
    where
        I: Iterator<Item = Node<'a>>,
    {
        let mut res: Vec<Box<Self>> = vec![];
        for child in children {
            res.push(Box::new(Self::parse(child, source)?));
        }
        Ok(res)
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
    Extra(ExtraChild),
}

impl NodeParser for BinaryExpressionRight {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => BinaryExpressionRight::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => BinaryExpressionRight::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
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
                    .map(Box::new)
                    .map(BinaryExpressionRight::_Expression)
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
}

impl BinaryExpressionRight {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => BinaryExpressionRight::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => BinaryExpressionRight::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
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
                return Ok(_ExpressionNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(BinaryExpressionRight::_Expression))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            BinaryExpressionRight::Extra(y) => y.kind(),
            BinaryExpressionRight::_Expression(y) => y.kind(),
            BinaryExpressionRight::DynamicVariableName(y) => y.kind(),
            BinaryExpressionRight::MemberAccessExpression(y) => y.kind(),
            BinaryExpressionRight::Name(y) => y.kind(),
            BinaryExpressionRight::NullsafeMemberAccessExpression(y) => y.kind(),
            BinaryExpressionRight::QualifiedName(y) => y.kind(),
            BinaryExpressionRight::ScopedPropertyAccessExpression(y) => y.kind(),
            BinaryExpressionRight::SubscriptExpression(y) => y.kind(),
            BinaryExpressionRight::VariableName(y) => y.kind(),
        }
    }

    pub fn parse_vec<'a, I>(children: I, source: &[u8]) -> Result<Vec<Box<Self>>, ParseError>
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
            BinaryExpressionRight::Extra(x) => x.get_utype(state, emitter),
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
            BinaryExpressionRight::Extra(x) => x.get_php_value(state, emitter),
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
            BinaryExpressionRight::Extra(x) => x.read_from(state, emitter),
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
            BinaryExpressionRight::Extra(x) => {
                format!("BinaryExpressionRight::extra({})", x.brief_desc())
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

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            BinaryExpressionRight::Extra(x) => x.as_any(),
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

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            BinaryExpressionRight::Extra(x) => x.children_any(),
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
            BinaryExpressionRight::Extra(x) => x.range(),
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
    pub left: _ExpressionNode,
    pub operator: Box<BinaryExpressionOperator>,
    pub right: Box<BinaryExpressionRight>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for BinaryExpressionNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
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
        let left: _ExpressionNode = Into::<Result<_, _>>::into(node.parse_child("left", source))?;
        let operator: Box<BinaryExpressionOperator> =
            Into::<Result<_, _>>::into(node.parse_child("operator", source))?;
        let right: Box<BinaryExpressionRight> =
            Into::<Result<_, _>>::into(node.parse_child("right", source))?;
        Ok(Self {
            range,
            left,
            operator,
            right,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl BinaryExpressionNode {
    pub fn kind(&self) -> &'static str {
        "binary_expression"
    }
}

impl NodeAccess for BinaryExpressionNode {
    fn brief_desc(&self) -> String {
        "BinaryExpressionNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::BinaryExpression(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.left.as_any());
        child_vec.push(self.operator.as_any());
        child_vec.push(self.right.as_any());

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
