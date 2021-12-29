use crate::analysis::state::AnalysisState;
use crate::autonodes::any::AnyNodeRef;
use crate::autonodes::comment::CommentNode;
use crate::autonodes::property_promotion_parameter::PropertyPromotionParameterNode;
use crate::autonodes::simple_parameter::SimpleParameterNode;
use crate::autonodes::text_interpolation::TextInterpolationNode;
use crate::autonodes::variadic_parameter::VariadicParameterNode;
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
pub enum FormalParametersChildren {
    PropertyPromotionParameter(Box<PropertyPromotionParameterNode>),
    SimpleParameter(Box<SimpleParameterNode>),
    VariadicParameter(Box<VariadicParameterNode>),
    Comment(Box<CommentNode>),
    TextInterpolation(Box<TextInterpolationNode>),
    Error(Box<ErrorNode>),
}

impl FormalParametersChildren {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        Ok(match node.kind() {
            "comment" => {
                FormalParametersChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => FormalParametersChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => FormalParametersChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
            "property_promotion_parameter" => FormalParametersChildren::PropertyPromotionParameter(
                Box::new(PropertyPromotionParameterNode::parse(node, source)?),
            ),
            "simple_parameter" => FormalParametersChildren::SimpleParameter(Box::new(
                SimpleParameterNode::parse(node, source)?,
            )),
            "variadic_parameter" => FormalParametersChildren::VariadicParameter(Box::new(
                VariadicParameterNode::parse(node, source)?,
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
            "comment" => {
                FormalParametersChildren::Comment(Box::new(CommentNode::parse(node, source)?))
            }
            "text_interpolation" => FormalParametersChildren::TextInterpolation(Box::new(
                TextInterpolationNode::parse(node, source)?,
            )),
            "ERROR" => FormalParametersChildren::Error(Box::new(ErrorNode::parse(node, source)?)),
            "property_promotion_parameter" => FormalParametersChildren::PropertyPromotionParameter(
                Box::new(PropertyPromotionParameterNode::parse(node, source)?),
            ),
            "simple_parameter" => FormalParametersChildren::SimpleParameter(Box::new(
                SimpleParameterNode::parse(node, source)?,
            )),
            "variadic_parameter" => FormalParametersChildren::VariadicParameter(Box::new(
                VariadicParameterNode::parse(node, source)?,
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
            FormalParametersChildren::Comment(x) => x.get_utype(state, emitter),
            FormalParametersChildren::TextInterpolation(x) => x.get_utype(state, emitter),
            FormalParametersChildren::Error(x) => x.get_utype(state, emitter),
            FormalParametersChildren::PropertyPromotionParameter(x) => x.get_utype(state, emitter),
            FormalParametersChildren::SimpleParameter(x) => x.get_utype(state, emitter),
            FormalParametersChildren::VariadicParameter(x) => x.get_utype(state, emitter),
        }
    }

    pub fn get_php_value(
        &self,
        state: &mut AnalysisState,
        emitter: &dyn IssueEmitter,
    ) -> Option<PHPValue> {
        match self {
            FormalParametersChildren::Comment(x) => x.get_php_value(state, emitter),
            FormalParametersChildren::TextInterpolation(x) => x.get_php_value(state, emitter),
            FormalParametersChildren::Error(x) => x.get_php_value(state, emitter),
            FormalParametersChildren::PropertyPromotionParameter(x) => {
                x.get_php_value(state, emitter)
            }
            FormalParametersChildren::SimpleParameter(x) => x.get_php_value(state, emitter),
            FormalParametersChildren::VariadicParameter(x) => x.get_php_value(state, emitter),
        }
    }

    pub fn read_from(&self, state: &mut AnalysisState, emitter: &dyn IssueEmitter) {
        match self {
            FormalParametersChildren::Comment(x) => x.read_from(state, emitter),
            FormalParametersChildren::TextInterpolation(x) => x.read_from(state, emitter),
            FormalParametersChildren::Error(x) => x.read_from(state, emitter),
            FormalParametersChildren::PropertyPromotionParameter(x) => x.read_from(state, emitter),
            FormalParametersChildren::SimpleParameter(x) => x.read_from(state, emitter),
            FormalParametersChildren::VariadicParameter(x) => x.read_from(state, emitter),
        }
    }
}

impl NodeAccess for FormalParametersChildren {
    fn brief_desc(&self) -> String {
        match self {
            FormalParametersChildren::Comment(x) => {
                format!("FormalParametersChildren::comment({})", x.brief_desc())
            }
            FormalParametersChildren::TextInterpolation(x) => format!(
                "FormalParametersChildren::text_interpolation({})",
                x.brief_desc()
            ),
            FormalParametersChildren::Error(x) => {
                format!("FormalParametersChildren::ERROR({})", x.brief_desc())
            }
            FormalParametersChildren::PropertyPromotionParameter(x) => format!(
                "FormalParametersChildren::property_promotion_parameter({})",
                x.brief_desc()
            ),
            FormalParametersChildren::SimpleParameter(x) => format!(
                "FormalParametersChildren::simple_parameter({})",
                x.brief_desc()
            ),
            FormalParametersChildren::VariadicParameter(x) => format!(
                "FormalParametersChildren::variadic_parameter({})",
                x.brief_desc()
            ),
        }
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        match self {
            FormalParametersChildren::Comment(x) => x.as_any(),
            FormalParametersChildren::TextInterpolation(x) => x.as_any(),
            FormalParametersChildren::Error(x) => x.as_any(),
            FormalParametersChildren::PropertyPromotionParameter(x) => x.as_any(),
            FormalParametersChildren::SimpleParameter(x) => x.as_any(),
            FormalParametersChildren::VariadicParameter(x) => x.as_any(),
        }
    }

    fn children_any<'a>(&'a self) -> Vec<AnyNodeRef<'a>> {
        match self {
            FormalParametersChildren::Comment(x) => x.children_any(),
            FormalParametersChildren::TextInterpolation(x) => x.children_any(),
            FormalParametersChildren::Error(x) => x.children_any(),
            FormalParametersChildren::PropertyPromotionParameter(x) => x.children_any(),
            FormalParametersChildren::SimpleParameter(x) => x.children_any(),
            FormalParametersChildren::VariadicParameter(x) => x.children_any(),
        }
    }

    fn range(&self) -> Range {
        match self {
            FormalParametersChildren::Comment(x) => x.range(),
            FormalParametersChildren::TextInterpolation(x) => x.range(),
            FormalParametersChildren::Error(x) => x.range(),
            FormalParametersChildren::PropertyPromotionParameter(x) => x.range(),
            FormalParametersChildren::SimpleParameter(x) => x.range(),
            FormalParametersChildren::VariadicParameter(x) => x.range(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct FormalParametersNode {
    pub range: Range,
    pub children: Vec<Box<FormalParametersChildren>>,
    pub extras: Vec<Box<ExtraChild>>,
}

impl FormalParametersNode {
    pub fn parse(node: Node, source: &Vec<u8>) -> Result<Self, ParseError> {
        let range = node.range();
        if node.kind() != "formal_parameters" {
            return Err(ParseError::new(
                range,
                format!(
                    "Node is of the wrong kind [{}] vs expected [formal_parameters] on pos {}:{}",
                    node.kind(),
                    range.start_point.row + 1,
                    range.start_point.column
                ),
            ));
        }

        Ok(Self {
            range,
            children: FormalParametersChildren::parse_vec(
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
        "formal_parameters"
    }
}

impl NodeAccess for FormalParametersNode {
    fn brief_desc(&self) -> String {
        "FormalParametersNode".into()
    }

    fn as_any<'a>(&'a self) -> AnyNodeRef<'a> {
        AnyNodeRef::FormalParameters(self)
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
