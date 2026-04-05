use std::{collections::HashSet, io, net::SocketAddr, time::Duration};

use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};
use serde::Serialize;
use serde_json::Value;
use sunspec::{
    client::{AsyncClient, AsyncDevice, AsyncModbusClient, Config},
    models::model1::Model1,
    DiscoveredModel, FieldKind, GroupInfo, GroupMeta, Model,
};
use tokio_modbus::client::tcp::connect;

#[derive(Parser)]
struct Args {
    addr: SocketAddr,
    device_id: u8,
}

#[derive(Debug)]
struct App {
    device_summary: String,
    discovered_models: Vec<DiscoveredModel>,
    unknown_models: Vec<String>,
    list_state: ListState,
    screen: Screen,
    status: String,
}

#[derive(Debug)]
enum Screen {
    List,
    Detail(ModelDetail),
}

#[derive(Debug)]
struct ModelDetail {
    model: DiscoveredModel,
    summary_lines: Vec<String>,
    value_tree: Vec<TreeNode>,
    expanded: HashSet<String>,
    selected: usize,
    scroll: usize,
}

#[derive(Debug)]
struct RenderedModel {
    summary_lines: Vec<String>,
    value_tree: Vec<TreeNode>,
}

#[derive(Debug)]
struct TreeNode {
    path: String,
    label: String,
    value: TreeValue,
}

#[derive(Debug)]
enum TreeValue {
    Scalar(String),
    Branch {
        kind: BranchKind,
        children: Vec<TreeNode>,
    },
}

#[derive(Clone, Copy, Debug)]
enum BranchKind {
    Object,
    Array,
}

#[derive(Clone, Debug)]
struct TreeRow {
    path: String,
    label: String,
    depth: usize,
    value_preview: Option<String>,
    expandable: bool,
    expanded: bool,
}

impl App {
    fn new(
        device_summary: String,
        discovered_models: Vec<DiscoveredModel>,
        unknown_models: Vec<String>,
    ) -> Self {
        let mut list_state = ListState::default();
        if !discovered_models.is_empty() {
            list_state.select(Some(0));
        }
        Self {
            device_summary,
            discovered_models,
            unknown_models,
            list_state,
            screen: Screen::List,
            status: "Enter opens a model. q quits.".to_string(),
        }
    }

    fn selected_index(&self) -> Option<usize> {
        self.list_state.selected()
    }

    fn selected_model(&self) -> Option<DiscoveredModel> {
        self.selected_index()
            .and_then(|index| self.discovered_models.get(index).copied())
    }

    fn next(&mut self) {
        if self.discovered_models.is_empty() {
            return;
        }
        let next = match self.selected_index() {
            Some(index) => (index + 1) % self.discovered_models.len(),
            None => 0,
        };
        self.list_state.select(Some(next));
    }

    fn previous(&mut self) {
        if self.discovered_models.is_empty() {
            return;
        }
        let previous = match self.selected_index() {
            Some(0) | None => self.discovered_models.len() - 1,
            Some(index) => index - 1,
        };
        self.list_state.select(Some(previous));
    }

    async fn open_selected<C: AsyncModbusClient>(&mut self, device: &AsyncDevice<C>) {
        let Some(model) = self.selected_model() else {
            self.status = "No discovered models available.".to_string();
            return;
        };

        match read_rendered_model(device, model).await {
            Ok(rendered) => {
                self.screen = Screen::Detail(ModelDetail {
                    model,
                    summary_lines: rendered.summary_lines,
                    value_tree: rendered.value_tree,
                    expanded: HashSet::new(),
                    selected: 0,
                    scroll: 0,
                });
                self.status =
                    "Up/Down move. Right or Enter expands. Left collapses. Esc returns. r reloads."
                        .to_string();
            }
            Err(error) => {
                self.status = format!("Failed to read model {}: {error}", model.info.id);
            }
        }
    }

    async fn reload_current<C: AsyncModbusClient>(&mut self, device: &AsyncDevice<C>) {
        let model = match &self.screen {
            Screen::List => self.selected_model(),
            Screen::Detail(detail) => Some(detail.model),
        };

        let Some(model) = model else {
            self.status = "No discovered models available.".to_string();
            return;
        };

        match read_rendered_model(device, model).await {
            Ok(rendered) => {
                if let Screen::Detail(detail) = &mut self.screen {
                    detail.summary_lines = rendered.summary_lines;
                    detail.value_tree = rendered.value_tree;
                    detail.expanded.clear();
                    detail.selected = 0;
                    detail.scroll = 0;
                } else {
                    self.screen = Screen::Detail(ModelDetail {
                        model,
                        summary_lines: rendered.summary_lines,
                        value_tree: rendered.value_tree,
                        expanded: HashSet::new(),
                        selected: 0,
                        scroll: 0,
                    });
                }
                self.status = format!("Reloaded model {}.", model.info.id);
            }
            Err(error) => {
                self.status = format!("Failed to reload model {}: {error}", model.info.id);
            }
        }
    }

