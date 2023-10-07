use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::php_tag::PhpTagNode;
use crate::autonodes::text::TextNode;
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
pub enum TextInterpolationChildren {
    PhpTag(Box<PhpTagNode>),
    Text(Box<TextNode>),
    Extra(ExtraChild),
}

impl TextInterpolationChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => TextInterpolationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                TextInterpolationChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => TextInterpolationChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "php_tag" => {
                TextInterpolationChildren::PhpTag(Box::new(PhpTagNode::parse(node, source)?))
            }
            "text" => TextInterpolationChildren::Text(Box::new(TextNode::parse(node, source)?)),

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
            "comment" => TextInterpolationChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                TextInterpolationChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => TextInterpolationChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "php_tag" => {
                TextInterpolationChildren::PhpTag(Box::new(PhpTagNode::parse(node, source)?))
            }
            "text" => TextInterpolationChildren::Text(Box::new(TextNode::parse(node, source)?)),

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
            TextInterpolationChildren::Extra(x) => x.get_utype(state, emitter),
            TextInterpolationChildren::PhpTag(x) => x.get_utype(state, emitter),
            TextInterpolationChildren::Text(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            TextInterpolationChildren::Extra(x) => x.get_php_value(state, emitter),
            TextInterpolationChildren::PhpTag(x) => x.get_php_value(state, emitter),
            TextInterpolationChildren::Text(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            TextInterpolationChildren::Extra(x) => x.read_from(state, emitter),
            TextInterpolationChildren::PhpTag(x) => x.read_from(state, emitter),
            TextInterpolationChildren::Text(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for TextInterpolationChildren {
    fn brief_desc(&self) -> String {
        match self {
            TextInterpolationChildren::Extra(x) => {
                format!("TextInterpolationChildren::extra({})", x.brief_desc())
            }
            TextInterpolationChildren::PhpTag(x) => {
                format!("TextInterpolationChildren::php_tag({})", x.brief_desc())
            }
            TextInterpolationChildren::Text(x) => {
                format!("TextInterpolationChildren::text({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            TextInterpolationChildren::Extra(x) => x.as_any(),
            TextInterpolationChildren::PhpTag(x) => x.as_any(),
            TextInterpolationChildren::Text(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            TextInterpolationChildren::Extra(x) => x.children_any(),
            TextInterpolationChildren::PhpTag(x) => x.children_any(),
            TextInterpolationChildren::Text(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            TextInterpolationChildren::Extra(x) => x.range(),
            TextInterpolationChildren::PhpTag(x) => x.range(),
            TextInterpolationChildren::Text(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct TextInterpolationNode {
    pub range: Range,
    pub children: Vec<Box<TextInterpolationChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl TextInterpolationNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "text_interpolation" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [text_interpolation] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: TextInterpolationChildren::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() != "comment"),
                source,
            )?,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
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
        "text_interpolation"
    }
}

impl NodeAccess for TextInterpolationNode {
    fn brief_desc(&self) -> String {
        "TextInterpolationNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::TextInterpolation(self)
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.extend(self.children.iter().map(|n| n.as_any()));
        child_vec.extend(self.extras.iter().map(|n| n.as_any()));

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
