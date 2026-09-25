export type Priority = 'none' | 'low' | 'medium' | 'high';

export type RepeatRule = 'daily' | 'weekly' | 'monthly';

export type ViewMode = 'board' | 'list';

export type Section = 'tasks' | 'notes';

export interface Task {
  id: number;
  title: string;
  description: string | null;
  columnId: number;
  position: number;
  priority: Priority;
  dueAt: string | null;
  completedAt: string | null;
  repeatRule: RepeatRule | null;
  parentTaskId: number | null;
  createdAt: string;
  updatedAt: string;
}

export interface Column {
  id: number;
  name: string;
  orderIndex: number;
  color: string | null;
  wipLimit: number | null;
}

export interface NewTask {
  title: string;
  description: string | null;
  columnId: number;
  priority: Priority;
  dueAt: string | null;
  repeatRule: RepeatRule | null;
  parentTaskId: number | null;
}

export interface NewColumn {
  name: string;
  color: string | null;
  wipLimit: number | null;
}

export interface Note {
  id: number;
  title: string;
  filePath: string;
  linkedTaskId: number | null;
  color: string;
  rotationDeg: number;
  posX: number | null;
  posY: number | null;
  createdAt: string;
  updatedAt: string;
}

export interface NoteCard extends Note {
  tags: string[];
}

export interface TagSummary {
  id: number;
  name: string;
  color: string | null;
  noteCount: number;
}