    fn back_to_list(&mut self) {
        self.screen = Screen::List;
        self.status = "Enter opens a model. q quits.".to_string();
    }

    fn scroll_down(&mut self) {
        if let Screen::Detail(detail) = &mut self.screen {
            detail.move_next();
        }
    }

    fn scroll_up(&mut self) {
        if let Screen::Detail(detail) = &mut self.screen {
            detail.move_previous();
        }
    }

    fn expand_selected(&mut self) {
        if let Screen::Detail(detail) = &mut self.screen {
            detail.expand_selected();
        }
    }

    fn collapse_selected(&mut self) {
        if let Screen::Detail(detail) = &mut self.screen {
            detail.collapse_selected();
        }
    }

    fn expand_all(&mut self) {
        if let Screen::Detail(detail) = &mut self.screen {
            detail.expand_all();
        }
    }

    fn collapse_all(&mut self) {
        if let Screen::Detail(detail) = &mut self.screen {
            detail.collapse_all();
        }
    }
}

impl ModelDetail {
    fn visible_group_rows(&self) -> Vec<TreeRow> {
        let mut rows = vec![TreeRow {
            path: String::new(),
            label: "Root".to_string(),
            depth: 0,
            value_preview: Some("group".to_string()),
            expandable: false,
            expanded: true,
        }];
        for node in &self.value_tree {
            append_group_rows(node, 1, &self.expanded, &mut rows);
        }
        rows
    }

    fn move_next(&mut self) {
        let len = self.visible_group_rows().len();
        if len == 0 {
            self.selected = 0;
            return;
        }
        self.selected = (self.selected + 1).min(len - 1);
    }

    fn move_previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    fn expand_selected(&mut self) {
        let rows = self.visible_group_rows();
        let Some(row) = rows.get(self.selected) else {
            return;
        };
        if row.expandable {
            self.expanded.insert(row.path.clone());
        }
    }

    fn collapse_selected(&mut self) {
        let rows = self.visible_group_rows();
        let Some(row) = rows.get(self.selected) else {
            return;
        };
        if row.expandable && row.expanded {
            self.expanded.remove(&row.path);
            return;
        }
        if let Some(parent) = parent_path(&row.path) {
            if let Some(index) = rows.iter().position(|candidate| candidate.path == parent) {
                self.selected = index;
            }
        }
    }

    fn sync_scroll(&mut self, viewport_height: usize) {
        let len = self.visible_group_rows().len();
        if len == 0 {
            self.selected = 0;
            self.scroll = 0;
            return;
        }
        self.selected = self.selected.min(len - 1);
        let viewport_height = viewport_height.max(1);
        let max_scroll = len.saturating_sub(viewport_height);
        if self.selected < self.scroll {
            self.scroll = self.selected;
        } else if self.selected >= self.scroll + viewport_height {
            self.scroll = self.selected + 1 - viewport_height;
        }
        self.scroll = self.scroll.min(max_scroll);
    }

    fn expand_all(&mut self) {
        self.expanded.clear();
        for node in &self.value_tree {
            collect_expandable_paths(node, &mut self.expanded);
        }
    }

    fn collapse_all(&mut self) {
        self.expanded.clear();
        self.selected = 0;
        self.scroll = 0;
    }

