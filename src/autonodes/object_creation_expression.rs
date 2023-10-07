use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::arguments::ArgumentsNode;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::base_clause::BaseClauseNode;
use crate::autonodes::class_interface_clause::ClassInterfaceClauseNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::declaration_list::DeclarationListNode;
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
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum ObjectCreationExpressionChildren {
    Arguments(Box<ArgumentsNode>),
    BaseClause(Box<BaseClauseNode>),
    ClassInterfaceClause(Box<ClassInterfaceClauseNode>),
    DeclarationList(Box<DeclarationListNode>),
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

impl ObjectCreationExpressionChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ObjectCreationExpressionChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                ObjectCreationExpressionChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => ObjectCreationExpressionChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "arguments" => ObjectCreationExpressionChildren::Arguments(Box::new(
                ArgumentsNode::parse(node, source)?,
            )),
            "base_clause" => ObjectCreationExpressionChildren::BaseClause(Box::new(
                BaseClauseNode::parse(node, source)?,
            )),
            "class_interface_clause" => ObjectCreationExpressionChildren::ClassInterfaceClause(
                Box::new(ClassInterfaceClauseNode::parse(node, source)?),
            ),
            "declaration_list" => ObjectCreationExpressionChildren::DeclarationList(Box::new(
                DeclarationListNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => ObjectCreationExpressionChildren::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "member_access_expression" => ObjectCreationExpressionChildren::MemberAccessExpression(
                Box::new(MemberAccessExpressionNode::parse(node, source)?),
            ),
            "name" => {
                ObjectCreationExpressionChildren::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nullsafe_member_access_expression" => {
                ObjectCreationExpressionChildren::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => ObjectCreationExpressionChildren::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                ObjectCreationExpressionChildren::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => ObjectCreationExpressionChildren::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => ObjectCreationExpressionChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

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
            "comment" => ObjectCreationExpressionChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                ObjectCreationExpressionChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => ObjectCreationExpressionChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "arguments" => ObjectCreationExpressionChildren::Arguments(Box::new(
                ArgumentsNode::parse(node, source)?,
            )),
            "base_clause" => ObjectCreationExpressionChildren::BaseClause(Box::new(
                BaseClauseNode::parse(node, source)?,
            )),
            "class_interface_clause" => ObjectCreationExpressionChildren::ClassInterfaceClause(
                Box::new(ClassInterfaceClauseNode::parse(node, source)?),
            ),
            "declaration_list" => ObjectCreationExpressionChildren::DeclarationList(Box::new(
                DeclarationListNode::parse(node, source)?,
            )),
            "dynamic_variable_name" => ObjectCreationExpressionChildren::DynamicVariableName(
                Box::new(DynamicVariableNameNode::parse(node, source)?),
            ),
            "member_access_expression" => ObjectCreationExpressionChildren::MemberAccessExpression(
                Box::new(MemberAccessExpressionNode::parse(node, source)?),
            ),
            "name" => {
                ObjectCreationExpressionChildren::Name(Box::new(NameNode::parse(node, source)?))
            }
            "nullsafe_member_access_expression" => {
                ObjectCreationExpressionChildren::NullsafeMemberAccessExpression(Box::new(
                    NullsafeMemberAccessExpressionNode::parse(node, source)?,
                ))
            }
            "qualified_name" => ObjectCreationExpressionChildren::QualifiedName(Box::new(
                QualifiedNameNode::parse(node, source)?,
            )),
            "scoped_property_access_expression" => {
                ObjectCreationExpressionChildren::ScopedPropertyAccessExpression(Box::new(
                    ScopedPropertyAccessExpressionNode::parse(node, source)?,
                ))
            }
            "subscript_expression" => ObjectCreationExpressionChildren::SubscriptExpression(
                Box::new(SubscriptExpressionNode::parse(node, source)?),
            ),
            "variable_name" => ObjectCreationExpressionChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
            )),

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
            ObjectCreationExpressionChildren::Extra(x) => x.get_utype(state, emitter),
            ObjectCreationExpressionChildren::Arguments(x) => x.get_utype(state, emitter),
            ObjectCreationExpressionChildren::BaseClause(x) => x.get_utype(state, emitter),
            ObjectCreationExpressionChildren::ClassInterfaceClause(x) => {
                x.get_utype(state, emitter)
            }
            ObjectCreationExpressionChildren::DeclarationList(x) => x.get_utype(state, emitter),
            ObjectCreationExpressionChildren::DynamicVariableName(x) => x.get_utype(state, emitter),
            ObjectCreationExpressionChildren::MemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ObjectCreationExpressionChildren::Name(x) => x.get_utype(state, emitter),
            ObjectCreationExpressionChildren::NullsafeMemberAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ObjectCreationExpressionChildren::QualifiedName(x) => x.get_utype(state, emitter),
            ObjectCreationExpressionChildren::ScopedPropertyAccessExpression(x) => {
                x.get_utype(state, emitter)
            }
            ObjectCreationExpressionChildren::SubscriptExpression(x) => x.get_utype(state, emitter),
            ObjectCreationExpressionChildren::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ObjectCreationExpressionChildren::Extra(x) => x.get_php_value(state, emitter),
            ObjectCreationExpressionChildren::Arguments(x) => x.get_php_value(state, emitter),
            ObjectCreationExpressionChildren::BaseClause(x) => x.get_php_value(state, emitter),
            ObjectCreationExpressionChildren::ClassInterfaceClause(x) => {
                x.get_php_value(state, emitter)
            }
            ObjectCreationExpressionChildren::DeclarationList(x) => x.get_php_value(state, emitter),
            ObjectCreationExpressionChildren::DynamicVariableName(x) => {
                x.get_php_value(state, emitter)
            }
            ObjectCreationExpressionChildren::MemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ObjectCreationExpressionChildren::Name(x) => x.get_php_value(state, emitter),
            ObjectCreationExpressionChildren::NullsafeMemberAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ObjectCreationExpressionChildren::QualifiedName(x) => x.get_php_value(state, emitter),
            ObjectCreationExpressionChildren::ScopedPropertyAccessExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ObjectCreationExpressionChildren::SubscriptExpression(x) => {
                x.get_php_value(state, emitter)
            }
            ObjectCreationExpressionChildren::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ObjectCreationExpressionChildren::Extra(x) => x.read_from(state, emitter),
            ObjectCreationExpressionChildren::Arguments(x) => x.read_from(state, emitter),
            ObjectCreationExpressionChildren::BaseClause(x) => x.read_from(state, emitter),
            ObjectCreationExpressionChildren::ClassInterfaceClause(x) => {
                x.read_from(state, emitter)
            }
            ObjectCreationExpressionChildren::DeclarationList(x) => x.read_from(state, emitter),
            ObjectCreationExpressionChildren::DynamicVariableName(x) => x.read_from(state, emitter),
            ObjectCreationExpressionChildren::MemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ObjectCreationExpressionChildren::Name(x) => x.read_from(state, emitter),
            ObjectCreationExpressionChildren::NullsafeMemberAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ObjectCreationExpressionChildren::QualifiedName(x) => x.read_from(state, emitter),
            ObjectCreationExpressionChildren::ScopedPropertyAccessExpression(x) => {
                x.read_from(state, emitter)
            }
            ObjectCreationExpressionChildren::SubscriptExpression(x) => x.read_from(state, emitter),
            ObjectCreationExpressionChildren::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ObjectCreationExpressionChildren {
    fn brief_desc(&self) -> String {
        match self {
            ObjectCreationExpressionChildren::Extra(x) => format!(
                "ObjectCreationExpressionChildren::extra({})",
                x.brief_desc()
            ),
            ObjectCreationExpressionChildren::Arguments(x) => format!(
                "ObjectCreationExpressionChildren::arguments({})",
                x.brief_desc()
            ),
            ObjectCreationExpressionChildren::BaseClause(x) => format!(
                "ObjectCreationExpressionChildren::base_clause({})",
                x.brief_desc()
            ),
            ObjectCreationExpressionChildren::ClassInterfaceClause(x) => format!(
                "ObjectCreationExpressionChildren::class_interface_clause({})",
                x.brief_desc()
            ),
            ObjectCreationExpressionChildren::DeclarationList(x) => format!(
                "ObjectCreationExpressionChildren::declaration_list({})",
                x.brief_desc()
            ),
            ObjectCreationExpressionChildren::DynamicVariableName(x) => format!(
                "ObjectCreationExpressionChildren::dynamic_variable_name({})",
                x.brief_desc()
            ),
            ObjectCreationExpressionChildren::MemberAccessExpression(x) => format!(
                "ObjectCreationExpressionChildren::member_access_expression({})",
                x.brief_desc()
            ),
            ObjectCreationExpressionChildren::Name(x) => {
                format!("ObjectCreationExpressionChildren::name({})", x.brief_desc())
            }
            ObjectCreationExpressionChildren::NullsafeMemberAccessExpression(x) => format!(
                "ObjectCreationExpressionChildren::nullsafe_member_access_expression({})",
                x.brief_desc()
            ),
            ObjectCreationExpressionChildren::QualifiedName(x) => format!(
                "ObjectCreationExpressionChildren::qualified_name({})",
                x.brief_desc()
            ),
            ObjectCreationExpressionChildren::ScopedPropertyAccessExpression(x) => format!(
                "ObjectCreationExpressionChildren::scoped_property_access_expression({})",
                x.brief_desc()
            ),
            ObjectCreationExpressionChildren::SubscriptExpression(x) => format!(
                "ObjectCreationExpressionChildren::subscript_expression({})",
                x.brief_desc()
            ),
            ObjectCreationExpressionChildren::VariableName(x) => format!(
                "ObjectCreationExpressionChildren::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ObjectCreationExpressionChildren::Extra(x) => x.as_any(),
            ObjectCreationExpressionChildren::Arguments(x) => x.as_any(),
            ObjectCreationExpressionChildren::BaseClause(x) => x.as_any(),
            ObjectCreationExpressionChildren::ClassInterfaceClause(x) => x.as_any(),
            ObjectCreationExpressionChildren::DeclarationList(x) => x.as_any(),
            ObjectCreationExpressionChildren::DynamicVariableName(x) => x.as_any(),
            ObjectCreationExpressionChildren::MemberAccessExpression(x) => x.as_any(),
            ObjectCreationExpressionChildren::Name(x) => x.as_any(),
            ObjectCreationExpressionChildren::NullsafeMemberAccessExpression(x) => x.as_any(),
            ObjectCreationExpressionChildren::QualifiedName(x) => x.as_any(),
            ObjectCreationExpressionChildren::ScopedPropertyAccessExpression(x) => x.as_any(),
            ObjectCreationExpressionChildren::SubscriptExpression(x) => x.as_any(),
            ObjectCreationExpressionChildren::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ObjectCreationExpressionChildren::Extra(x) => x.children_any(),
            ObjectCreationExpressionChildren::Arguments(x) => x.children_any(),
            ObjectCreationExpressionChildren::BaseClause(x) => x.children_any(),
            ObjectCreationExpressionChildren::ClassInterfaceClause(x) => x.children_any(),
            ObjectCreationExpressionChildren::DeclarationList(x) => x.children_any(),
            ObjectCreationExpressionChildren::DynamicVariableName(x) => x.children_any(),
            ObjectCreationExpressionChildren::MemberAccessExpression(x) => x.children_any(),
            ObjectCreationExpressionChildren::Name(x) => x.children_any(),
            ObjectCreationExpressionChildren::NullsafeMemberAccessExpression(x) => x.children_any(),
            ObjectCreationExpressionChildren::QualifiedName(x) => x.children_any(),
            ObjectCreationExpressionChildren::ScopedPropertyAccessExpression(x) => x.children_any(),
            ObjectCreationExpressionChildren::SubscriptExpression(x) => x.children_any(),
            ObjectCreationExpressionChildren::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ObjectCreationExpressionChildren::Extra(x) => x.range(),
            ObjectCreationExpressionChildren::Arguments(x) => x.range(),
            ObjectCreationExpressionChildren::BaseClause(x) => x.range(),
            ObjectCreationExpressionChildren::ClassInterfaceClause(x) => x.range(),
            ObjectCreationExpressionChildren::DeclarationList(x) => x.range(),
            ObjectCreationExpressionChildren::DynamicVariableName(x) => x.range(),
            ObjectCreationExpressionChildren::MemberAccessExpression(x) => x.range(),
            ObjectCreationExpressionChildren::Name(x) => x.range(),
            ObjectCreationExpressionChildren::NullsafeMemberAccessExpression(x) => x.range(),
            ObjectCreationExpressionChildren::QualifiedName(x) => x.range(),
            ObjectCreationExpressionChildren::ScopedPropertyAccessExpression(x) => x.range(),
            ObjectCreationExpressionChildren::SubscriptExpression(x) => x.range(),
            ObjectCreationExpressionChildren::VariableName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ObjectCreationExpressionNode {
    pub range: Range,
    pub attributes: Option<AttributeListNode>,
    pub children: Vec<Box<ObjectCreationExpressionChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl ObjectCreationExpressionNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "object_creation_expression" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [object_creation_expression] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let attributes: Option<AttributeListNode> = node
            .children_by_field_name("attributes", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| AttributeListNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        Ok(Self {
            range,
            attributes,
            children: ObjectCreationExpressionChildren::parse_vec(
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
        "object_creation_expression"
    }
}

impl NodeAccess for ObjectCreationExpressionNode {
    fn brief_desc(&self) -> String {
        "ObjectCreationExpressionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ObjectCreationExpression(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.attributes {
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
