#[derive(Clone, Debug, Copy)]
pub struct PHPDocConfig {
    pub known_tags: &'static [&'static str],
}

impl Default for PHPDocConfig {
    fn default() -> Self {
        Self {
            known_tags: &[
                "OpenAPI",
                "NoOpenAPI",
                "package",
                "testsuite",
                "subpackage",
                "runTestsInSeparateProcesses",
                "log",
            ],
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct PHPAnalyzeConfig {
    pub phpdoc: PHPDocConfig,
}