    fn selected_group_path(&self) -> String {
        self.visible_group_rows()
            .get(self.selected)
            .map(|row| row.path.clone())
            .unwrap_or_default()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().map_err(|error| anyhow!(error.to_string()))?;

    let args = Args::parse();
    let client = AsyncClient::new(
        connect(args.addr)
            .await
            .with_context(|| format!("failed to connect to {}", args.addr))?,
        Config::default(),
    );
    let device = client
        .device(args.device_id)
        .await
        .with_context(|| format!("failed to discover SunSpec device {}", args.device_id))?;

    let device_summary = read_device_summary(&device).await;
    let unknown_models = device
        .unknown_models
        .iter()
        .map(|model| format!("{} @ {} (len {})", model.id, model.addr, model.len))
        .collect();
    let mut app = App::new(
        device_summary,
        device.models.discovered_models(),
        unknown_models,
    );

    let mut terminal = init_terminal()?;
    let result = run_app(&mut terminal, &device, &mut app).await;
    restore_terminal(&mut terminal)?;
    result
}

async fn run_app<C: AsyncModbusClient>(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    device: &AsyncDevice<C>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, app))?;

        if !event::poll(Duration::from_millis(100))? {
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if key.kind != KeyEventKind::Press {
            continue;
        }

        match key.code {
            KeyCode::Char('q') => return Ok(()),
            KeyCode::Char('r') => app.reload_current(device).await,
            KeyCode::Up | KeyCode::Char('k') => match &app.screen {
                Screen::List => app.previous(),
                Screen::Detail(_) => app.scroll_up(),
            },
            KeyCode::Down | KeyCode::Char('j') => match &app.screen {
                Screen::List => app.next(),
                Screen::Detail(_) => app.scroll_down(),
            },
            KeyCode::PageUp => {
                for _ in 0..10 {
                    app.scroll_up();
                }
            }
            KeyCode::PageDown => {
                for _ in 0..10 {
                    app.scroll_down();
                }
            }
            KeyCode::Enter => {
                if matches!(app.screen, Screen::List) {
                    app.open_selected(device).await;
                } else {
                    app.expand_selected();
                }
            }
            KeyCode::Char('e') => app.expand_all(),
            KeyCode::Char('c') => app.collapse_all(),
            KeyCode::Right | KeyCode::Char('l') => app.expand_selected(),
            KeyCode::Left | KeyCode::Char('h') => app.collapse_selected(),
            KeyCode::Esc | KeyCode::Backspace => {
                if matches!(app.screen, Screen::Detail(_)) {
                    app.back_to_list();
                }
            }
            _ => {}
        }
    }
}

fn render(frame: &mut Frame<'_>, app: &mut App) {
    frame.render_widget(
        Block::default().style(Style::default().bg(Color::Rgb(10, 14, 22))),
        frame.area(),
    );

    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(4),
        ])
        .split(frame.area());

    let header = Paragraph::new(app.device_summary.as_str())
        .style(Style::default().fg(Color::Rgb(230, 234, 244)))
        .block(section_block("Device", Color::Cyan))
        .wrap(Wrap { trim: true });
    frame.render_widget(header, areas[0]);

    match &mut app.screen {
        Screen::List => render_list(frame, app, areas[1]),
        Screen::Detail(detail) => render_detail(frame, detail, areas[1]),
    }

    let footer = Paragraph::new(Text::from(vec![
        Line::from(Span::styled(
            app.status.as_str(),
            Style::default().fg(Color::Rgb(255, 221, 87)),
        )),
        Line::from(match &app.screen {
            Screen::List => vec![
                Span::styled("q", key_style()),
                Span::raw(" quit  "),
                Span::styled("↑/↓", key_style()),
                Span::raw(" move  "),
                Span::styled("Enter", key_style()),
                Span::raw(" open"),
            ],
            Screen::Detail(_) => vec![
                Span::styled("q", key_style()),
                Span::raw(" quit  "),
                Span::styled("Esc", key_style()),
                Span::raw(" back  "),
                Span::styled("↑/↓", key_style()),
                Span::raw(" move  "),
                Span::styled("→/Enter", key_style()),
                Span::raw(" expand  "),
                Span::styled("←", key_style()),
                Span::raw(" collapse  "),
                Span::styled("e", key_style()),
                Span::raw(" expand all  "),
                Span::styled("c", key_style()),
                Span::raw(" collapse all  "),
                Span::styled("r", key_style()),
                Span::raw(" reload"),
            ],
        }),
    ]))
    .block(section_block("Status / Keys", Color::Yellow))
    .wrap(Wrap { trim: true });
    frame.render_widget(footer, areas[2]);
}

