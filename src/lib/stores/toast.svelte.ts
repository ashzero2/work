class ToastStore {
  message = $state<string | null>(null);

  show(message: string): void {
    this.message = message;
    window.setTimeout(() => {
      if (this.message === message) this.message = null;
    }, 3500);
  }

  clear(): void {
    this.message = null;
  }
}

export const toast = new ToastStore();
