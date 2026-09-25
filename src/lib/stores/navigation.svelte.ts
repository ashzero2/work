import type { Section } from '$lib/types';

class NavigationStore {
  section = $state<Section>('tasks');
  sidebarCollapsed = $state(false);

  setSection(section: Section): void {
    this.section = section;
  }

  toggleSidebar(): void {
    this.sidebarCollapsed = !this.sidebarCollapsed;
  }
}

export const navigation = new NavigationStore();