fn render_list(frame: &mut Frame<'_>, app: &mut App, area: Rect) {
    let sections = if app.unknown_models.is_empty() {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1)])
            .split(area)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(4)])
            .split(area)
    };

    if app.discovered_models.is_empty() {
        let empty = Paragraph::new("No supported models were discovered on this device.")
            .style(Style::default().fg(Color::Rgb(220, 225, 232)))
            .block(section_block("Models", Color::Green))
            .wrap(Wrap { trim: true });
        frame.render_widget(empty, sections[0]);
    } else {
        let items = app.discovered_models.iter().map(|model| {
            let title = Line::from(vec![
                Span::styled(
                    format!("{:>5}", model.info.id),
                    Style::default()
                        .fg(Color::Rgb(255, 196, 107))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("  "),
                Span::styled(
                    model.info.label,
                    Style::default()
                        .fg(Color::Rgb(235, 239, 247))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  addr {}  len {}", model.addr, model.len),
                    Style::default().fg(Color::Rgb(123, 211, 255)),
                ),
            ]);
            let detail = if model.info.description.is_empty() {
                Line::from(Span::styled(
                    format!("      {}", model.info.name),
                    Style::default().fg(Color::Rgb(152, 161, 178)),
                ))
            } else {
                Line::from(Span::styled(
                    format!("      {}", model.info.description),
                    Style::default().fg(Color::Rgb(152, 161, 178)),
                ))
            };
            ListItem::new(vec![title, detail])
        });

        let list = List::new(items)
            .block(section_block("Models", Color::Green))
            .highlight_style(
                Style::default()
                    .bg(Color::Rgb(63, 177, 255))
                    .fg(Color::Rgb(8, 12, 20))
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▸ ");
        frame.render_stateful_widget(list, sections[0], &mut app.list_state);
    }

    if sections.len() > 1 {
        let unknown = Paragraph::new(app.unknown_models.join("\n"))
            .style(Style::default().fg(Color::Rgb(255, 187, 136)))
            .block(section_block("Unknown Models", Color::LightRed))
            .wrap(Wrap { trim: true });
        frame.render_widget(unknown, sections[1]);
    }
}

fn render_detail(frame: &mut Frame<'_>, detail: &mut ModelDetail, area: Rect) {
    frame.render_widget(Clear, area);

    let sections = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(34),
            Constraint::Length(34),
            Constraint::Min(1),
        ])
        .split(area);

    let overview = Text::from(
        detail
            .summary_lines
            .iter()
            .map(|line| style_detail_line(line))
            .collect::<Vec<_>>(),
    );
    let overview_paragraph = Paragraph::new(overview)
        .style(Style::default().fg(Color::Rgb(228, 233, 241)))
        .block(section_block("Model", Color::Magenta))
        .wrap(Wrap { trim: false });
    frame.render_widget(overview_paragraph, sections[0]);

    let groups_inner_height = sections[1].height.saturating_sub(2) as usize;
    detail.sync_scroll(groups_inner_height);
    let rows = detail.visible_group_rows();
    let groups = Text::from(
        rows.iter()
            .enumerate()
            .map(|(index, row)| style_group_row(row, index == detail.selected))
            .collect::<Vec<_>>(),
    );
    let groups_paragraph = Paragraph::new(groups)
        .style(Style::default().fg(Color::Rgb(228, 233, 241)))
        .block(section_block("Groups", Color::Blue))
        .scroll((detail.scroll as u16, 0))
        .wrap(Wrap { trim: false });
    frame.render_widget(groups_paragraph, sections[1]);

    let selected_group_path = detail.selected_group_path();
    let points = Text::from(points_for_group(&detail.value_tree, &selected_group_path));
    let points_paragraph = Paragraph::new(points)
        .style(Style::default().fg(Color::Rgb(228, 233, 241)))
        .block(section_block("Points", Color::Cyan))
        .wrap(Wrap { trim: false });
    frame.render_widget(points_paragraph, sections[2]);
}

async fn read_device_summary<C: AsyncModbusClient>(device: &AsyncDevice<C>) -> String {
    match device.read_model::<Model1>().await {
        Ok(common) => format!(
            "{} | {} | serial {} | slave {} | {} supported models",
            common.mn,
            common.md,
            common.sn,
            device.slave_id,
            device.models.discovered_models().len()
        ),
        Err(_) => format!(
            "slave {} | {} supported models",
            device.slave_id,
            device.models.discovered_models().len()
        ),
    }
}

async fn read_typed_model<M, C>(
    device: &AsyncDevice<C>,
    discovered: DiscoveredModel,
) -> Result<RenderedModel>
where
    M: Model + GroupMeta + Serialize,
    C: AsyncModbusClient,
{
    let model: M = device
        .read_model()
        .await
        .map_err(|error| anyhow!(error.to_string()))?;
    let value = serde_json::to_value(model)?;
    Ok(render_model_value(discovered, M::group_info(), &value))
}

