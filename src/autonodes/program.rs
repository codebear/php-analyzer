use crate::analysis::state::AnalysisState;
use crate::autonodes::_statement::_StatementNode;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::php_tag::PhpTagNode;
use crate::autonodes::text::TextNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;

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
pub enum ProgramChildren {
    _Statement(Box<_StatementNode>),
    PhpTag(Box<PhpTagNode>),
    Text(Box<TextNode>),
    Extra(ExtraChild),
}

impl NodeParser for ProgramChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ProgramChildren::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => ProgramChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => {
                ProgramChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "php_tag" => ProgramChildren::PhpTag(Box::new(PhpTagNode::parse(node, source)?)),
            "text" => ProgramChildren::Text(Box::new(TextNode::parse(node, source)?)),

            _ => {
                if let Some(x) = _StatementNode::parse_opt(node, source)?
                    .map(|x| Box::new(x))
                    .map(|y| ProgramChildren::_Statement(y))
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

impl ProgramChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => ProgramChildren::Extra(ExtraChild::Comment(Box::new(CommentNode::parse(
                node, source,
            )?))),
            "text_interpolation" => ProgramChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => {
                ProgramChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(node, source)?)))
            }
            "php_tag" => ProgramChildren::PhpTag(Box::new(PhpTagNode::parse(node, source)?)),
            "text" => ProgramChildren::Text(Box::new(TextNode::parse(node, source)?)),

            _ => {
                return Ok(
                    if let Some(x) = _StatementNode::parse_opt(node, source)?
                        .map(|x| Box::new(x))
                        .map(|y| ProgramChildren::_Statement(y))
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
            ProgramChildren::Extra(y) => y.kind(),
            ProgramChildren::_Statement(y) => y.kind(),
            ProgramChildren::PhpTag(y) => y.kind(),
            ProgramChildren::Text(y) => y.kind(),
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
            ProgramChildren::Extra(x) => x.get_utype(state, emitter),
            ProgramChildren::_Statement(x) => x.get_utype(state, emitter),
            ProgramChildren::PhpTag(x) => x.get_utype(state, emitter),
            ProgramChildren::Text(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ProgramChildren::Extra(x) => x.get_php_value(state, emitter),
            ProgramChildren::_Statement(x) => x.get_php_value(state, emitter),
            ProgramChildren::PhpTag(x) => x.get_php_value(state, emitter),
            ProgramChildren::Text(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ProgramChildren::Extra(x) => x.read_from(state, emitter),
            ProgramChildren::_Statement(x) => x.read_from(state, emitter),
            ProgramChildren::PhpTag(x) => x.read_from(state, emitter),
            ProgramChildren::Text(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ProgramChildren {
    fn brief_desc(&self) -> String {
        match self {
            ProgramChildren::Extra(x) => format!("ProgramChildren::extra({})", x.brief_desc()),
            ProgramChildren::_Statement(x) => {
                format!("ProgramChildren::_statement({})", x.brief_desc())
            }
            ProgramChildren::PhpTag(x) => format!("ProgramChildren::php_tag({})", x.brief_desc()),
            ProgramChildren::Text(x) => format!("ProgramChildren::text({})", x.brief_desc()),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ProgramChildren::Extra(x) => x.as_any(),
            ProgramChildren::_Statement(x) => x.as_any(),
            ProgramChildren::PhpTag(x) => x.as_any(),
            ProgramChildren::Text(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ProgramChildren::Extra(x) => x.children_any(),
            ProgramChildren::_Statement(x) => x.children_any(),
            ProgramChildren::PhpTag(x) => x.children_any(),
            ProgramChildren::Text(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ProgramChildren::Extra(x) => x.range(),
            ProgramChildren::_Statement(x) => x.range(),
            ProgramChildren::PhpTag(x) => x.range(),
            ProgramChildren::Text(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProgramNode {
    pub range: Range,
    pub children: Vec<Box<ProgramChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for ProgramNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "program" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [program] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: ProgramChildren::parse_vec(
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

impl ProgramNode {
    pub fn kind(&self) -> &'static str {
        "program"
    }
}

impl NodeAccess for ProgramNode {
    fn brief_desc(&self) -> String {
        "ProgramNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::Program(self)
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
