import * as api from '$lib/api';
import { NoteCache } from '$lib/note-cache';
import { toast } from '$lib/stores/toast.svelte';
import type { NoteCard, TagSummary } from '$lib/types';

const SEARCH_DEBOUNCE_MS = 150;

/// Owns every note's metadata plus the note/tag filters. Bodies are not held
/// here beyond the small LRU — they're read from disk only when opened.
class NotesStore {
  notes = $state<NoteCard[]>([]);
  tags = $state<TagSummary[]>([]);
  activeTag = $state<string | null>(null);
  query = $state('');
  selectedId = $state<number | null>(null);
  loading = $state(true);

  private matchingIds = $state<number[] | null>(null);
  private bodies = new NoteCache();
  private searchTimer: number | undefined;

  async load(): Promise<void> {
    try {
      await this.refresh();
    } finally {
      this.loading = false;
    }
  }

  async refresh(): Promise<void> {
    try {
      const [notes, tags] = await Promise.all([api.listNotes(), api.listTags()]);
      this.notes = notes;
      this.tags = tags;
      if (this.activeTag && !tags.some((tag) => tag.name === this.activeTag)) {
        this.activeTag = null;
      }
    } catch (error) {
      toast.show(errorMessage(error));
    }
  }

  get visible(): NoteCard[] {
    return this.notes.filter((note) => {
      if (this.activeTag && !note.tags.includes(this.activeTag)) return false;
      if (this.matchingIds && !this.matchingIds.includes(note.id)) return false;
      return true;
    });
  }

  get selected(): NoteCard | null {
    return this.notes.find((note) => note.id === this.selectedId) ?? null;
  }

  get isSearching(): boolean {
    return this.matchingIds !== null;
  }

  setActiveTag(tag: string | null): void {
    this.activeTag = tag;
  }

  setQuery(value: string): void {
    this.query = value;
    window.clearTimeout(this.searchTimer);

    const trimmed = value.trim();
    if (!trimmed) {
      this.matchingIds = null;
      return;
    }

    this.searchTimer = window.setTimeout(async () => {
      try {
        this.matchingIds = await api.searchNotes(trimmed);
      } catch (error) {
        this.matchingIds = [];
        toast.show(errorMessage(error));
      }
    }, SEARCH_DEBOUNCE_MS);
  }

  open(id: number): void {
    this.selectedId = id;
  }

  closeEditor(): void {
    this.selectedId = null;
  }

  async bodyFor(id: number): Promise<string> {
    const cached = this.bodies.get(id);
    if (cached !== undefined) return cached;

    const body = await api.noteBody(id);
    this.bodies.set(id, body);
    return body;
  }

  async createNote(): Promise<void> {
    try {
      const note = await api.createNote('Untitled note');
      this.bodies.set(note.id, '');
      await this.refresh();
      this.selectedId = note.id;
    } catch (error) {
      toast.show(errorMessage(error));
    }
  }

  async saveNote(id: number, title: string, body: string, tags: string[]): Promise<void> {
    try {
      await api.saveNote(id, title, body, tags);
      this.bodies.set(id, body);
      await this.refresh();
    } catch (error) {
      toast.show(errorMessage(error));
      throw error;
    }
  }

  /// Local-only reposition, used while dragging so no write happens mid-gesture.
  dragTo(id: number, x: number, y: number): void {
    this.notes = this.notes.map((note) =>
      note.id === id ? { ...note, posX: x, posY: y } : note
    );
  }

  async moveNote(id: number, x: number, y: number): Promise<void> {
    this.dragTo(id, x, y);
    try {
      await api.moveNote(id, x, y);
    } catch (error) {
      toast.show(errorMessage(error));
      await this.refresh();
    }
  }

  async deleteNote(id: number): Promise<void> {
    this.notes = this.notes.filter((note) => note.id !== id);
    if (this.selectedId === id) this.selectedId = null;
    this.bodies.delete(id);

    try {
      await api.deleteNote(id);
    } catch (error) {
      toast.show(errorMessage(error));
    }
    await this.refresh();
  }

  async deleteTag(id: number): Promise<void> {
    try {
      await api.deleteTag(id);
    } catch (error) {
      toast.show(errorMessage(error));
    }
    await this.refresh();
  }
}

function errorMessage(error: unknown): string {
  return typeof error === 'string' ? error : String(error);
}

export const notes = new NotesStore();