async fn read_rendered_model<C: AsyncModbusClient>(
    device: &AsyncDevice<C>,
    discovered: DiscoveredModel,
) -> Result<RenderedModel> {
    match discovered.info.id {
        1 => read_typed_model::<sunspec::models::model1::Model1, _>(device, discovered).await,
        2 => read_typed_model::<sunspec::models::model2::Model2, _>(device, discovered).await,
        3 => read_typed_model::<sunspec::models::model3::Model3, _>(device, discovered).await,
        4 => read_typed_model::<sunspec::models::model4::Model4, _>(device, discovered).await,
        5 => read_typed_model::<sunspec::models::model5::Model5, _>(device, discovered).await,
        6 => read_typed_model::<sunspec::models::model6::Model6, _>(device, discovered).await,
        7 => read_typed_model::<sunspec::models::model7::Model7, _>(device, discovered).await,
        8 => read_typed_model::<sunspec::models::model8::Model8, _>(device, discovered).await,
        9 => read_typed_model::<sunspec::models::model9::Model9, _>(device, discovered).await,
        10 => read_typed_model::<sunspec::models::model10::Model10, _>(device, discovered).await,
        11 => read_typed_model::<sunspec::models::model11::Model11, _>(device, discovered).await,
        12 => read_typed_model::<sunspec::models::model12::Model12, _>(device, discovered).await,
        13 => read_typed_model::<sunspec::models::model13::Model13, _>(device, discovered).await,
        14 => read_typed_model::<sunspec::models::model14::Model14, _>(device, discovered).await,
        15 => read_typed_model::<sunspec::models::model15::Model15, _>(device, discovered).await,
        16 => read_typed_model::<sunspec::models::model16::Model16, _>(device, discovered).await,
        17 => read_typed_model::<sunspec::models::model17::Model17, _>(device, discovered).await,
        18 => read_typed_model::<sunspec::models::model18::Model18, _>(device, discovered).await,
        19 => read_typed_model::<sunspec::models::model19::Model19, _>(device, discovered).await,
        101 => read_typed_model::<sunspec::models::model101::Model101, _>(device, discovered).await,
        102 => read_typed_model::<sunspec::models::model102::Model102, _>(device, discovered).await,
        103 => read_typed_model::<sunspec::models::model103::Model103, _>(device, discovered).await,
        111 => read_typed_model::<sunspec::models::model111::Model111, _>(device, discovered).await,
        112 => read_typed_model::<sunspec::models::model112::Model112, _>(device, discovered).await,
        113 => read_typed_model::<sunspec::models::model113::Model113, _>(device, discovered).await,
        120 => read_typed_model::<sunspec::models::model120::Model120, _>(device, discovered).await,
        121 => read_typed_model::<sunspec::models::model121::Model121, _>(device, discovered).await,
        122 => read_typed_model::<sunspec::models::model122::Model122, _>(device, discovered).await,
        123 => read_typed_model::<sunspec::models::model123::Model123, _>(device, discovered).await,
        124 => read_typed_model::<sunspec::models::model124::Model124, _>(device, discovered).await,
        125 => read_typed_model::<sunspec::models::model125::Model125, _>(device, discovered).await,
        126 => read_typed_model::<sunspec::models::model126::Model126, _>(device, discovered).await,
        127 => read_typed_model::<sunspec::models::model127::Model127, _>(device, discovered).await,
        128 => read_typed_model::<sunspec::models::model128::Model128, _>(device, discovered).await,
        129 => read_typed_model::<sunspec::models::model129::Model129, _>(device, discovered).await,
        130 => read_typed_model::<sunspec::models::model130::Model130, _>(device, discovered).await,
        131 => read_typed_model::<sunspec::models::model131::Model131, _>(device, discovered).await,
        132 => read_typed_model::<sunspec::models::model132::Model132, _>(device, discovered).await,
        133 => read_typed_model::<sunspec::models::model133::Model133, _>(device, discovered).await,
        134 => read_typed_model::<sunspec::models::model134::Model134, _>(device, discovered).await,
        135 => read_typed_model::<sunspec::models::model135::Model135, _>(device, discovered).await,
        136 => read_typed_model::<sunspec::models::model136::Model136, _>(device, discovered).await,
        137 => read_typed_model::<sunspec::models::model137::Model137, _>(device, discovered).await,
        138 => read_typed_model::<sunspec::models::model138::Model138, _>(device, discovered).await,
        139 => read_typed_model::<sunspec::models::model139::Model139, _>(device, discovered).await,
        140 => read_typed_model::<sunspec::models::model140::Model140, _>(device, discovered).await,
        141 => read_typed_model::<sunspec::models::model141::Model141, _>(device, discovered).await,
        142 => read_typed_model::<sunspec::models::model142::Model142, _>(device, discovered).await,
        143 => read_typed_model::<sunspec::models::model143::Model143, _>(device, discovered).await,
        144 => read_typed_model::<sunspec::models::model144::Model144, _>(device, discovered).await,
        145 => read_typed_model::<sunspec::models::model145::Model145, _>(device, discovered).await,
        160 => read_typed_model::<sunspec::models::model160::Model160, _>(device, discovered).await,
        201 => read_typed_model::<sunspec::models::model201::Model201, _>(device, discovered).await,
        202 => read_typed_model::<sunspec::models::model202::Model202, _>(device, discovered).await,
        203 => read_typed_model::<sunspec::models::model203::Model203, _>(device, discovered).await,
        204 => read_typed_model::<sunspec::models::model204::Model204, _>(device, discovered).await,
        211 => read_typed_model::<sunspec::models::model211::Model211, _>(device, discovered).await,
        212 => read_typed_model::<sunspec::models::model212::Model212, _>(device, discovered).await,
        213 => read_typed_model::<sunspec::models::model213::Model213, _>(device, discovered).await,
        214 => read_typed_model::<sunspec::models::model214::Model214, _>(device, discovered).await,
        220 => read_typed_model::<sunspec::models::model220::Model220, _>(device, discovered).await,
        302 => read_typed_model::<sunspec::models::model302::Model302, _>(device, discovered).await,
        303 => read_typed_model::<sunspec::models::model303::Model303, _>(device, discovered).await,
        304 => read_typed_model::<sunspec::models::model304::Model304, _>(device, discovered).await,
        305 => read_typed_model::<sunspec::models::model305::Model305, _>(device, discovered).await,
        306 => read_typed_model::<sunspec::models::model306::Model306, _>(device, discovered).await,
        307 => read_typed_model::<sunspec::models::model307::Model307, _>(device, discovered).await,
        308 => read_typed_model::<sunspec::models::model308::Model308, _>(device, discovered).await,
        401 => read_typed_model::<sunspec::models::model401::Model401, _>(device, discovered).await,
        402 => read_typed_model::<sunspec::models::model402::Model402, _>(device, discovered).await,
        403 => read_typed_model::<sunspec::models::model403::Model403, _>(device, discovered).await,
        404 => read_typed_model::<sunspec::models::model404::Model404, _>(device, discovered).await,
        501 => read_typed_model::<sunspec::models::model501::Model501, _>(device, discovered).await,
        502 => read_typed_model::<sunspec::models::model502::Model502, _>(device, discovered).await,
        601 => read_typed_model::<sunspec::models::model601::Model601, _>(device, discovered).await,
        701 => read_typed_model::<sunspec::models::model701::Model701, _>(device, discovered).await,
        702 => read_typed_model::<sunspec::models::model702::Model702, _>(device, discovered).await,
        703 => read_typed_model::<sunspec::models::model703::Model703, _>(device, discovered).await,
        704 => read_typed_model::<sunspec::models::model704::Model704, _>(device, discovered).await,
        705 => read_typed_model::<sunspec::models::model705::Model705, _>(device, discovered).await,
        706 => read_typed_model::<sunspec::models::model706::Model706, _>(device, discovered).await,
        707 => read_typed_model::<sunspec::models::model707::Model707, _>(device, discovered).await,
        708 => read_typed_model::<sunspec::models::model708::Model708, _>(device, discovered).await,
        709 => read_typed_model::<sunspec::models::model709::Model709, _>(device, discovered).await,
        710 => read_typed_model::<sunspec::models::model710::Model710, _>(device, discovered).await,
        711 => read_typed_model::<sunspec::models::model711::Model711, _>(device, discovered).await,
        712 => read_typed_model::<sunspec::models::model712::Model712, _>(device, discovered).await,
        713 => read_typed_model::<sunspec::models::model713::Model713, _>(device, discovered).await,
        714 => read_typed_model::<sunspec::models::model714::Model714, _>(device, discovered).await,
        715 => read_typed_model::<sunspec::models::model715::Model715, _>(device, discovered).await,
        801 => read_typed_model::<sunspec::models::model801::Model801, _>(device, discovered).await,
        802 => read_typed_model::<sunspec::models::model802::Model802, _>(device, discovered).await,
        803 => read_typed_model::<sunspec::models::model803::Model803, _>(device, discovered).await,
        804 => read_typed_model::<sunspec::models::model804::Model804, _>(device, discovered).await,
        805 => read_typed_model::<sunspec::models::model805::Model805, _>(device, discovered).await,
        806 => read_typed_model::<sunspec::models::model806::Model806, _>(device, discovered).await,
        807 => read_typed_model::<sunspec::models::model807::Model807, _>(device, discovered).await,
        808 => read_typed_model::<sunspec::models::model808::Model808, _>(device, discovered).await,
        809 => read_typed_model::<sunspec::models::model809::Model809, _>(device, discovered).await,
        63001 => {
            read_typed_model::<sunspec::models::model63001::Model63001, _>(device, discovered).await
        }
        63002 => {
            read_typed_model::<sunspec::models::model63002::Model63002, _>(device, discovered).await
        }
        64001 => {
            read_typed_model::<sunspec::models::model64001::Model64001, _>(device, discovered).await
        }
        64020 => {
            read_typed_model::<sunspec::models::model64020::Model64020, _>(device, discovered).await
        }
        64101 => {
            read_typed_model::<sunspec::models::model64101::Model64101, _>(device, discovered).await
        }
        64111 => {
            read_typed_model::<sunspec::models::model64111::Model64111, _>(device, discovered).await
        }
        64112 => {
            read_typed_model::<sunspec::models::model64112::Model64112, _>(device, discovered).await
        }
        64410 => {
            read_typed_model::<sunspec::models::model64410::Model64410, _>(device, discovered).await
        }
        64411 => {
            read_typed_model::<sunspec::models::model64411::Model64411, _>(device, discovered).await
        }
        64412 => {
            read_typed_model::<sunspec::models::model64412::Model64412, _>(device, discovered).await
        }
        64413 => {
            read_typed_model::<sunspec::models::model64413::Model64413, _>(device, discovered).await
        }
        64414 => {
            read_typed_model::<sunspec::models::model64414::Model64414, _>(device, discovered).await
        }
        64415 => {
            read_typed_model::<sunspec::models::model64415::Model64415, _>(device, discovered).await
        }
        _ => bail!(
            "model {} is not supported by this build",
            discovered.info.id
        ),
    }
}

