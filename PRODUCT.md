# Product

<!-- impeccable:product-schema 1 -->

## Platform

adaptive

## Users

The primary user is a solo knowledge worker or maker, initially Rahul, who needs one focused desktop workspace for planning and completing personal work without assembling a stack of separate tools.

The project is intended to become an open-source option for other solo users with the same need.

## Product Purpose

Worke is a minimal work-management desktop app for tasks, markdown notes, focused work sessions, and reminders. It exists to make everyday work capture and follow-through feel lightweight and coherent. Success means managing the daily work loop without paying for a heavy suite or maintaining a collection of plugins and companion apps.

## Positioning

Worke combines a real task workspace, file-backed markdown notes, a native Pomodoro loop, and operating-system integrations in one local-first app. Its position is defined by deliberate small scope, open-source ownership, no subscription requirement, and an integrated daily workflow rather than a broad team-product feature set.

## Operating Context

Worke is used as a desktop companion during daily individual work. Users capture tasks from the app or a global shortcut, organize them in a kanban or list view, record quick notes and meeting notes, attach focus sessions to tasks, and rely on native reminders and a menu-bar timer while the main window is hidden or closed.

The app is cross-platform in its product direction, with macOS-native behavior prioritized for the first implementation. It works locally without an account or server. Structured task data lives in SQLite; note bodies live as markdown files with frontmatter and are indexed locally.

## Capabilities and Constraints

- Tasks support CRUD, kanban and list views, configurable columns, priorities, due dates, one level of subtasks, minimal daily/weekly/monthly recurrence, and drag reordering.
- Notes support quick creation, markdown bodies, tags, full-text search, optional task links, and spatial corkboard positioning.
- Focus supports configurable work and break durations, task attachment, session logging, and a menu-bar/tray view.
- Reminders are manual and can use native OS notifications with actionable Snooze and Dismiss controls plus per-kind toggles.
- The app provides a global quick-capture shortcut and JSON/CSV task export.
- Storage is local-first and hybrid: SQLite is used for structured data and indexing; markdown files are the source of truth for note bodies.
- The frontend is a SvelteKit SPA using Svelte 5 and plain CSS; the desktop shell and storage/domain layer use Tauri v2 and Rust.
- The product does not include accounts, a server dependency, in-app quiet hours, automatic reminder rules, time-analytics UI, or cross-device sync in v1. These remain explicitly deferred until real usage demonstrates a need.
- Native notification, tray, and shortcut behavior must be verified in packaged builds, with macOS-specific behavior kept behind platform boundaries.

## Brand Commitments

- Working product name: Worke; the desktop product name currently appears as Work Dashboard in the Tauri configuration.
- The product is open-source and minimal by intent.
- The voice should remain direct, practical, and low-friction rather than enterprise-oriented or feature-marketing-heavy.

## Evidence on Hand

- `docs/plans/work-dashboard-plan.md` contains the confirmed architecture, data model, feature scope, and product decisions.
- `docs/plans/work-dashboard-market-research.md` contains comparative research on task, notes, and Pomodoro tools.
- `docs/plans/notes-research-deep-dive.md` contains research on note organization, menu-bar Pomodoro usage, and scope decisions.
- `docs/plans/ui-design-research.md` contains interaction and usability research for the existing interface direction.
- The repository contains an implemented Tauri v2 + SvelteKit prototype with task, notes, focus, reminders, settings, quick capture, and native integration surfaces.
- No user testimonials, customer studies, pricing claims, or external performance benchmarks are approved product evidence; future work must not fabricate them.

## Product Principles

- Keep the daily work loop small enough to use without managing the tool.
- Prefer local ownership, portable files, and transparent storage over account dependence.
- Integrate tasks, notes, focus, and reminders where the connection removes friction.
- Add scope only when real usage proves that the missing capability matters.
- Treat native operating-system behavior and keyboard access as part of the product, not optional polish.
