const CAPACITY = 20;

/// Count-based LRU for note bodies: reopening a note is instant, and memory
/// stays flat no matter how many notes exist.
export class NoteCache {
  private entries = new Map<number, string>();

  get(id: number): string | undefined {
    const body = this.entries.get(id);
    if (body === undefined) return undefined;
    this.entries.delete(id);
    this.entries.set(id, body);
    return body;
  }

  set(id: number, body: string): void {
    this.entries.delete(id);
    this.entries.set(id, body);
    while (this.entries.size > CAPACITY) {
      const oldest = this.entries.keys().next().value;
      if (oldest === undefined) break;
      this.entries.delete(oldest);
    }
  }

  delete(id: number): void {
    this.entries.delete(id);
  }
}
