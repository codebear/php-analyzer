use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::attribute_list::AttributeListNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::const_element::ConstElementNode;
use crate::autonodes::final_modifier::FinalModifierNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::visibility_modifier::VisibilityModifierNode;
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
pub enum ConstDeclarationChildren {
    ConstElement(Box<ConstElementNode>),
    VisibilityModifier(Box<VisibilityModifierNode>),
    Extra(ExtraChild),
}

impl ConstDeclarationChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ConstDeclarationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ConstDeclarationChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
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

    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ConstDeclarationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => ConstDeclarationChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
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

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ConstDeclarationChildren::Extra(x) => x.as_any(),
            ConstDeclarationChildren::ConstElement(x) => x.as_any(),
            ConstDeclarationChildren::VisibilityModifier(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
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
    pub children: Vec<Box<ConstDeclarationChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl ConstDeclarationNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
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
        let modifier: Option<FinalModifierNode> = node
            .children_by_field_name("modifier", &mut node.walk())
            .map(|chnode| {
                skip_nodes.push(chnode.id());
                chnode
            })
            .map(|chnode1| FinalModifierNode::parse(chnode1, source))
            .collect::<Result<Vec<_>, ParseError>>()?
            .drain(..)
            .next();
        Ok(Self {
            range,
            attributes,
            modifier,
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
        "const_declaration"
    }
}

impl NodeAccess for ConstDeclarationNode {
    fn brief_desc(&self) -> String {
        "ConstDeclarationNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::ConstDeclaration(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        if let Some(x) = &self.attributes {
            child_vec.push(x.as_any());
        }
        if let Some(x) = &self.modifier {
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
