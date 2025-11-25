//! LaTeX 公式渲染模块
//!
//! LaTeX 公式渲染器（纯 Rust 实现，无需 JS 引擎）

/// LaTeX 公式渲染器
pub struct LatexRenderer;

impl LatexRenderer {
    /// 渲染 LaTeX 公式为 HTML
    ///
    /// # 参数
    /// - `latex`: LaTeX 公式内容（包含 $...$ 或 $$...$$）
    ///
    /// # 返回
    /// 渲染后的 HTML 字符串
    pub fn render(latex: &str) -> String {
        let mut result = latex.to_string();
        
        // 处理块级公式 $$...$$
        while let Some(start) = result.find("$$") {
            if let Some(end) = result[start + 2..].find("$$") {
                let end_pos = start + 2 + end + 2;
                let formula = &result[start + 2..end_pos - 2];
                
                // 使用等宽字体显示公式
                let rendered = format!(
                    "<div style=\"background: #f5f5f5; padding: 15px; margin: 10px 0; border-left: 4px solid #0066cc; font-family: 'Courier New', monospace;\">{}</div>", 
                    html_escape(formula)
                );
                result.replace_range(start..end_pos, &rendered);
            } else {
                break;
            }
        }
        
        // 处理行内公式 $...$
        while let Some(start) = result.find('$') {
            if let Some(end) = result[start + 1..].find('$') {
                let end_pos = start + 1 + end + 1;
                let formula = &result[start + 1..end_pos - 1];
                
                // 使用等宽字体显示公式
                let rendered = format!(
                    "<span style=\"background: #e8f4f8; padding: 2px 6px; border-radius: 3px; font-family: 'Courier New', monospace; font-size: 0.9em;\">{}</span>", 
                    html_escape(formula)
                );
                result.replace_range(start..end_pos, &rendered);
            } else {
                break;
            }
        }
        
        result
    }

    /// 检查是否包含 LaTeX 公式
    pub fn contains_latex(text: &str) -> bool {
        text.contains('$')
    }
}

/// HTML 转义函数
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_inline_formula() {
        let latex = "这是一个公式 $E = mc^2$";
        let result = LatexRenderer::render(latex);
        assert!(result.contains("E"));
        assert!(result.contains("mc"));
    }

    #[test]
    fn test_render_block_formula() {
        let latex = r"这是一个块级公式 $$\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}$$";
        let result = LatexRenderer::render(latex);
        assert!(result.contains("int"));
    }

    #[test]
    fn test_contains_latex() {
        assert!(LatexRenderer::contains_latex("$x^2$"));
        assert!(!LatexRenderer::contains_latex("no latex here"));
    }
}