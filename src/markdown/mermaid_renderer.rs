//! Mermaid 流程图渲染模块
//!
//! Mermaid 图表渲染器（占位实现）

/// Mermaid 图表类型
#[derive(Debug, Clone, Copy)]
pub enum DiagramType {
    Flowchart,
    SequenceDiagram,
    ClassDiagram,
    StateDiagram,
    Gantt,
}

/// Mermaid 图表渲染器
pub struct MermaidRenderer;

impl MermaidRenderer {
    /// 渲染 Mermaid 图表为 SVG（占位实现）
    pub fn render(mermaid: &str, _diagram_type: DiagramType) -> String {
        // 占位实现，实际渲染需要集成 mermaid 库
        format!(
            "<div style=\"background: #f5f5f5; padding: 20px; border: 1px dashed #999;\">Mermaid 图表占位符<br><pre>{}</pre></div>",
            html_escape(mermaid)
        )
    }

    /// 渲染流程图
    pub fn render_flowchart(mermaid: &str) -> String {
        Self::render(mermaid, DiagramType::Flowchart)
    }

    /// 渲染时序图
    pub fn render_sequence(mermaid: &str) -> String {
        Self::render(mermaid, DiagramType::SequenceDiagram)
    }

    /// 渲染类图
    pub fn render_class(mermaid: &str) -> String {
        Self::render(mermaid, DiagramType::ClassDiagram)
    }

    /// 渲染状态图
    pub fn render_state(mermaid: &str) -> String {
        Self::render(mermaid, DiagramType::StateDiagram)
    }

    /// 渲染甘特图
    pub fn render_gantt(mermaid: &str) -> String {
        Self::render(mermaid, DiagramType::Gantt)
    }

    /// 检查是否包含 Mermaid 图表
    pub fn contains_mermaid(text: &str) -> bool {
        text.contains("```mermaid") || text.contains("```graph")
    }

    /// 提取 Mermaid 图表定义
    pub fn extract_mermaid(text: &str) -> Vec<(String, DiagramType)> {
        let mut diagrams = Vec::new();
        let lines: Vec<&str> = text.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            
            if line == "```mermaid" || line == "```graph" {
                let mut diagram = String::new();
                let mut diagram_type = DiagramType::Flowchart;
                
                if line == "```mermaid" {
                    i += 1;
                    // 尝试从下一行获取图表类型
                    if i < lines.len() {
                        let type_line = lines[i].trim();
                        diagram_type = match type_line {
                            "graph TD" | "graph LR" => DiagramType::Flowchart,
                            "sequenceDiagram" => DiagramType::SequenceDiagram,
                            "classDiagram" => DiagramType::ClassDiagram,
                            "stateDiagram" => DiagramType::StateDiagram,
                            "gantt" => DiagramType::Gantt,
                            _ => DiagramType::Flowchart,
                        };
                        diagram.push_str(type_line);
                        diagram.push('\n');
                    }
                } else {
                    diagram_type = DiagramType::Flowchart;
                    diagram.push_str(&line[3..]); // 移除 ```
                    diagram.push('\n');
                }
                
                i += 1;
                
                // 收集图表内容
                while i < lines.len() && lines[i].trim() != "```" {
                    diagram.push_str(lines[i]);
                    diagram.push('\n');
                    i += 1;
                }
                
                diagrams.push((diagram, diagram_type));
            }
            
            i += 1;
        }
        
        diagrams
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
    fn test_render_flowchart() {
        let mermaid = r#"
graph TD
    A[Start] --> B{Is it?}
    B -->|Yes| C[OK]
    B -->|No| D[Not OK]
"#;
        let result = MermaidRenderer::render_flowchart(mermaid);
        assert!(result.contains("Mermaid 图表占位符"));
    }

    #[test]
    fn test_contains_mermaid() {
        assert!(MermaidRenderer::contains_mermaid("```mermaid\ngraph TD\n```"));
        assert!(MermaidRenderer::contains_mermaid("```graph\nA-->B\n```"));
        assert!(!MermaidRenderer::contains_mermaid("no mermaid here"));
    }

    #[test]
    fn test_extract_mermaid() {
        let text = r#"
# Document

```mermaid
graph TD
    A --> B
```

Some text

```mermaid
sequenceDiagram
    A->>B: Message
```
"#;
        let diagrams = MermaidRenderer::extract_mermaid(text);
        assert_eq!(diagrams.len(), 2);
        assert!(matches!(diagrams[0].1, DiagramType::Flowchart));
        assert!(matches!(diagrams[1].1, DiagramType::SequenceDiagram));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_flowchart() {
        let mermaid = r#"
graph TD
    A[Start] --> B{Is it?}
    B -->|Yes| C[OK]
    B -->|No| D[Not OK]
"#;
        let result = MermaidRenderer::render_flowchart(mermaid);
        assert!(result.contains("<svg"));
    }

    #[test]
    fn test_contains_mermaid() {
        assert!(MermaidRenderer::contains_mermaid("```mermaid\ngraph TD\n```"));
        assert!(MermaidRenderer::contains_mermaid("```graph\nA-->B\n```"));
        assert!(!MermaidRenderer::contains_mermaid("no mermaid here"));
    }

    #[test]
    fn test_extract_mermaid() {
        let text = r#"
# Document

```mermaid
graph TD
    A --> B
```

Some text

```mermaid
sequenceDiagram
    A->>B: Message
```
"#;
        let diagrams = MermaidRenderer::extract_mermaid(text);
        assert_eq!(diagrams.len(), 2);
        assert!(matches!(diagrams[0].1, DiagramType::Flowchart));
        assert!(matches!(diagrams[1].1, DiagramType::SequenceDiagram));
    }
}
