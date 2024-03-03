use crate::analysis::state::AnalysisState;
use crate::autonodes::_expression::_ExpressionNode;
use crate::autonodes::_type::_TypeNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::bottom_type::BottomTypeNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::formal_parameters::FormalParametersNode;
use crate::autonodes::reference_modifier::ReferenceModifierNode;
use crate::autonodes::static_modifier::StaticModifierNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum ArrowFunctionReturnType {
    _Type(Box<_TypeNode>),
    BottomType(Box<BottomTypeNode>),
    Extra(ExtraChild),
}

impl NodeParser for ArrowFunctionReturnType {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ArrowFunctionReturnType::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ArrowFunctionReturnType::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ArrowFunctionReturnType::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "bottom_type" => {
                ArrowFunctionReturnType::BottomType(Box::new(BottomTypeNode::parse(node, source)?))
            }

            _ => {
                if let Some(x) = _TypeNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| ArrowFunctionReturnType::_Type(y))
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

impl ArrowFunctionReturnType {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ArrowFunctionReturnType::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ArrowFunctionReturnType::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => ArrowFunctionReturnType::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "bottom_type" => {
                ArrowFunctionReturnType::BottomType(Box::new(BottomTypeNode::parse(node, source)?))
            }

            _ => {
                return Ok(
                    if let Some(x) = _TypeNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| ArrowFunctionReturnType::_Type(y))
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
        match self {
            ArrowFunctionReturnType::Extra(y) => y.kind(),
            ArrowFunctionReturnType::_Type(y) => y.kind(),
            ArrowFunctionReturnType::BottomType(y) => y.kind(),
        }
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
            ArrowFunctionReturnType::Extra(x) => x.get_utype(state, emitter),
            ArrowFunctionReturnType::_Type(x) => x.get_utype(state, emitter),
            ArrowFunctionReturnType::BottomType(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ArrowFunctionReturnType::Extra(x) => x.get_php_value(state, emitter),
            ArrowFunctionReturnType::_Type(x) => x.get_php_value(state, emitter),
            ArrowFunctionReturnType::BottomType(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ArrowFunctionReturnType::Extra(x) => x.read_from(state, emitter),
            ArrowFunctionReturnType::_Type(x) => x.read_from(state, emitter),
            ArrowFunctionReturnType::BottomType(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ArrowFunctionReturnType {
    fn brief_desc(&self) -> String {
        match self {
            ArrowFunctionReturnType::Extra(x) => {
                format!("ArrowFunctionReturnType::extra({})", x.brief_desc())
            }
            ArrowFunctionReturnType::_Type(x) => {
                format!("ArrowFunctionReturnType::_type({})", x.brief_desc())
            }
            ArrowFunctionReturnType::BottomType(x) => {
                format!("ArrowFunctionReturnType::bottom_type({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ArrowFunctionReturnType::Extra(x) => x.as_any(),
            ArrowFunctionReturnType::_Type(x) => x.as_any(),
            ArrowFunctionReturnType::BottomType(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ArrowFunctionReturnType::Extra(x) => x.children_any(),
            ArrowFunctionReturnType::_Type(x) => x.children_any(),
            ArrowFunctionReturnType::BottomType(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ArrowFunctionReturnType::Extra(x) => x.range(),
            ArrowFunctionReturnType::_Type(x) => x.range(),
            ArrowFunctionReturnType::BottomType(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArrowFunctionNode {
    pub range: Range,
    pub attributes: Option<AttributeListNode>,
    pub body: _ExpressionNode,
    pub parameters: FormalParametersNode,
    pub reference_modifier: Option<ReferenceModifierNode>,
    pub return_type: Option<Box<ArrowFunctionReturnType>>,
    pub child: Option<Box<StaticModifierNode>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ArrowFunctionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "arrow_function" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [arrow_function] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let attributes: Option<AttributeListNode> = Result::from(
            node.parse_child("attributes", source)
                .mark_skipped_node(&mut skip_nodes)
                .into(),
        )?;
        let body: _ExpressionNode = Result::from(
            node.parse_child("body", source)
                .mark_skipped_node(&mut skip_nodes)
                .into(),
        )?;
        let parameters: FormalParametersNode = Result::from(
            node.parse_child("parameters", source)
                .mark_skipped_node(&mut skip_nodes)
                .into(),
        )?;
        let reference_modifier: Option<ReferenceModifierNode> = Result::from(
            node.parse_child("reference_modifier", source)
                .mark_skipped_node(&mut skip_nodes)
                .into(),
        )?;
        let return_type: Option<Box<ArrowFunctionReturnType>> = Result::from(
            node.parse_child("return_type", source)
                .mark_skipped_node(&mut skip_nodes)
                .into(),
        )?;
        Ok(Self {
            range,
            attributes,
            body,
            parameters,
            reference_modifier,
            return_type,
            child: node
                .named_children(&mut node.walk())
                .filter(|node| !skip_nodes.contains(&node.id()))
                .filter(|node| node.kind() != "comment")
                .map(|k| StaticModifierNode::parse(k, source))
                .collect::<Result<Vec<StaticModifierNode>, ParseError>>()?
                .drain(..)
                .map(|j| Box::new(j))
                .next(),
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment")
                    .filter(|node| !skip_nodes.contains(&node.id())),
                source,
            )?,
        })
    }
}

impl ArrowFunctionNode {
    pub fn kind(&self) -> &'static str {
        "arrow_function"
    }
}

impl NodeAccess for ArrowFunctionNode {
    fn brief_desc(&self) -> String {
        "ArrowFunctionNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ArrowFunction(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.attributes {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.body.as_any());
        child_vec.push(self.parameters.as_any());
        if let Some(x) = &self.reference_modifier {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.return_type {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.child {
            child_vec.push(x.as_any());
        }
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
