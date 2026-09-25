export type Priority = 'none' | 'low' | 'medium' | 'high';

export type RepeatRule = 'daily' | 'weekly' | 'monthly';

export type ViewMode = 'board' | 'list';

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
