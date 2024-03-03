use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::case_statement::CaseStatementNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::default_statement::DefaultStatementNode;
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
pub enum SwitchBlockChildren {
    CaseStatement(Box<CaseStatementNode>),
    DefaultStatement(Box<DefaultStatementNode>),
    Extra(ExtraChild),
}

impl NodeParser for SwitchBlockChildren {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => SwitchBlockChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => SwitchBlockChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => SwitchBlockChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "case_statement" => SwitchBlockChildren::CaseStatement(Box::new(
                CaseStatementNode::parse(node, source)?,
            )),
            "default_statement" => SwitchBlockChildren::DefaultStatement(Box::new(
                DefaultStatementNode::parse(node, source)?,
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

impl SwitchBlockChildren {
    pub fn parse_opt(node: Node, source: &Vec<u8>) -> Result<Option<Self>, ParseError> {
        Ok(Some(match node.kind() {
            "comment" => SwitchBlockChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => SwitchBlockChildren::Extra(ExtraChild::TextInterpolation(
                Box::new(TextInterpolationNode::parse(node, source)?),
            )),
            "ERROR" => SwitchBlockChildren::Extra(ExtraChild::Error(Box::new(ErrorNode::parse(
                node, source,
            )?))),
            "case_statement" => SwitchBlockChildren::CaseStatement(Box::new(
                CaseStatementNode::parse(node, source)?,
            )),
            "default_statement" => SwitchBlockChildren::DefaultStatement(Box::new(
                DefaultStatementNode::parse(node, source)?,
            )),

            _ => return Ok(None),
        }))
    }

    pub fn kind(&self) -> &'static str {
        match self {
            SwitchBlockChildren::Extra(y) => y.kind(),
            SwitchBlockChildren::CaseStatement(y) => y.kind(),
            SwitchBlockChildren::DefaultStatement(y) => y.kind(),
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
            SwitchBlockChildren::Extra(x) => x.get_utype(state, emitter),
            SwitchBlockChildren::CaseStatement(x) => x.get_utype(state, emitter),
            SwitchBlockChildren::DefaultStatement(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            SwitchBlockChildren::Extra(x) => x.get_php_value(state, emitter),
            SwitchBlockChildren::CaseStatement(x) => x.get_php_value(state, emitter),
            SwitchBlockChildren::DefaultStatement(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            SwitchBlockChildren::Extra(x) => x.read_from(state, emitter),
            SwitchBlockChildren::CaseStatement(x) => x.read_from(state, emitter),
            SwitchBlockChildren::DefaultStatement(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for SwitchBlockChildren {
    fn brief_desc(&self) -> String {
        match self {
            SwitchBlockChildren::Extra(x) => {
                format!("SwitchBlockChildren::extra({})", x.brief_desc())
            }
            SwitchBlockChildren::CaseStatement(x) => {
                format!("SwitchBlockChildren::case_statement({})", x.brief_desc())
            }
            SwitchBlockChildren::DefaultStatement(x) => {
                format!("SwitchBlockChildren::default_statement({})", x.brief_desc())
            }
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            SwitchBlockChildren::Extra(x) => x.as_any(),
            SwitchBlockChildren::CaseStatement(x) => x.as_any(),
            SwitchBlockChildren::DefaultStatement(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            SwitchBlockChildren::Extra(x) => x.children_any(),
            SwitchBlockChildren::CaseStatement(x) => x.children_any(),
            SwitchBlockChildren::DefaultStatement(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            SwitchBlockChildren::Extra(x) => x.range(),
            SwitchBlockChildren::CaseStatement(x) => x.range(),
            SwitchBlockChildren::DefaultStatement(x) => x.range(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SwitchBlockNode {
    pub range: Range,
    pub children: Vec<Box<SwitchBlockChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl NodeParser for SwitchBlockNode {
    fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "switch_block" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [switch_block] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: SwitchBlockChildren::parse_vec(
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

impl SwitchBlockNode {
    pub fn kind(&self) -> &'static str {
        "switch_block"
    }
}

impl NodeAccess for SwitchBlockNode {
    fn brief_desc(&self) -> String {
        "SwitchBlockNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::SwitchBlock(self)
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