fn render_model_value(
    model: DiscoveredModel,
    group_info: &'static GroupInfo,
    value: &Value,
) -> RenderedModel {
    let summary_lines = vec![
        format!("ID: {}", model.info.id),
        format!("Label: {}", model.info.label),
        format!("Name: {}", model.info.name),
        format!("Address: {}", model.addr),
        format!("Length: {}", model.len),
        format!(
            "Description: {}",
            if model.info.description.is_empty() {
                "-"
            } else {
                model.info.description
            }
        ),
    ];

    RenderedModel {
        summary_lines,
        value_tree: build_tree(value, group_info, ""),
    }
}

fn format_scalar(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::String(value) => value.clone(),
        Value::Array(_) | Value::Object(_) => unreachable!("nested values are handled separately"),
    }
}

fn build_tree(value: &Value, group_info: &GroupInfo, parent_path: &str) -> Vec<TreeNode> {
    let Value::Object(map) = value else {
        return vec![TreeNode {
            path: parent_path.to_string(),
            label: group_info.label.to_string(),
            value: TreeValue::Scalar(format_scalar(value)),
        }];
    };

    let mut nodes = Vec::new();
    for field in group_info.fields {
        let Some(field_value) = map.get(field.name) else {
            continue;
        };
        let path = join_path(parent_path, field.name);
        match field.kind {
            FieldKind::Point => nodes.push(TreeNode {
                path,
                label: field.label.to_string(),
                value: TreeValue::Scalar(format_scalar(field_value)),
            }),
            FieldKind::Group(group_info_fn) => nodes.push(TreeNode {
                path: path.clone(),
                label: field.label.to_string(),
                value: TreeValue::Branch {
                    kind: BranchKind::Object,
                    children: build_tree(field_value, group_info_fn(), &path),
                },
            }),
            FieldKind::RepeatingGroup(group_info_fn) => {
                let children = match field_value {
                    Value::Array(items) => items
                        .iter()
                        .enumerate()
                        .map(|(index, item)| {
                            let item_path = join_path(&path, &format!("[{index}]"));
                            TreeNode {
                                path: item_path.clone(),
                                label: format!("[{index}]"),
                                value: TreeValue::Branch {
                                    kind: BranchKind::Object,
                                    children: build_tree(item, group_info_fn(), &item_path),
                                },
                            }
                        })
                        .collect(),
                    _ => Vec::new(),
                };
                nodes.push(TreeNode {
                    path,
                    label: field.label.to_string(),
                    value: TreeValue::Branch {
                        kind: BranchKind::Array,
                        children,
                    },
                });
            }
        }
    }
    nodes
}

