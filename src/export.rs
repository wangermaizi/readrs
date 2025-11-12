// src/export.rs
use gpui::*;
use std::path::Path;
use tokio::fs;
use anyhow::Result;
use pulldown_cmark::{Parser, Options, html};

pub struct Exporter;

impl Exporter {
    pub async fn export_to_html(markdown_content: &str, output_path: &str) -> Result<()> {
        // 转换Markdown为HTML
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);
        
        let parser = Parser::new_ext(markdown_content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        
        // 创建完整的HTML文档
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

    pub async fn export_to_pdf(markdown_content: &str, output_path: &str) -> Result<()> {
        // PDF导出需要额外的库支持，这里提供一个框架
        // 实际实现中可能需要使用如 printpdf 或其他PDF库
        todo!("PDF导出功能需要额外的库支持")
    }

    pub async fn export_to_docx(markdown_content: &str, output_path: &str) -> Result<()> {
        // DOCX导出需要额外的库支持，这里提供一个框架
        // 实际实现中可能需要使用如 docx-rs 库
        todo!("DOCX导出功能需要额外的库支持")
    }

    pub async fn export_to_image(markdown_content: &str, output_path: &str) -> Result<()> {
        // 图片导出需要额外的库支持，这里提供一个框架
        // 实际实现中可能需要使用如 image 或其他图形库
        todo!("图片导出功能需要额外的库支持")
    }
}

// 导出对话框组件
pub struct ExportDialog {
    markdown_content: SharedString,
}

impl ExportDialog {
    pub fn new(markdown_content: SharedString) -> Self {
        Self { markdown_content }
    }

    pub fn show_export_dialog(&self) -> Option<ExportFormat> {
        // 这里应该显示原生导出对话框
        // 暂时返回一个默认格式
        Some(ExportFormat::Html)
    }
}

#[derive(Debug, Clone)]
pub enum ExportFormat {
    Html,
    Pdf,
    Docx,
    Image,
    Epub,
    Latex,
    Rtf,
}

impl ExportFormat {
    pub fn file_extension(&self) -> &str {
        match self {
            ExportFormat::Html => "html",
            ExportFormat::Pdf => "pdf",
            ExportFormat::Docx => "docx",
            ExportFormat::Image => "png",
            ExportFormat::Epub => "epub",
            ExportFormat::Latex => "tex",
            ExportFormat::Rtf => "rtf",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            ExportFormat::Html => "HTML",
            ExportFormat::Pdf => "PDF",
            ExportFormat::Docx => "Word Document",
            ExportFormat::Image => "Image",
            ExportFormat::Epub => "EPUB",
            ExportFormat::Latex => "LaTeX",
            ExportFormat::Rtf => "RTF",
        }
    }
}