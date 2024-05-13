use crate::analysis::state::AnalysisState;
use crate::autonodes::_type::_TypeNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::const_element::ConstElementNode;
use crate::autonodes::final_modifier::FinalModifierNode;
use crate::autonodes::visibility_modifier::VisibilityModifierNode;
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
pub enum ConstDeclarationChildren {
    ConstElement(Box<ConstElementNode>),
    VisibilityModifier(Box<VisibilityModifierNode>),
    Extra(ExtraChild),
}

impl NodeParser for ConstDeclarationChildren {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ConstDeclarationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ConstDeclarationChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "const_element" => ConstDeclarationChildren::ConstElement(Box::new(
                ConstElementNode::parse(node, source)?,
            )),
            "visibility_modifier" => ConstDeclarationChildren::VisibilityModifier(Box::new(
                VisibilityModifierNode::parse(node, source)?,
            )),

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!("Parse error, unexpected node-type: {}", node.kind()),
                ))
            }
        })
    }
}

impl ConstDeclarationChildren {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ConstDeclarationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "ERROR" => ConstDeclarationChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "const_element" => ConstDeclarationChildren::ConstElement(Box::new(
                ConstElementNode::parse(node, source)?,
            )),
            "visibility_modifier" => ConstDeclarationChildren::VisibilityModifier(Box::new(
                VisibilityModifierNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            ConstDeclarationChildren::Extra(y) => y.kind(),
            ConstDeclarationChildren::ConstElement(y) => y.kind(),
            ConstDeclarationChildren::VisibilityModifier(y) => y.kind(),
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
            ConstDeclarationChildren::Extra(x) => x.get_utype(state, emitter),
            ConstDeclarationChildren::ConstElement(x) => x.get_utype(state, emitter),
            ConstDeclarationChildren::VisibilityModifier(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ConstDeclarationChildren::Extra(x) => x.get_php_value(state, emitter),
            ConstDeclarationChildren::ConstElement(x) => x.get_php_value(state, emitter),
            ConstDeclarationChildren::VisibilityModifier(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ConstDeclarationChildren::Extra(x) => x.read_from(state, emitter),
            ConstDeclarationChildren::ConstElement(x) => x.read_from(state, emitter),
            ConstDeclarationChildren::VisibilityModifier(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ConstDeclarationChildren {
    fn brief_desc(&self) -> String {
        match self {
            ConstDeclarationChildren::Extra(x) => {
                format!("ConstDeclarationChildren::extra({})", x.brief_desc())
            }
            ConstDeclarationChildren::ConstElement(x) => format!(
                "ConstDeclarationChildren::const_element({})",
                x.brief_desc()
            ),
            ConstDeclarationChildren::VisibilityModifier(x) => format!(
                "ConstDeclarationChildren::visibility_modifier({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            ConstDeclarationChildren::Extra(x) => x.as_any(),
            ConstDeclarationChildren::ConstElement(x) => x.as_any(),
            ConstDeclarationChildren::VisibilityModifier(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            ConstDeclarationChildren::Extra(x) => x.children_any(),
            ConstDeclarationChildren::ConstElement(x) => x.children_any(),
            ConstDeclarationChildren::VisibilityModifier(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ConstDeclarationChildren::Extra(x) => x.range(),
            ConstDeclarationChildren::ConstElement(x) => x.range(),
            ConstDeclarationChildren::VisibilityModifier(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConstDeclarationNode {
    pub range: Range,
    pub attributes: Option<AttributeListNode>,
    pub modifier: Option<FinalModifierNode>,
    pub type_: Option<_TypeNode>,
    pub children: Vec<Box<ConstDeclarationChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ConstDeclarationNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "const_declaration" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [const_declaration] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }
        let mut skip_nodes: Vec<usize> = vec![];
        let attributes: Option<AttributeListNode> = Into::<Result<_, _>>::into(
            node.parse_child("attributes", source)
                .mark_skipped_node(&mut skip_nodes),
        )?;
        let modifier: Option<FinalModifierNode> = Into::<Result<_, _>>::into(
            node.parse_child("modifier", source)
                .mark_skipped_node(&mut skip_nodes),
        )?;
        let type_: Option<_TypeNode> = Into::<Result<_, _>>::into(
            node.parse_child("type", source)
                .mark_skipped_node(&mut skip_nodes),
        )?;
        Ok(Self {
            range,
            attributes,
            modifier,
            type_,
            children: ConstDeclarationChildren::parse_vec(
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
}

impl ConstDeclarationNode {
    pub fn kind(&self) -> &'static str {
        "const_declaration"
    }
}

impl NodeAccess for ConstDeclarationNode {
    fn brief_desc(&self) -> String {
        "ConstDeclarationNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::ConstDeclaration(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.attributes {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.modifier {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.type_ {
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