fn append_group_rows(
    node: &TreeNode,
    depth: usize,
    expanded: &HashSet<String>,
    out: &mut Vec<TreeRow>,
) {
    let TreeValue::Branch { kind, children } = &node.value else {
        return;
    };
    let is_expanded = expanded.contains(&node.path);

    out.push(TreeRow {
        path: node.path.clone(),
        label: node.label.clone(),
        depth,
        value_preview: Some(format!("{} {}", kind.label(), children.len())),
        expandable: true,
        expanded: is_expanded,
    });

    if is_expanded {
        for child in children {
            append_group_rows(child, depth + 1, expanded, out);
        }
    }
}

fn direct_point_lines(nodes: &[TreeNode]) -> Vec<Line<'static>> {
    nodes
        .iter()
        .filter_map(|node| match &node.value {
            TreeValue::Scalar(value) => Some(Line::from(vec![
                Span::styled(
                    node.label.clone(),
                    Style::default()
                        .fg(Color::Rgb(255, 196, 107))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(": ", Style::default().fg(Color::Rgb(92, 103, 125))),
                Span::styled(
                    value.clone(),
                    Style::default().fg(Color::Rgb(232, 236, 243)),
                ),
            ])),
            TreeValue::Branch { .. } => None,
        })
        .collect()
}

