#[derive(Clone, Debug, Copy)]
pub struct PHPDocConfig {
    pub known_tags: &'static [&'static str],
}

#[derive(Clone, Debug, Copy, Default)]
pub struct PHPConfigVersionRequirements {}

impl PHPConfigVersionRequirements {
    pub fn is_less_than(&self, major: u8, minor: u8, patchlevel: u8) -> bool {
        true
    }
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
    pub php_version: PHPConfigVersionRequirements,
}
