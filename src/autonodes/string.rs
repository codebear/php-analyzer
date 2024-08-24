use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::escape_sequence::EscapeSequenceNode;
use crate::autonodes::string_value::StringValueNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::parser::Range;
use crate::types::union::PHPType;
use crate::value::PHPValue;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum StringChildren {
    EscapeSequence(Box<EscapeSequenceNode>),
    StringValue(Box<StringValueNode>),
    Extra(ExtraChild),
}

impl NodeParser for StringChildren {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => StringChildren::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => StringChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            ))),
            "ERROR" => {
                StringChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "escape_sequence" => {
                StringChildren::EscapeSequence(Box::new(EscapeSequenceNode::parse(node, source)?))
            }
            "string_value" => {
                StringChildren::StringValue(Box::new(StringValueNode::parse(node, source)?))
            }

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!(
                        "StringChildren: Parse error, unexpected node-type: {}",
                        node.kind()
                    ),
                ))
            }
        })
    }
}

impl StringChildren {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => StringChildren::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => StringChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            ))),
            "ERROR" => {
                StringChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "escape_sequence" => {
                StringChildren::EscapeSequence(Box::new(EscapeSequenceNode::parse(node, source)?))
            }
            "string_value" => {
                StringChildren::StringValue(Box::new(StringValueNode::parse(node, source)?))
            }

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            StringChildren::Extra(y) => y.kind(),
            StringChildren::EscapeSequence(y) => y.kind(),
            StringChildren::StringValue(y) => y.kind(),
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
    ) -> Option<PHPType> {
        match self {
            StringChildren::Extra(x) => x.get_utype(state, emitter),
            StringChildren::EscapeSequence(x) => x.get_utype(state, emitter),
            StringChildren::StringValue(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            StringChildren::Extra(x) => x.get_php_value(state, emitter),
            StringChildren::EscapeSequence(x) => x.get_php_value(state, emitter),
            StringChildren::StringValue(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            StringChildren::Extra(x) => x.read_from(state, emitter),
            StringChildren::EscapeSequence(x) => x.read_from(state, emitter),
            StringChildren::StringValue(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for StringChildren {
    fn brief_desc(&self) -> String {
        match self {
            StringChildren::Extra(x) => format!("StringChildren::extra({})", x.brief_desc()),
            StringChildren::EscapeSequence(x) => {
                format!("StringChildren::escape_sequence({})", x.brief_desc())
            }
            StringChildren::StringValue(x) => {
                format!("StringChildren::string_value({})", x.brief_desc())
            }
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            StringChildren::Extra(x) => x.as_any(),
            StringChildren::EscapeSequence(x) => x.as_any(),
            StringChildren::StringValue(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            StringChildren::Extra(x) => x.children_any(),
            StringChildren::EscapeSequence(x) => x.children_any(),
            StringChildren::StringValue(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            StringChildren::Extra(x) => x.range(),
            StringChildren::EscapeSequence(x) => x.range(),
            StringChildren::StringValue(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StringNode {
    pub range: Range,
    pub children: Vec<Box<StringChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for StringNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "string" {
            return Err(ParseError::new(
                range,
                format!(
                    "StringNode: Node is of the wrong kind [{}] vs expected [string] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: StringChildren::parse_vec(
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
}

impl StringNode {
    pub fn kind(&self) -> &'static str {
        "string"
    }
}

impl NodeAccess for StringNode {
    fn brief_desc(&self) -> String {
        "StringNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::String(self)
    }

    #[allow(clippy::vec_init_then_push)]
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
