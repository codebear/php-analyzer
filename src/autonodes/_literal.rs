use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::boolean::BooleanNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::encapsed_string::EncapsedStringNode;
use crate::autonodes::float::FloatNode;
use crate::autonodes::heredoc::HeredocNode;
use crate::autonodes::integer::IntegerNode;
use crate::autonodes::nowdoc::NowdocNode;
use crate::autonodes::null::NullNode;
use crate::autonodes::string::StringNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autotree::NodeAccess;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::issue::IssueEmitter;
use crate::types::union::UnionType;
use crate::value::PHPValue;
use tree_sitter::Node;
use tree_sitter::Range;

#[derive(Debug, Clone)]
pub enum _LiteralNode {
    Boolean(Box<BooleanNode>),
    EncapsedString(Box<EncapsedStringNode>),
    Float(Box<FloatNode>),
    Heredoc(Box<HeredocNode>),
    Integer(Box<IntegerNode>),
    Nowdoc(Box<NowdocNode>),
    Null(Box<NullNode>),
    String(Box<StringNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl _LiteralNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => _LiteralNode::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => _LiteralNode::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => _LiteralNode::Error(Box::new(ErrorNode::parse(node, source)?)),
            "boolean" => _LiteralNode::Boolean(Box::new(BooleanNode::parse(node, source)?)),
            "encapsed_string" => {
                _LiteralNode::EncapsedString(Box::new(EncapsedStringNode::parse(node, source)?))
            }
            "float" => _LiteralNode::Float(Box::new(FloatNode::parse(node, source)?)),
            "heredoc" => _LiteralNode::Heredoc(Box::new(HeredocNode::parse(node, source)?)),
            "integer" => _LiteralNode::Integer(Box::new(IntegerNode::parse(node, source)?)),
            "nowdoc" => _LiteralNode::Nowdoc(Box::new(NowdocNode::parse(node, source)?)),
            "null" => _LiteralNode::Null(Box::new(NullNode::parse(node, source)?)),
            "string" => _LiteralNode::String(Box::new(StringNode::parse(node, source)?)),

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
            "comment" => _LiteralNode::Comment(Box::new(CommentNode::parse(node, source)?)),
            "text_interpolation" => _LiteralNode::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => _LiteralNode::Error(Box::new(ErrorNode::parse(node, source)?)),
            "boolean" => _LiteralNode::Boolean(Box::new(BooleanNode::parse(node, source)?)),
            "encapsed_string" => {
                _LiteralNode::EncapsedString(Box::new(EncapsedStringNode::parse(node, source)?))
            }
            "float" => _LiteralNode::Float(Box::new(FloatNode::parse(node, source)?)),
            "heredoc" => _LiteralNode::Heredoc(Box::new(HeredocNode::parse(node, source)?)),
            "integer" => _LiteralNode::Integer(Box::new(IntegerNode::parse(node, source)?)),
            "nowdoc" => _LiteralNode::Nowdoc(Box::new(NowdocNode::parse(node, source)?)),
            "null" => _LiteralNode::Null(Box::new(NullNode::parse(node, source)?)),
            "string" => _LiteralNode::String(Box::new(StringNode::parse(node, source)?)),

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
            _LiteralNode::Comment(x) => x.get_utype(state, emitter),
            _LiteralNode::TextInterpolation(x) => x.get_utype(state, emitter),
            _LiteralNode::Error(x) => x.get_utype(state, emitter),
            _LiteralNode::Boolean(x) => x.get_utype(state, emitter),
            _LiteralNode::EncapsedString(x) => x.get_utype(state, emitter),
            _LiteralNode::Float(x) => x.get_utype(state, emitter),
            _LiteralNode::Heredoc(x) => x.get_utype(state, emitter),
            _LiteralNode::Integer(x) => x.get_utype(state, emitter),
            _LiteralNode::Nowdoc(x) => x.get_utype(state, emitter),
            _LiteralNode::Null(x) => x.get_utype(state, emitter),
            _LiteralNode::String(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            _LiteralNode::Comment(x) => x.get_php_value(state, emitter),
            _LiteralNode::TextInterpolation(x) => x.get_php_value(state, emitter),
            _LiteralNode::Error(x) => x.get_php_value(state, emitter),
            _LiteralNode::Boolean(x) => x.get_php_value(state, emitter),
            _LiteralNode::EncapsedString(x) => x.get_php_value(state, emitter),
            _LiteralNode::Float(x) => x.get_php_value(state, emitter),
            _LiteralNode::Heredoc(x) => x.get_php_value(state, emitter),
            _LiteralNode::Integer(x) => x.get_php_value(state, emitter),
            _LiteralNode::Nowdoc(x) => x.get_php_value(state, emitter),
            _LiteralNode::Null(x) => x.get_php_value(state, emitter),
            _LiteralNode::String(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            _LiteralNode::Comment(x) => x.read_from(state, emitter),
            _LiteralNode::TextInterpolation(x) => x.read_from(state, emitter),
            _LiteralNode::Error(x) => x.read_from(state, emitter),
            _LiteralNode::Boolean(x) => x.read_from(state, emitter),
            _LiteralNode::EncapsedString(x) => x.read_from(state, emitter),
            _LiteralNode::Float(x) => x.read_from(state, emitter),
            _LiteralNode::Heredoc(x) => x.read_from(state, emitter),
            _LiteralNode::Integer(x) => x.read_from(state, emitter),
            _LiteralNode::Nowdoc(x) => x.read_from(state, emitter),
            _LiteralNode::Null(x) => x.read_from(state, emitter),
            _LiteralNode::String(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for _LiteralNode {
    fn brief_desc(&self) -> String {
        match self {
            _LiteralNode::Comment(x) => format!("_LiteralNode::comment({})", x.brief_desc()),
            _LiteralNode::TextInterpolation(x) => {
                format!("_LiteralNode::text_interpolation({})", x.brief_desc())
            }
            _LiteralNode::Error(x) => format!("_LiteralNode::ERROR({})", x.brief_desc()),
            _LiteralNode::Boolean(x) => format!("_LiteralNode::boolean({})", x.brief_desc()),
            _LiteralNode::EncapsedString(x) => {
                format!("_LiteralNode::encapsed_string({})", x.brief_desc())
            }
            _LiteralNode::Float(x) => format!("_LiteralNode::float({})", x.brief_desc()),
            _LiteralNode::Heredoc(x) => format!("_LiteralNode::heredoc({})", x.brief_desc()),
            _LiteralNode::Integer(x) => format!("_LiteralNode::integer({})", x.brief_desc()),
            _LiteralNode::Nowdoc(x) => format!("_LiteralNode::nowdoc({})", x.brief_desc()),
            _LiteralNode::Null(x) => format!("_LiteralNode::null({})", x.brief_desc()),
            _LiteralNode::String(x) => format!("_LiteralNode::string({})", x.brief_desc()),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            _LiteralNode::Comment(x) => x.as_any(),
            _LiteralNode::TextInterpolation(x) => x.as_any(),
            _LiteralNode::Error(x) => x.as_any(),
            _LiteralNode::Boolean(x) => x.as_any(),
            _LiteralNode::EncapsedString(x) => x.as_any(),
            _LiteralNode::Float(x) => x.as_any(),
            _LiteralNode::Heredoc(x) => x.as_any(),
            _LiteralNode::Integer(x) => x.as_any(),
            _LiteralNode::Nowdoc(x) => x.as_any(),
            _LiteralNode::Null(x) => x.as_any(),
            _LiteralNode::String(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            _LiteralNode::Comment(x) => x.children_any(),
            _LiteralNode::TextInterpolation(x) => x.children_any(),
            _LiteralNode::Error(x) => x.children_any(),
            _LiteralNode::Boolean(x) => x.children_any(),
            _LiteralNode::EncapsedString(x) => x.children_any(),
            _LiteralNode::Float(x) => x.children_any(),
            _LiteralNode::Heredoc(x) => x.children_any(),
            _LiteralNode::Integer(x) => x.children_any(),
            _LiteralNode::Nowdoc(x) => x.children_any(),
            _LiteralNode::Null(x) => x.children_any(),
            _LiteralNode::String(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            _LiteralNode::Comment(x) => x.range(),
            _LiteralNode::TextInterpolation(x) => x.range(),
            _LiteralNode::Error(x) => x.range(),
            _LiteralNode::Boolean(x) => x.range(),
            _LiteralNode::EncapsedString(x) => x.range(),
            _LiteralNode::Float(x) => x.range(),
            _LiteralNode::Heredoc(x) => x.range(),
            _LiteralNode::Integer(x) => x.range(),
            _LiteralNode::Nowdoc(x) => x.range(),
            _LiteralNode::Null(x) => x.range(),
            _LiteralNode::String(x) => x.range(),
        }
    }
}
