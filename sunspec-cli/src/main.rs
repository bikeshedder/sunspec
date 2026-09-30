use std::{collections::HashSet, io, net::SocketAddr, time::Duration};

use anyhow::{anyhow, Context, Result};
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
    widgets::{
        Block, Borders, Cell, Clear, List, ListItem, ListState, Paragraph, Row, Table, TableState,
        Wrap,
    },
    Terminal,
};
use serde_json::Value;
use sunspec::{
    client::{AsyncClient, AsyncDevice, AsyncModbusClient, Config},
    models::model1::Model1,
    FieldKind, GroupInfo, ModelInfo, Models, MODELS,
};
use tokio_modbus::client::tcp::connect;

#[derive(Parser)]
struct Args {
    addr: SocketAddr,
    device_id: u8,
}

#[derive(Clone, Copy, Debug)]
struct DiscoveredModel {
    info: &'static ModelInfo,
    addr: u16,
    len: u16,
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
    group_selected: usize,
    group_scroll: usize,
    point_state: TableState,
    point_scroll: usize,
    focus: DetailFocus,
}

#[derive(Debug)]
struct RenderedModel {
    summary_lines: Vec<String>,
    value_tree: Vec<TreeNode>,
}

#[derive(Debug)]
struct TreeNode {
    path: String,
    name: String,
    label: String,
    description: String,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DetailFocus {
    Groups,
    Points,
}

#[derive(Clone, Debug)]
struct PointRow {
    name: String,
    label: String,
    value: String,
    description: String,
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
                    group_selected: 0,
                    group_scroll: 0,
                    point_state: TableState::default().with_selected(Some(0)),
                    point_scroll: 0,
                    focus: DetailFocus::Groups,
                });
                self.status =
                    "Tab switches panels. Groups use arrows to navigate. Esc returns. r reloads."
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
                    detail.group_selected = 0;
                    detail.group_scroll = 0;
                    detail.point_state.select(Some(0));
                    detail.point_scroll = 0;
                    detail.focus = DetailFocus::Groups;
                } else {
                    self.screen = Screen::Detail(ModelDetail {
                        model,
                        summary_lines: rendered.summary_lines,
                        value_tree: rendered.value_tree,
                        expanded: HashSet::new(),
                        group_selected: 0,
                        group_scroll: 0,
                        point_state: TableState::default().with_selected(Some(0)),
                        point_scroll: 0,
                        focus: DetailFocus::Groups,
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
            detail.move_down();
        }
    }

    fn scroll_up(&mut self) {
        if let Screen::Detail(detail) = &mut self.screen {
            detail.move_up();
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

    fn toggle_focus(&mut self) {
        if let Screen::Detail(detail) = &mut self.screen {
            detail.toggle_focus();
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

    fn move_group_next(&mut self) {
        let len = self.visible_group_rows().len();
        if len == 0 {
            self.group_selected = 0;
            return;
        }
        self.group_selected = (self.group_selected + 1).min(len - 1);
        self.reset_points_for_group();
    }

    fn move_group_previous(&mut self) {
        self.group_selected = self.group_selected.saturating_sub(1);
        self.reset_points_for_group();
    }

    fn expand_selected(&mut self) {
        let rows = self.visible_group_rows();
        let Some(row) = rows.get(self.group_selected) else {
            return;
        };
        if row.expandable {
            self.expanded.insert(row.path.clone());
        }
    }

    fn collapse_selected(&mut self) {
        let rows = self.visible_group_rows();
        let Some(row) = rows.get(self.group_selected) else {
            return;
        };
        if row.expandable && row.expanded {
            self.expanded.remove(&row.path);
            return;
        }
        if let Some(parent) = parent_path(&row.path) {
            if let Some(index) = rows.iter().position(|candidate| candidate.path == parent) {
                self.group_selected = index;
                self.reset_points_for_group();
            }
        }
    }

    fn sync_group_scroll(&mut self, viewport_height: usize) {
        let len = self.visible_group_rows().len();
        if len == 0 {
            self.group_selected = 0;
            self.group_scroll = 0;
            return;
        }
        self.group_selected = self.group_selected.min(len - 1);
        let viewport_height = viewport_height.max(1);
        let max_scroll = len.saturating_sub(viewport_height);
        if self.group_selected < self.group_scroll {
            self.group_scroll = self.group_selected;
        } else if self.group_selected >= self.group_scroll + viewport_height {
            self.group_scroll = self.group_selected + 1 - viewport_height;
        }
        self.group_scroll = self.group_scroll.min(max_scroll);
    }

    fn expand_all(&mut self) {
        self.expanded.clear();
        for node in &self.value_tree {
            collect_expandable_paths(node, &mut self.expanded);
        }
    }

    fn collapse_all(&mut self) {
        self.expanded.clear();
        self.group_selected = 0;
        self.group_scroll = 0;
        self.reset_points_for_group();
    }

    fn selected_group_path(&self) -> String {
        self.visible_group_rows()
            .get(self.group_selected)
            .map(|row| row.path.clone())
            .unwrap_or_default()
    }

    fn visible_points(&self) -> Vec<PointRow> {
        points_for_group(&self.value_tree, &self.selected_group_path())
    }

    fn move_point_next(&mut self) {
        let len = self.visible_points().len();
        if len == 0 {
            self.point_state.select(None);
            return;
        }
        let next = match self.point_state.selected() {
            Some(index) => (index + 1).min(len - 1),
            None => 0,
        };
        self.point_state.select(Some(next));
    }

    fn move_point_previous(&mut self) {
        let next = self.point_state.selected().unwrap_or(0).saturating_sub(1);
        self.point_state.select(Some(next));
    }

    fn sync_point_scroll(&mut self, viewport_height: usize) {
        let len = self.visible_points().len();
        if len == 0 {
            self.point_state.select(None);
            self.point_scroll = 0;
            return;
        }
        let selected = self.point_state.selected().unwrap_or(0).min(len - 1);
        self.point_state.select(Some(selected));
        let viewport_height = viewport_height.max(1);
        let max_scroll = len.saturating_sub(viewport_height);
        if selected < self.point_scroll {
            self.point_scroll = selected;
        } else if selected >= self.point_scroll + viewport_height {
            self.point_scroll = selected + 1 - viewport_height;
        }
        self.point_scroll = self.point_scroll.min(max_scroll);
    }

    fn selected_point_description(&self) -> String {
        let points = self.visible_points();
        match self.point_state.selected().and_then(|i| points.get(i)) {
            Some(point) if !point.description.is_empty() => point.description.clone(),
            Some(_) => "No description available for this point.".to_string(),
            None => "No direct points in this group.".to_string(),
        }
    }

    fn move_down(&mut self) {
        match self.focus {
            DetailFocus::Groups => self.move_group_next(),
            DetailFocus::Points => self.move_point_next(),
        }
    }

    fn move_up(&mut self) {
        match self.focus {
            DetailFocus::Groups => self.move_group_previous(),
            DetailFocus::Points => self.move_point_previous(),
        }
    }

    fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            DetailFocus::Groups => DetailFocus::Points,
            DetailFocus::Points => DetailFocus::Groups,
        };
    }

    fn reset_points_for_group(&mut self) {
        if self.visible_points().is_empty() {
            self.point_state.select(None);
        } else {
            self.point_state.select(Some(0));
        }
        self.point_scroll = 0;
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

    let discovered_models = discovered_models(&device.models);
    let device_summary = read_device_summary(&device, discovered_models.len()).await;
    let unknown_models = device
        .unknown_models
        .iter()
        .map(|model| format!("{} @ {} (len {})", model.id, model.addr, model.len))
        .collect();
    let mut app = App::new(device_summary, discovered_models, unknown_models);

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
            KeyCode::Tab | KeyCode::BackTab => app.toggle_focus(),
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
                } else if matches!(&app.screen, Screen::Detail(detail) if detail.focus == DetailFocus::Groups)
                {
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
                Span::styled("Tab", key_style()),
                Span::raw(" switch pane  "),
                Span::styled("↑/↓", key_style()),
                Span::raw(" move  "),
                Span::styled("→/Enter", key_style()),
                Span::raw(" expand group  "),
                Span::styled("←", key_style()),
                Span::raw(" collapse group  "),
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
            let detail = if model.info.group.description.is_empty() {
                Line::from(Span::styled(
                    format!("      {}", model.info.name),
                    Style::default().fg(Color::Rgb(152, 161, 178)),
                ))
            } else {
                Line::from(Span::styled(
                    format!("      {}", model.info.group.description),
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
    detail.sync_group_scroll(groups_inner_height);
    let rows = detail.visible_group_rows();
    let groups = Text::from(
        rows.iter()
            .enumerate()
            .map(|(index, row)| {
                style_group_row(
                    row,
                    detail.focus == DetailFocus::Groups && index == detail.group_selected,
                )
            })
            .collect::<Vec<_>>(),
    );
    let groups_paragraph = Paragraph::new(groups)
        .style(Style::default().fg(Color::Rgb(228, 233, 241)))
        .block(section_block_with_focus(
            "Groups",
            Color::Blue,
            detail.focus == DetailFocus::Groups,
        ))
        .scroll((detail.group_scroll as u16, 0))
        .wrap(Wrap { trim: false });
    frame.render_widget(groups_paragraph, sections[1]);

    let point_sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(4)])
        .split(sections[2]);
    let point_inner_height = point_sections[0].height.saturating_sub(3) as usize;
    detail.sync_point_scroll(point_inner_height);
    let point_rows = detail.visible_points();
    let table_rows = point_rows.iter().map(|point| {
        Row::new(vec![
            Cell::from(point.name.clone()),
            Cell::from(point.label.clone()),
            Cell::from(point.value.clone()),
        ])
    });
    let widths = [
        Constraint::Length(18),
        Constraint::Percentage(34),
        Constraint::Percentage(46),
    ];
    let points_table = Table::new(table_rows, widths)
        .header(
            Row::new(vec![
                Cell::from("Name"),
                Cell::from("Label"),
                Cell::from("Value"),
            ])
            .style(
                Style::default()
                    .fg(Color::Rgb(145, 216, 255))
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .block(section_block_with_focus(
            "Points",
            Color::Cyan,
            detail.focus == DetailFocus::Points,
        ))
        .row_highlight_style(
            Style::default()
                .bg(Color::Rgb(63, 177, 255))
                .fg(Color::Rgb(8, 12, 20))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▸ ")
        .column_spacing(1);
    frame.render_stateful_widget(points_table, point_sections[0], &mut detail.point_state);

    let point_description = Paragraph::new(detail.selected_point_description())
        .style(Style::default().fg(Color::Rgb(180, 188, 203)))
        .block(section_block("Description", Color::LightCyan))
        .wrap(Wrap { trim: true })
        .scroll((detail.point_scroll as u16, 0));
    frame.render_widget(point_description, point_sections[1]);
}

fn discovered_models(models: &Models) -> Vec<DiscoveredModel> {
    MODELS
        .iter()
        .map(|&info| (info, info.addr(models)))
        .filter(|(_, addr)| addr.addr != 0)
        .map(|(info, addr)| DiscoveredModel {
            info,
            addr: addr.addr,
            len: addr.len,
        })
        .collect()
}

async fn read_device_summary<C: AsyncModbusClient>(
    device: &AsyncDevice<C>,
    model_count: usize,
) -> String {
    match device.read_model::<Model1>().await {
        Ok(common) => format!(
            "{} | {} | serial {} | slave {} | {} supported models",
            common.mn, common.md, common.sn, device.slave_id, model_count
        ),
        Err(_) => format!(
            "slave {} | {} supported models",
            device.slave_id, model_count
        ),
    }
}

async fn read_rendered_model<C: AsyncModbusClient>(
    device: &AsyncDevice<C>,
    discovered: DiscoveredModel,
) -> Result<RenderedModel> {
    let model = device
        .read_any_model(discovered.info)
        .await
        .map_err(|error| anyhow!(error.to_string()))?;
    let value = serde_json::to_value(model)?;
    Ok(render_model_value(discovered, &value))
}

fn render_model_value(model: DiscoveredModel, value: &Value) -> RenderedModel {
    let summary_lines = vec![
        format!("ID: {}", model.info.id),
        format!("Label: {}", model.info.label),
        format!("Name: {}", model.info.name),
        format!("Address: {}", model.addr),
        format!("Length: {}", model.len),
        format!(
            "Description: {}",
            if model.info.group.description.is_empty() {
                "-"
            } else {
                model.info.group.description
            }
        ),
    ];

    RenderedModel {
        summary_lines,
        value_tree: build_tree(value, model.info.group, ""),
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
            name: group_info.name.to_string(),
            label: group_info.label.to_string(),
            description: group_info.description.to_string(),
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
                name: field.name.to_string(),
                label: field.label.to_string(),
                description: field.description.to_string(),
                value: TreeValue::Scalar(format_scalar(field_value)),
            }),
            FieldKind::Group(group_info) => nodes.push(TreeNode {
                path: path.clone(),
                name: field.name.to_string(),
                label: field.label.to_string(),
                description: field.description.to_string(),
                value: TreeValue::Branch {
                    kind: BranchKind::Object,
                    children: build_tree(field_value, group_info, &path),
                },
            }),
            FieldKind::RepeatingGroup(group_info) => {
                let children = match field_value {
                    Value::Array(items) => items
                        .iter()
                        .enumerate()
                        .map(|(index, item)| {
                            let item_path = join_path(&path, &format!("[{index}]"));
                            TreeNode {
                                path: item_path.clone(),
                                name: format!("[{index}]"),
                                label: format!("[{index}]"),
                                description: group_info.description.to_string(),
                                value: TreeValue::Branch {
                                    kind: BranchKind::Object,
                                    children: build_tree(item, group_info, &item_path),
                                },
                            }
                        })
                        .collect(),
                    _ => Vec::new(),
                };
                nodes.push(TreeNode {
                    path,
                    name: field.name.to_string(),
                    label: field.label.to_string(),
                    description: field.description.to_string(),
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

fn direct_point_rows(nodes: &[TreeNode]) -> Vec<PointRow> {
    nodes
        .iter()
        .filter_map(|node| match &node.value {
            TreeValue::Scalar(value) => Some(PointRow {
                name: node.name.clone(),
                label: node.label.clone(),
                value: value.clone(),
                description: node.description.clone(),
            }),
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

fn points_for_group(nodes: &[TreeNode], path: &str) -> Vec<PointRow> {
    let points = if path.is_empty() {
        direct_point_rows(nodes)
    } else {
        find_group_node(nodes, path)
            .map(|group| match &group.value {
                TreeValue::Branch { children, .. } => direct_point_rows(children),
                TreeValue::Scalar(_) => Vec::new(),
            })
            .unwrap_or_default()
    };

    points
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

fn section_block_with_focus<'a>(title: &'a str, accent: Color, focused: bool) -> Block<'a> {
    let border = if focused {
        Color::Rgb(255, 221, 87)
    } else {
        accent
    };
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border))
        .title(Span::styled(
            format!(" {title} "),
            Style::default().fg(border).add_modifier(Modifier::BOLD),
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
