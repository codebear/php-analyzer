use crate::analysis::state::AnalysisState;
use crate::autonodes::_type::_TypeNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::bottom_type::BottomTypeNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::compound_statement::CompoundStatementNode;
use crate::autonodes::formal_parameters::FormalParametersNode;
use crate::autonodes::name::NameNode;
use crate::autonodes::reference_modifier::ReferenceModifierNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::parser::Range;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum FunctionDefinitionReturnType {
    _Type(Box<_TypeNode>),
    BottomType(Box<BottomTypeNode>),
    Extra(ExtraChild),
}

impl NodeParser for FunctionDefinitionReturnType {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => FunctionDefinitionReturnType::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => FunctionDefinitionReturnType::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "bottom_type" => FunctionDefinitionReturnType::BottomType(Box::new(
                BottomTypeNode::parse(node, source)?,
            )),

            _ => {
                if let Some(x) = _TypeNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(FunctionDefinitionReturnType::_Type)
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

impl FunctionDefinitionReturnType {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => FunctionDefinitionReturnType::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => FunctionDefinitionReturnType::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "bottom_type" => FunctionDefinitionReturnType::BottomType(Box::new(
                BottomTypeNode::parse(node, source)?,
            )),

            _ => {
                return Ok(_TypeNode::parse_opt(node, source)?
                    .map(Box::new)
                    .map(FunctionDefinitionReturnType::_Type))
            }
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            FunctionDefinitionReturnType::Extra(y) => y.kind(),
            FunctionDefinitionReturnType::_Type(y) => y.kind(),
            FunctionDefinitionReturnType::BottomType(y) => y.kind(),
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
            FunctionDefinitionReturnType::Extra(x) => x.get_utype(state, emitter),
            FunctionDefinitionReturnType::_Type(x) => x.get_utype(state, emitter),
            FunctionDefinitionReturnType::BottomType(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            FunctionDefinitionReturnType::Extra(x) => x.get_php_value(state, emitter),
            FunctionDefinitionReturnType::_Type(x) => x.get_php_value(state, emitter),
            FunctionDefinitionReturnType::BottomType(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            FunctionDefinitionReturnType::Extra(x) => x.read_from(state, emitter),
            FunctionDefinitionReturnType::_Type(x) => x.read_from(state, emitter),
            FunctionDefinitionReturnType::BottomType(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for FunctionDefinitionReturnType {
    fn brief_desc(&self) -> String {
        match self {
            FunctionDefinitionReturnType::Extra(x) => {
                format!("FunctionDefinitionReturnType::extra({})", x.brief_desc())
            }
            FunctionDefinitionReturnType::_Type(x) => {
                format!("FunctionDefinitionReturnType::_type({})", x.brief_desc())
            }
            FunctionDefinitionReturnType::BottomType(x) => format!(
                "FunctionDefinitionReturnType::bottom_type({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            FunctionDefinitionReturnType::Extra(x) => x.as_any(),
            FunctionDefinitionReturnType::_Type(x) => x.as_any(),
            FunctionDefinitionReturnType::BottomType(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            FunctionDefinitionReturnType::Extra(x) => x.children_any(),
            FunctionDefinitionReturnType::_Type(x) => x.children_any(),
            FunctionDefinitionReturnType::BottomType(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            FunctionDefinitionReturnType::Extra(x) => x.range(),
            FunctionDefinitionReturnType::_Type(x) => x.range(),
            FunctionDefinitionReturnType::BottomType(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDefinitionNode {
    pub range: Range,
    pub attributes: Option<AttributeListNode>,
    pub body: CompoundStatementNode,
    pub name: NameNode,
    pub parameters: FormalParametersNode,
    pub reference_modifier: Option<ReferenceModifierNode>,
    pub return_type: Option<Box<FunctionDefinitionReturnType>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for FunctionDefinitionNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "function_definition" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [function_definition] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let attributes: Option<AttributeListNode> =
            Into::<Result<_, _>>::into(node.parse_child("attributes", source))?;
        let body: CompoundStatementNode =
            Into::<Result<_, _>>::into(node.parse_child("body", source))?;
        let name: NameNode = Into::<Result<_, _>>::into(node.parse_child("name", source))?;
        let parameters: FormalParametersNode =
            Into::<Result<_, _>>::into(node.parse_child("parameters", source))?;
        let reference_modifier: Option<ReferenceModifierNode> =
            Into::<Result<_, _>>::into(node.parse_child("reference_modifier", source))?;
        let return_type: Option<Box<FunctionDefinitionReturnType>> =
            Into::<Result<_, _>>::into(node.parse_child("return_type", source))?;
        Ok(Self {
            range,
            attributes,
            body,
            name,
            parameters,
            reference_modifier,
            return_type,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl FunctionDefinitionNode {
    pub fn kind(&self) -> &'static str {
        "function_definition"
    }
}

impl NodeAccess for FunctionDefinitionNode {
    fn brief_desc(&self) -> String {
        "FunctionDefinitionNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::FunctionDefinition(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.attributes {
            child_vec.push(x.as_any());
        }
        child_vec.push(self.body.as_any());
        child_vec.push(self.name.as_any());
        child_vec.push(self.parameters.as_any());
        if let Some(x) = &self.reference_modifier {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.return_type {
            child_vec.push(x.as_any());
        }

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
