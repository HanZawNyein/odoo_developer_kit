#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    MacOs,
    Linux,
    Windows,
    Unknown,
}

impl Platform {
    pub fn detect() -> Self {
        if cfg!(target_os = "macos") {
            Self::MacOs
        } else if cfg!(target_os = "linux") {
            Self::Linux
        } else if cfg!(target_os = "windows") {
            Self::Windows
        } else {
            Self::Unknown
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::MacOs => "macOS",
            Self::Linux => "Linux",
            Self::Windows => "Windows",
            Self::Unknown => "Unknown",
        }
    }
}

pub fn install_suggestion(tool: &str, platform: Platform) -> &'static str {
    match (tool, platform) {
        ("uv", Platform::MacOs) => "brew install uv",
        ("uv", Platform::Linux) => "curl -LsSf https://astral.sh/uv/install.sh | sh",
        ("uv", Platform::Windows) => {
            "powershell -ExecutionPolicy ByPass -c \"irm https://astral.sh/uv/install.ps1 | iex\""
        }
        ("Python", Platform::MacOs) => "brew install python@3.13",
        ("Python", Platform::Linux) => "Install python3 with your system package manager",
        ("Python", Platform::Windows) => "winget install Python.Python.3.13",
        ("PyCharm", Platform::MacOs) => "brew install --cask pycharm-ce",
        ("PyCharm", Platform::Linux) => "Install PyCharm with JetBrains Toolbox",
        ("PyCharm", Platform::Windows) => "winget install JetBrains.PyCharm.Community",
        ("VS Code", Platform::MacOs) => "brew install --cask visual-studio-code",
        ("VS Code", Platform::Linux) => "Install VS Code from https://code.visualstudio.com/",
        ("VS Code", Platform::Windows) => "winget install Microsoft.VisualStudioCode",
        ("PostgreSQL", Platform::MacOs) => "brew install postgresql@17",
        ("PostgreSQL", Platform::Linux) => "sudo apt install postgresql postgresql-client",
        ("PostgreSQL", Platform::Windows) => "winget install PostgreSQL.PostgreSQL",
        ("wkhtmltopdf", Platform::MacOs) => "brew install wkhtmltopdf",
        ("wkhtmltopdf", Platform::Linux) => "sudo apt install wkhtmltopdf",
        ("wkhtmltopdf", Platform::Windows) => "winget install wkhtmltopdf",
        _ => "Install the missing tool and rerun `odk doctor`",
    }
}
