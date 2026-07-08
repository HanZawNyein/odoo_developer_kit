use console::style;

fn main() {
    if let Err(error) = odoo_developer_kit::cli::run() {
        eprintln!("{} {error:#}", style("Error:").red().bold());
        std::process::exit(1);
    }
}
