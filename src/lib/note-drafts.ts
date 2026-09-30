export interface NoteDraft {
  title: string;
  body: string;
  tags: string[];
}

export const noteDrafts = new Map<number, NoteDraft>();
