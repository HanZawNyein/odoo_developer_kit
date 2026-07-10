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
        ("Docker", Platform::MacOs) => "brew install --cask docker",
        ("Docker", Platform::Linux) => {
            "Install Docker Engine from https://docs.docker.com/engine/install/"
        }
        ("Docker", Platform::Windows) => "winget install Docker.DockerDesktop",
        ("Python", Platform::MacOs) => "brew install python@3.13",
        ("Python", Platform::Linux) => "Install python3 with your system package manager",
        ("Python", Platform::Windows) => "winget install Python.Python.3.13",
        ("PyCharm", Platform::MacOs) => "brew install --cask pycharm-ce",
        ("PyCharm", Platform::Linux) => "Install PyCharm with JetBrains Toolbox",
        ("PyCharm", Platform::Windows) => "winget install JetBrains.PyCharm.Community",
        ("VS Code", Platform::MacOs) => "brew install --cask visual-studio-code",
        ("VS Code", Platform::Linux) => "Install VS Code from https://code.visualstudio.com/",
        ("VS Code", Platform::Windows) => "winget install Microsoft.VisualStudioCode",
        ("PostgreSQL", Platform::MacOs) => {
            "Download PostgreSQL from https://www.postgresql.org/download/"
        }
        ("PostgreSQL", Platform::Linux) => {
            "Download PostgreSQL from https://www.postgresql.org/download/"
        }
        ("PostgreSQL", Platform::Windows) => {
            "Download PostgreSQL from https://www.postgresql.org/download/"
        }
        ("wkhtmltopdf", Platform::MacOs) => {
            "Download wkhtmltopdf from https://wkhtmltopdf.org/downloads.html"
        }
        ("wkhtmltopdf", Platform::Linux) => {
            "Download wkhtmltopdf from https://wkhtmltopdf.org/downloads.html"
        }
        ("wkhtmltopdf", Platform::Windows) => {
            "Download wkhtmltopdf from https://wkhtmltopdf.org/downloads.html"
        }
        _ => "Install the missing tool and rerun `odk doctor`",
    }
}
