use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::by_ref::ByRefNode;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::variable_name::VariableNameNode;
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
pub enum AnonymousFunctionUseClauseChildren {
    ByRef(Box<ByRefNode>),
    VariableName(Box<VariableNameNode>),
    Extra(ExtraChild),
}

impl AnonymousFunctionUseClauseChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => AnonymousFunctionUseClauseChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                AnonymousFunctionUseClauseChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => AnonymousFunctionUseClauseChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "by_ref" => {
                AnonymousFunctionUseClauseChildren::ByRef(Box::new(ByRefNode::parse(node, source)?))
            }
            "variable_name" => AnonymousFunctionUseClauseChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
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
            "comment" => AnonymousFunctionUseClauseChildren::Extra(ExtraChild::Comment(Box::new(
                CommentNode::parse(node, source)?,
            ))),
            "text_interpolation" => {
                AnonymousFunctionUseClauseChildren::Extra(ExtraChild::TextInterpolation(Box::new(
                    TextInterpolationNode::parse(node, source)?,
                )))
            }
            "ERROR" => AnonymousFunctionUseClauseChildren::Extra(ExtraChild::Error(Box::new(
                ErrorNode::parse(node, source)?,
            ))),
            "by_ref" => {
                AnonymousFunctionUseClauseChildren::ByRef(Box::new(ByRefNode::parse(node, source)?))
            }
            "variable_name" => AnonymousFunctionUseClauseChildren::VariableName(Box::new(
                VariableNameNode::parse(node, source)?,
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
            AnonymousFunctionUseClauseChildren::Extra(x) => x.get_utype(state, emitter),
            AnonymousFunctionUseClauseChildren::ByRef(x) => x.get_utype(state, emitter),
            AnonymousFunctionUseClauseChildren::VariableName(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            AnonymousFunctionUseClauseChildren::Extra(x) => x.get_php_value(state, emitter),
            AnonymousFunctionUseClauseChildren::ByRef(x) => x.get_php_value(state, emitter),
            AnonymousFunctionUseClauseChildren::VariableName(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            AnonymousFunctionUseClauseChildren::Extra(x) => x.read_from(state, emitter),
            AnonymousFunctionUseClauseChildren::ByRef(x) => x.read_from(state, emitter),
            AnonymousFunctionUseClauseChildren::VariableName(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for AnonymousFunctionUseClauseChildren {
    fn brief_desc(&self) -> String {
        match self {
            AnonymousFunctionUseClauseChildren::Extra(x) => format!(
                "AnonymousFunctionUseClauseChildren::extra({})",
                x.brief_desc()
            ),
            AnonymousFunctionUseClauseChildren::ByRef(x) => format!(
                "AnonymousFunctionUseClauseChildren::by_ref({})",
                x.brief_desc()
            ),
            AnonymousFunctionUseClauseChildren::VariableName(x) => format!(
                "AnonymousFunctionUseClauseChildren::variable_name({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            AnonymousFunctionUseClauseChildren::Extra(x) => x.as_any(),
            AnonymousFunctionUseClauseChildren::ByRef(x) => x.as_any(),
            AnonymousFunctionUseClauseChildren::VariableName(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            AnonymousFunctionUseClauseChildren::Extra(x) => x.children_any(),
            AnonymousFunctionUseClauseChildren::ByRef(x) => x.children_any(),
            AnonymousFunctionUseClauseChildren::VariableName(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            AnonymousFunctionUseClauseChildren::Extra(x) => x.range(),
            AnonymousFunctionUseClauseChildren::ByRef(x) => x.range(),
            AnonymousFunctionUseClauseChildren::VariableName(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AnonymousFunctionUseClauseNode {
    pub range: Range,
    pub children: Vec<Box<AnonymousFunctionUseClauseChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl AnonymousFunctionUseClauseNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "anonymous_function_use_clause" {
            return Err(ParseError::new(range, format!("Node is of the wrong kind [{}] vs expected [anonymous_function_use_clause] on pos {}:{}", node.kind(), range.start_point.row+1, range.start_point.column)));
        }

        Ok(Self {
            range,
            children: AnonymousFunctionUseClauseChildren::parse_vec(
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
        "anonymous_function_use_clause"
    }
}

impl NodeAccess for AnonymousFunctionUseClauseNode {
    fn brief_desc(&self) -> String {
        "AnonymousFunctionUseClauseNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::AnonymousFunctionUseClause(self)
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
