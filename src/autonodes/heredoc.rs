use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::heredoc_body::HeredocBodyNode;
use crate::autonodes::heredoc_end::HeredocEndNode;
use crate::autonodes::heredoc_start::HeredocStartNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autotree::ChildNodeParser;
use crate::autotree::NodeAccess;
use crate::autotree::NodeParser;
use crate::autotree::ParseError;
use crate::errornode::ErrorNode;
use crate::extra::ExtraChild;
use crate::issue::IssueEmitter;
use crate::parser::Range;
use crate::types::union::DiscreteType;
use crate::types::union::PHPType;
use crate::value::PHPValue;
use std::ffi::OsStr;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum HeredocIdentifier {
    DoubleQuote(&'static str, Range),
    HeredocStart(Box<HeredocStartNode>),
    Extra(ExtraChild),
}

impl NodeParser for HeredocIdentifier {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => HeredocIdentifier::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => HeredocIdentifier::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => HeredocIdentifier::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            r#"""# => HeredocIdentifier::DoubleQuote(r#"""#, node.range().into()),
            "heredoc_start" => {
                HeredocIdentifier::HeredocStart(Box::new(HeredocStartNode::parse(node, source)?))
            }

            _ => {
                return Err(ParseError::new(
                    node.range(),
                    format!(
                        "HeredocIdentifier: Parse error, unexpected node-type: {}",
                        node.kind()
                    ),
                ))
            }
        })
    }
}

impl HeredocIdentifier {
    pub fn parse_opt(node: Node, source: &[u8]) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => HeredocIdentifier::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => HeredocIdentifier::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => HeredocIdentifier::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            r#"""# => HeredocIdentifier::DoubleQuote(r#"""#, node.range().into()),
            "heredoc_start" => {
                HeredocIdentifier::HeredocStart(Box::new(HeredocStartNode::parse(node, source)?))
            }

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            HeredocIdentifier::Extra(y) => y.kind(),
            HeredocIdentifier::DoubleQuote(y, _) => y,
            HeredocIdentifier::HeredocStart(y) => y.kind(),
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
            HeredocIdentifier::Extra(x) => x.get_utype(state, emitter),
            HeredocIdentifier::DoubleQuote(_, _) => Some(DiscreteType::String.into()),
            HeredocIdentifier::HeredocStart(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            HeredocIdentifier::Extra(x) => x.get_php_value(state, emitter),
            HeredocIdentifier::DoubleQuote(a, _) => {
                Some(PHPValue::String(OsStr::new(a).to_os_string()))
            }
            HeredocIdentifier::HeredocStart(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            HeredocIdentifier::Extra(x) => x.read_from(state, emitter),
            HeredocIdentifier::DoubleQuote(_, _) => (),
            HeredocIdentifier::HeredocStart(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for HeredocIdentifier {
    fn brief_desc(&self) -> String {
        match self {
            HeredocIdentifier::Extra(x) => format!("HeredocIdentifier::extra({})", x.brief_desc()),
            HeredocIdentifier::DoubleQuote(a, _) => a.to_string(),
            HeredocIdentifier::HeredocStart(x) => {
                format!("HeredocIdentifier::heredoc_start({})", x.brief_desc())
            }
        }
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        match self {
            HeredocIdentifier::Extra(x) => x.as_any(),
            HeredocIdentifier::DoubleQuote(a, b) => AnyNodeRef::StaticExpr(a, *b),
            HeredocIdentifier::HeredocStart(x) => x.as_any(),
        }
    }

    fn children_any(&self) -> Vec<AnyNodeRef<'_>> {
        match self {
            HeredocIdentifier::Extra(x) => x.children_any(),
            HeredocIdentifier::DoubleQuote(_, _) => todo!("Crap"),
            HeredocIdentifier::HeredocStart(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            HeredocIdentifier::Extra(x) => x.range(),
            HeredocIdentifier::DoubleQuote(_, r) => *r,
            HeredocIdentifier::HeredocStart(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeredocNode {
    pub range: Range,
    pub end_tag: HeredocEndNode,
    pub identifier: Vec<Box<HeredocIdentifier>>,
    pub value: Option<HeredocBodyNode>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for HeredocNode {
    fn parse(node: Node, source: &[u8]) -> Result<Self, ParseError> {
        let range: Range = node.range().into();
        if node.kind() != "heredoc" {
            return Err(ParseError::new(range, format!("HeredocNode: Node is of the wrong kind [{}] vs expected [heredoc] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }
        let end_tag: HeredocEndNode =
            Into::<Result<_, _>>::into(node.parse_child("end_tag", source))?;
        let identifier: Vec<Box<HeredocIdentifier>> =
            Into::<Result<_, _>>::into(node.parse_child("identifier", source))?;
        let value: Option<HeredocBodyNode> =
            Into::<Result<_, _>>::into(node.parse_child("value", source))?;
        Ok(Self {
            range,
            end_tag,
            identifier,
            value,
            extras: ExtraChild::parse_vec(
                node.named_children(&mut node.walk())
                    .filter(|node| node.kind() == "comment"),
                source,
            )
            .unwrap(),
        })
    }
}

impl HeredocNode {
    pub fn kind(&self) -> &'static str {
        "heredoc"
    }
}

impl NodeAccess for HeredocNode {
    fn brief_desc(&self) -> String {
        "HeredocNode".into()
    }

    fn as_any(&self) -> AnyNodeRef<'_> {
        AnyNodeRef::Heredoc(self)
    }

    #[allow(clippy::vec_init_then_push)]
    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        let mut child_vec: Vec<AnyNodeRef<'a>> = vec![];

        // let any_children: Vec<AnyNodeRef<'a>> = self.children.iter().map(|x| x.as_any()).collect();
        child_vec.push(self.end_tag.as_any());
        child_vec.extend(self.identifier.iter().map(|v| v.as_any()));
        if let Some(x) = &self.value {
            child_vec.push(x.as_any());
        }

        child_vec.sort_by(|a, b| a.range().start_byte.cmp(&b.range().start_byte));
        child_vec
    }

    fn range(&self) -> Range {
        self.range
    }
}
