// src/main.rs
use gpui::*;
use std::sync::Arc;

mod editor;
mod preview;
mod file_manager;
mod export;
mod theme;
mod features;

use editor::{MarkdownEditor, EditorEvent};
use preview::MarkdownPreview;
use file_manager::{FileManager, FileDialog};
use export::{Exporter, ExportDialog, ExportFormat};
use theme::{AppSettings, ThemeManager};
use features::{OutlineView, SpellChecker, ShortcutManager, SearchPanel, FileListView};

// 应用状态
#[derive(Default)]
struct AppState {
    text: SharedString,
    current_file_path: Option<String>,
    settings: AppSettings,
    active_tab: SidebarTab, // 左侧边栏当前活动标签
    view_mode: ViewMode,    // 视图模式
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SidebarTab {
    Files,
    Outline,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ViewMode {
    Preview,  // 阅读模式
    Edit,     // 编辑模式
    Dual,     // 所见即所得模式
}

// 主窗口视图
struct MainWindow {
    state: Model<AppState>,
    editor: View<MarkdownEditor>,
    preview: View<MarkdownPreview>,
    outline: View<OutlineView>,
    file_list: View<FileListView>,
    search_panel: View<SearchPanel>,
}

impl MainWindow {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        let state = cx.new_model(|_| AppState {
            active_tab: SidebarTab::Files,
            view_mode: ViewMode::Dual,
            ..Default::default()
        });
        
        let editor = cx.new_view(|cx| {
            MarkdownEditor::new(state.read(cx).text.clone(), cx)
        });
        
        let preview = cx.new_view(|cx| {
            MarkdownPreview::new(state.read(cx).text.clone(), cx)
        });
        
        let outline = cx.new_view(|_| {
            OutlineView::new()
        });
        
        let file_list = cx.new_view(|_| {
            FileListView::new()
        });
        
        let search_panel = cx.new_view(|_| {
            SearchPanel::new()
        });
        
        // 订阅编辑器事件以更新预览
        cx.subscribe(&editor, |this, _, event: &EditorEvent, cx| {
            this.state.update(cx, |s, _| {
                s.text = event.text.clone();
            });
            // 更新大纲视图
            this.outline.update(cx, |outline, _| {
                outline.update_from_markdown(&event.text);
            });
            cx.notify();
        }).detach();
        
        Self { 
            state,
            editor,
            preview,
            outline,
            file_list,
            search_panel,
        }
    }

    fn handle_new_file(&mut self, cx: &mut ViewContext<Self>) {
        cx.spawn(|this, mut cx| async move {
            let new_content = FileManager::new_file().await.unwrap_or_default();
            let shared_string = new_content.into();
            
            this.update(&mut cx, |this, cx| {
                this.state.update(cx, |state, _| {
                    state.text = shared_string;
                    state.current_file_path = None;
                });
                cx.emit(EditorEvent {
                    text: this.state.read(cx).text.clone(),
                });
            }).ok();
        }).detach();
    }

    fn handle_open_file(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(file_path) = FileDialog::show_open_dialog() {
            cx.spawn(|this, mut cx| async move {
                if let Ok(content) = FileManager::open_file(&file_path).await {
                    let shared_string = content.into();
                    
                    this.update(&mut cx, |this, cx| {
                        this.state.update(cx, |state, _| {
                            state.text = shared_string;
                            state.current_file_path = Some(file_path.clone());
                        });
                        // 添加到最近文件列表
                        this.file_list.update(cx, |file_list, _| {
                            file_list.add_recent_file(file_path);
                        });
                        cx.emit(EditorEvent {
                            text: this.state.read(cx).text.clone(),
                        });
                    }).ok();
                }
            }).detach();
        }
    }

    fn handle_save_file(&mut self, cx: &mut ViewContext<Self>) {
        let current_path = self.state.read(cx).current_file_path.clone();
        
        if let Some(file_path) = current_path {
            let text = self.state.read(cx).text.clone();
            cx.spawn(|_, _| async move {
                let _ = FileManager::save_file(&file_path, &text).await;
            }).detach();
        } else {
            self.handle_save_as_file(cx);
        }
    }

    fn handle_save_as_file(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(file_path) = FileDialog::show_save_dialog() {
            let text = self.state.read(cx).text.clone();
            cx.spawn(|this, mut cx| async move {
                if FileManager::save_file_as(&file_path, &text).await.is_ok() {
                    this.update(&mut cx, |this, cx| {
                        this.state.update(cx, |state, _| {
                            state.current_file_path = Some(file_path.clone());
                        });
                        // 添加到最近文件列表
                        this.file_list.update(cx, |file_list, _| {
                            file_list.add_recent_file(file_path);
                        });
                    }).ok();
                }
            }).detach();
        }
    }

    fn handle_export_file(&mut self, cx: &mut ViewContext<Self>) {
        let text = self.state.read(cx).text.clone();
        let export_dialog = ExportDialog::new(text.clone());
        
        if let Some(format) = export_dialog.show_export_dialog() {
            cx.spawn(|_, _| async move {
                match format {
                    ExportFormat::Html => {
                        let _ = Exporter::export_to_html(&text, "output.html").await;
                    }
                    ExportFormat::Pdf => {
                        let _ = Exporter::export_to_pdf(&text, "output.pdf").await;
                    }
                    ExportFormat::Docx => {
                        let _ = Exporter::export_to_docx(&text, "output.docx").await;
                    }
                    ExportFormat::Image => {
                        let _ = Exporter::export_to_image(&text, "output.png").await;
                    }
                    _ => {
                        // 其他格式处理
                    }
                }
            }).detach();
        }
    }

    fn handle_toggle_theme(&mut self, cx: &mut ViewContext<Self>) {
        self.state.update(cx, |state, _| {
            state.settings.theme_manager.next_theme();
        });
        cx.notify();
    }

    fn handle_toggle_search(&mut self, cx: &mut ViewContext<Self>) {
        self.search_panel.update(cx, |search, _| {
            search.toggle_visibility();
        });
        cx.notify();
    }

    fn switch_sidebar_tab(&mut self, tab: SidebarTab, cx: &mut ViewContext<Self>) {
        self.state.update(cx, |state, _| {
            state.active_tab = tab;
        });
        cx.notify();
    }

    fn switch_view_mode(&mut self, mode: ViewMode, cx: &mut ViewContext<Self>) {
        self.state.update(cx, |state, _| {
            state.view_mode = mode;
        });
        cx.notify();
    }
}

impl Render for MainWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = self.state.read(cx).settings.theme_manager.current_theme();
        let active_tab = self.state.read(cx).active_tab;
        let view_mode = self.state.read(cx).view_mode;
        
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.background)
            .text_color(theme.text)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .py_2()
                    .bg(theme.primary)
                    .text_color(theme.text)
                    .child(Label::new("ReadRS - Markdown Editor"))
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(Button::new("new", "New").on_click({
                                let view = cx.view().clone();
                                move |_, cx| {
                                    view.update(cx, |this, cx| {
                                        this.handle_new_file(cx);
                                    });
                                }
                            }))
                            .child(Button::new("open", "Open").on_click({
                                let view = cx.view().clone();
                                move |_, cx| {
                                    view.update(cx, |this, cx| {
                                        this.handle_open_file(cx);
                                    });
                                }
                            }))
                            .child(Button::new("save", "Save").on_click({
                                let view = cx.view().clone();
                                move |_, cx| {
                                    view.update(cx, |this, cx| {
                                        this.handle_save_file(cx);
                                    });
                                }
                            }))
                            .child(Button::new("export", "Export").on_click({
                                let view = cx.view().clone();
                                move |_, cx| {
                                    view.update(cx, |this, cx| {
                                        this.handle_export_file(cx);
                                    });
                                }
                            }))
                            .child(Button::new("theme", "Toggle Theme").on_click({
                                let view = cx.view().clone();
                                move |_, cx| {
                                    view.update(cx, |this, cx| {
                                        this.handle_toggle_theme(cx);
                                    });
                                }
                            }))
                            .child(Button::new("search", "Search").on_click({
                                let view = cx.view().clone();
                                move |_, cx| {
                                    view.update(cx, |this, cx| {
                                        this.handle_toggle_search(cx);
                                    });
                                }
                            }))
                            .child(
                                div()
                                    .flex()
                                    .gap_1()
                                    .child(
                                        Button::new("preview_mode", "阅读模式")
                                            .style(ButtonStyle::Subtle)
                                            .selected(view_mode == ViewMode::Preview)
                                            .on_click({
                                                let view = cx.view().clone();
                                                move |_, cx| {
                                                    view.update(cx, |this, cx| {
                                                        this.switch_view_mode(ViewMode::Preview, cx);
                                                    });
                                                }
                                            })
                                    )
                                    .child(
                                        Button::new("edit_mode", "编辑模式")
                                            .style(ButtonStyle::Subtle)
                                            .selected(view_mode == ViewMode::Edit)
                                            .on_click({
                                                let view = cx.view().clone();
                                                move |_, cx| {
                                                    view.update(cx, |this, cx| {
                                                        this.switch_view_mode(ViewMode::Edit, cx);
                                                    });
                                                }
                                            })
                                    )
                                    .child(
                                        Button::new("dual_mode", "所见即所得")
                                            .style(ButtonStyle::Subtle)
                                            .selected(view_mode == ViewMode::Dual)
                                            .on_click({
                                                let view = cx.view().clone();
                                                move |_, cx| {
                                                    view.update(cx, |this, cx| {
                                                        this.switch_view_mode(ViewMode::Dual, cx);
                                                    });
                                                }
                                            })
                                    )
                            )
                    )
            )
            .child(
                div()
                    .flex()
                    .size_full()
                    .child(
                        div()
                            .w_64()
                            .h_full()
                            .border_r_1()
                            .border_color(theme.border)
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .flex()
                                    .child(
                                        Button::new("files_tab", "文件")
                                            .style(ButtonStyle::Subtle)
                                            .selected(active_tab == SidebarTab::Files)
                                            .flex_1()
                                            .on_click({
                                                let view = cx.view().clone();
                                                move |_, cx| {
                                                    view.update(cx, |this, cx| {
                                                        this.switch_sidebar_tab(SidebarTab::Files, cx);
                                                    });
                                                }
                                            })
                                    )
                                    .child(
                                        Button::new("outline_tab", "大纲")
                                            .style(ButtonStyle::Subtle)
                                            .selected(active_tab == SidebarTab::Outline)
                                            .flex_1()
                                            .on_click({
                                                let view = cx.view().clone();
                                                move |_, cx| {
                                                    view.update(cx, |this, cx| {
                                                        this.switch_sidebar_tab(SidebarTab::Outline, cx);
                                                    });
                                                }
                                            })
                                    )
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .overflow_y_scroll()
                                    .when(active_tab == SidebarTab::Files, |this| {
                                        this.child(self.file_list.clone())
                                    })
                                    .when(active_tab == SidebarTab::Outline, |this| {
                                        this.child(self.outline.clone())
                                    })
                            )
                    )
                    .child(
                        div()
                            .flex_1()
                            .h_full()
                            .child(
                                div()
                                    .flex()
                                    .size_full()
                                    .when(view_mode == ViewMode::Edit, |this| {
                                        this.child(
                                            div()
                                                .w_full()
                                                .h_full()
                                                .child(Label::new("Editor"))
                                                .child(self.editor.clone())
                                        )
                                    })
                                    .when(view_mode == ViewMode::Preview, |this| {
                                        this.child(
                                            div()
                                                .w_full()
                                                .h_full()
                                                .child(Label::new("Preview"))
                                                .child(self.preview.clone())
                                        )
                                    })
                                    .when(view_mode == ViewMode::Dual, |this| {
                                        this.child(
                                            div()
                                                .w_1_2()
                                                .h_full()
                                                .border_r_1()
                                                .border_color(theme.border)
                                                .child(Label::new("Editor"))
                                                .child(self.editor.clone())
                                        )
                                        .child(
                                            div()
                                                .w_1_2()
                                                .h_full()
                                                .child(Label::new("Preview"))
                                                .child(self.preview.clone())
                                        )
                                    })
                            )
                    )
            )
            .child(self.search_panel.clone())
    }
}

fn main() {
    App::new()
        .run(|cx: &mut AppContext| {
            cx.open_window(
                WindowOptions {
                    title: "ReadRS".into(),
                    ..Default::default()
                },
                |cx| cx.new_view(|cx| MainWindow::new(cx)),
            );
        });
}