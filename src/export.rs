// src/export.rs
use anyhow::Result;
use pulldown_cmark::{Parser, Options, html};
use tokio::fs;

pub struct Exporter;

impl Exporter {
    pub async fn export_to_html(markdown_content: &str, output_path: &str) -> Result<()> {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);
        
        let parser = Parser::new_ext(markdown_content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        
        let full_html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>ReadRS Export</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
        }}
        h1, h2, h3, h4, h5, h6 {{
            margin-top: 24px;
            margin-bottom: 16px;
        }}
        code {{
            background-color: #f6f8fa;
            padding: 0.2em 0.4em;
            border-radius: 3px;
            font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
        }}
        pre {{
            background-color: #f6f8fa;
            padding: 16px;
            overflow: auto;
            border-radius: 3px;
        }}
        blockquote {{
            padding: 0 1em;
            color: #6a737d;
            border-left: 0.25em solid #dfe2e5;
        }}
        table {{
            border-collapse: collapse;
            width: 100%;
        }}
        table th, table td {{
            border: 1px solid #dfe2e5;
            padding: 6px 13px;
        }}
        table tr:nth-child(2n) {{
            background-color: #f6f8fa;
        }}
    </style>
</head>
<body>
{}
</body>
</html>"#,
            html_output
        );
        
        fs::write(output_path, full_html).await?;
        Ok(())
    }

    pub async fn export_to_pdf(_markdown_content: &str, _output_path: &str) -> Result<()> {
        todo!("PDF导出功能需要额外的库支持")
    }

    pub async fn export_to_docx(_markdown_content: &str, _output_path: &str) -> Result<()> {
        todo!("DOCX导出功能需要额外的库支持")
    }
}

#[derive(Debug, Clone)]
pub enum ExportFormat {
    Html,
    Pdf,
    Docx,
}