fn find_group_node<'a>(nodes: &'a [TreeNode], path: &str) -> Option<&'a TreeNode> {
    for node in nodes {
        if node.path == path {
            return Some(node);
        }
        if let TreeValue::Branch { children, .. } = &node.value {
            if let Some(found) = find_group_node(children, path) {
                return Some(found);
            }
        }
    }
    None
}

fn points_for_group(nodes: &[TreeNode], path: &str) -> Vec<Line<'static>> {
    let points = if path.is_empty() {
        direct_point_lines(nodes)
    } else {
        find_group_node(nodes, path)
            .map(|group| match &group.value {
                TreeValue::Branch { children, .. } => direct_point_lines(children),
                TreeValue::Scalar(_) => Vec::new(),
            })
            .unwrap_or_default()
    };

    if points.is_empty() {
        vec![Line::from(Span::styled(
            "No direct points in this group.",
            Style::default().fg(Color::Rgb(152, 161, 178)),
        ))]
    } else {
        points
    }
}

fn style_group_row(row: &TreeRow, selected: bool) -> Line<'static> {
    let indent = "  ".repeat(row.depth);
    let marker = if row.expanded { "▾" } else { "▸" };

    let base_style = if selected {
        Style::default()
            .bg(Color::Rgb(63, 177, 255))
            .fg(Color::Rgb(8, 12, 20))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Rgb(228, 233, 241))
    };

    let label_style = base_style.fg(if selected {
        Color::Rgb(8, 12, 20)
    } else {
        Color::Rgb(145, 216, 255)
    });

    let preview_style = base_style.fg(if selected {
        Color::Rgb(8, 12, 20)
    } else {
        Color::Rgb(166, 174, 189)
    });

    let mut spans = vec![
        Span::styled(indent, base_style),
        Span::styled(format!("{marker} "), base_style),
        Span::styled(row.label.clone(), label_style),
    ];

    if let Some(preview) = &row.value_preview {
        spans.push(Span::styled(": ", preview_style));
        spans.push(Span::styled(preview.clone(), preview_style));
    }

    Line::from(spans)
}

fn join_path(parent: &str, segment: &str) -> String {
    if parent.is_empty() || segment.starts_with('[') {
        format!("{parent}{segment}")
    } else {
        format!("{parent}.{segment}")
    }
}

fn parent_path(path: &str) -> Option<String> {
    if path.is_empty() {
        return None;
    }
    if let Some(index) = path.rfind('.') {
        return Some(path[..index].to_string());
    }
    if let Some(index) = path.rfind('[') {
        if index == 0 {
            return None;
        }
        return Some(path[..index].to_string());
    }
    None
}

fn collect_expandable_paths(node: &TreeNode, expanded: &mut HashSet<String>) {
    if let TreeValue::Branch { children, .. } = &node.value {
        expanded.insert(node.path.clone());
        for child in children {
            collect_expandable_paths(child, expanded);
        }
    }
}

fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).map_err(Into::into)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn section_block<'a>(title: &'a str, accent: Color) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(accent))
        .title(Span::styled(
            format!(" {title} "),
            Style::default().fg(accent).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(Color::Rgb(14, 20, 30)))
}

fn key_style() -> Style {
    Style::default()
        .fg(Color::Rgb(145, 216, 255))
        .add_modifier(Modifier::BOLD)
}

fn style_detail_line(line: &str) -> Line<'static> {
    if line.is_empty() {
        return Line::raw("");
    }

    if line == "Values:" {
        return Line::from(Span::styled(
            line.to_string(),
            Style::default()
                .fg(Color::Rgb(145, 216, 255))
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        ));
    }

    if let Some((key, value)) = line.split_once(':') {
        let indent_width = key.len() - key.trim_start().len();
        let indent = " ".repeat(indent_width);
        let trimmed_key = key.trim();
        let key_style = if indent_width == 0 {
            Style::default()
                .fg(Color::Rgb(255, 196, 107))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Rgb(123, 211, 255))
        };
        return Line::from(vec![
            Span::raw(indent),
            Span::styled(trimmed_key.to_string(), key_style),
            Span::styled(":", Style::default().fg(Color::Rgb(92, 103, 125))),
            Span::styled(
                value.to_string(),
                Style::default().fg(Color::Rgb(232, 236, 243)),
            ),
        ]);
    }

    if line.trim_start().starts_with('[') {
        return Line::from(Span::styled(
            line.to_string(),
            Style::default().fg(Color::Rgb(196, 167, 255)),
        ));
    }

    Line::from(Span::styled(
        line.to_string(),
        Style::default().fg(Color::Rgb(180, 188, 203)),
    ))
}

impl BranchKind {
    fn label(self) -> &'static str {
        match self {
            BranchKind::Object => "group",
            BranchKind::Array => "items",
        }
    }
}
