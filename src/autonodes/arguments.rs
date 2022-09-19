use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::argument::ArgumentNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::variadic_placeholder::VariadicPlaceholderNode;
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
pub enum ArgumentsChildren {
    Argument(Box<ArgumentNode>),
    VariadicPlaceholder(Box<VariadicPlaceholderNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl ArgumentsChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => ArgumentsChildren::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => ArgumentsChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => ArgumentsChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
            "argument" => ArgumentsChildren::Argument(Box::new(ArgumentNode::parse(node, source)?)),
            "variadic_placeholder" => ArgumentsChildren::VariadicPlaceholder(Box::new(
                VariadicPlaceholderNode::parse(node, source)?,
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
            "comment" => ArgumentsChildren::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => ArgumentsChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => ArgumentsChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
            "argument" => ArgumentsChildren::Argument(Box::new(ArgumentNode::parse(node, source)?)),
            "variadic_placeholder" => ArgumentsChildren::VariadicPlaceholder(Box::new(
                VariadicPlaceholderNode::parse(node, source)?,
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
            ArgumentsChildren::Comment(x) => x.get_utype(state, emitter),
            ArgumentsChildren::TextInterpolation(x) => x.get_utype(state, emitter),
            ArgumentsChildren::Error(x) => x.get_utype(state, emitter),
            ArgumentsChildren::Argument(x) => x.get_utype(state, emitter),
            ArgumentsChildren::VariadicPlaceholder(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            ArgumentsChildren::Comment(x) => x.get_php_value(state, emitter),
            ArgumentsChildren::TextInterpolation(x) => x.get_php_value(state, emitter),
            ArgumentsChildren::Error(x) => x.get_php_value(state, emitter),
            ArgumentsChildren::Argument(x) => x.get_php_value(state, emitter),
            ArgumentsChildren::VariadicPlaceholder(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            ArgumentsChildren::Comment(x) => x.read_from(state, emitter),
            ArgumentsChildren::TextInterpolation(x) => x.read_from(state, emitter),
            ArgumentsChildren::Error(x) => x.read_from(state, emitter),
            ArgumentsChildren::Argument(x) => x.read_from(state, emitter),
            ArgumentsChildren::VariadicPlaceholder(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for ArgumentsChildren {
    fn brief_desc(&self) -> String {
        match self {
            ArgumentsChildren::Comment(x) => {
                format!("ArgumentsChildren::comment({})", x.brief_desc())
            }
            ArgumentsChildren::TextInterpolation(x) => {
                format!("ArgumentsChildren::text_interpolation({})", x.brief_desc())
            }
            ArgumentsChildren::Error(x) => format!("ArgumentsChildren::ERROR({})", x.brief_desc()),
            ArgumentsChildren::Argument(x) => {
                format!("ArgumentsChildren::argument({})", x.brief_desc())
            }
            ArgumentsChildren::VariadicPlaceholder(x) => format!(
                "ArgumentsChildren::variadic_placeholder({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            ArgumentsChildren::Comment(x) => x.as_any(),
            ArgumentsChildren::TextInterpolation(x) => x.as_any(),
            ArgumentsChildren::Error(x) => x.as_any(),
            ArgumentsChildren::Argument(x) => x.as_any(),
            ArgumentsChildren::VariadicPlaceholder(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            ArgumentsChildren::Comment(x) => x.children_any(),
            ArgumentsChildren::TextInterpolation(x) => x.children_any(),
            ArgumentsChildren::Error(x) => x.children_any(),
            ArgumentsChildren::Argument(x) => x.children_any(),
            ArgumentsChildren::VariadicPlaceholder(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            ArgumentsChildren::Comment(x) => x.range(),
            ArgumentsChildren::TextInterpolation(x) => x.range(),
            ArgumentsChildren::Error(x) => x.range(),
            ArgumentsChildren::Argument(x) => x.range(),
            ArgumentsChildren::VariadicPlaceholder(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ArgumentsNode {
    pub range: Range,
    pub children: Vec<Box<ArgumentsChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl ArgumentsNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "arguments" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [arguments] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: ArgumentsChildren::parse_vec(
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
        "arguments"
    }
}

impl NodeAccess for ArgumentsNode {
    fn brief_desc(&self) -> String {
        "ArgumentsNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::Arguments(self)
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
