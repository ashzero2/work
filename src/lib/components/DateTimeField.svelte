<script lang="ts">
  import { CalendarDays, ChevronLeft, ChevronRight } from '@lucide/svelte';

  let { value = $bindable(''), label = 'Due date' }: { value: string; label?: string } = $props();
  let open = $state(false);
  let month = $state(new Date(new Date().getFullYear(), new Date().getMonth(), 1));

  const datePart = $derived(value.split('T')[0] ?? '');
  const timePart = $derived(value.split('T')[1] ?? '09:00');
  const monthLabel = $derived(
    new Intl.DateTimeFormat(undefined, { month: 'long', year: 'numeric' }).format(month)
  );
  const displayDate = $derived(
    datePart
      ? new Intl.DateTimeFormat(undefined, { month: 'short', day: 'numeric', year: 'numeric' }).format(
          new Date(`${datePart}T12:00:00`)
        )
      : 'Choose a date'
  );
  const cells = $derived.by(() => {
    const first = new Date(month.getFullYear(), month.getMonth(), 1 - month.getDay());
    return Array.from({ length: 42 }, (_, index) => {
      const date = new Date(first.getFullYear(), first.getMonth(), first.getDate() + index);
      return {
        date,
        key: localDate(date),
        inMonth: date.getMonth() === month.getMonth()
      };
    });
  });

  $effect(() => {
    if (!open || !datePart) return;
    const selected = new Date(`${datePart}T12:00:00`);
    month = new Date(selected.getFullYear(), selected.getMonth(), 1);
  });

  function localDate(date: Date): string {
    return `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`;
  }

  function chooseDate(date: Date): void {
    const nextDate = localDate(date);
    value = `${nextDate}T${timePart || '09:00'}`;
    month = new Date(date.getFullYear(), date.getMonth(), 1);
    open = false;
  }

  function shiftMonth(amount: number): void {
    month = new Date(month.getFullYear(), month.getMonth() + amount, 1);
  }

  function clearDate(): void {
    value = '';
    open = false;
  }
</script>

<div class="date-field">
  <span class="label">{label}</span>
  <div class="controls">
    <div class="picker">
      <button
        type="button"
        class="date-button"
        aria-haspopup="dialog"
        aria-expanded={open}
        onclick={() => (open = !open)}
      >
        <CalendarDays size={15} />
        <span>{displayDate}</span>
      </button>
      {#if open}
        <div class="calendar" role="group" aria-label="Choose due date">
          <div class="calendar-head">
            <button type="button" class="month-nav" aria-label="Previous month" onclick={() => shiftMonth(-1)}><ChevronLeft size={16} /></button>
            <strong>{monthLabel}</strong>
            <button type="button" class="month-nav" aria-label="Next month" onclick={() => shiftMonth(1)}><ChevronRight size={16} /></button>
          </div>
          <div class="weekdays" aria-hidden="true">
            {#each ['S', 'M', 'T', 'W', 'T', 'F', 'S'] as day, index (`${day}-${index}`)}<span>{day}</span>{/each}
          </div>
          <div class="days">
            {#each cells as cell (cell.key)}
              <button
                type="button"
                class="day"
                class:outside={!cell.inMonth}
                class:selected={datePart === cell.key}
                class:today={localDate(new Date()) === cell.key}
                aria-label={new Intl.DateTimeFormat(undefined, { dateStyle: 'full' }).format(cell.date)}
                aria-pressed={datePart === cell.key}
                onclick={() => chooseDate(cell.date)}
              >{cell.date.getDate()}</button>
            {/each}
          </div>
          <div class="calendar-foot"><button type="button" class="clear" onclick={clearDate}>Clear date</button><button type="button" class="today-button" onclick={() => chooseDate(new Date())}>Today</button></div>
        </div>
      {/if}
    </div>
    <label class="time-control"><span class="sr-only">Due time</span><input type="time" value={datePart ? timePart : ''} disabled={!datePart} oninput={(event) => value = `${datePart}T${event.currentTarget.value}`} /></label>
  </div>
</div>

<style>
  .date-field { display: flex; flex-direction: column; gap: 8px; }
  .label { font-size: 13px; font-weight: 600; }
  .controls { display: flex; gap: 8px; }
  .picker { position: relative; flex: 1; min-width: 0; }
  .date-button, input { min-height: 38px; border: 1px solid var(--field-border); border-radius: var(--radius-sm); background: var(--bg); color: var(--fg); font: inherit; font-size: 13px; }
  .date-button { display: flex; align-items: center; gap: 9px; width: 100%; padding: 0 10px; text-align: left; cursor: pointer; }
  .date-button span { flex: 1; }
  .date-button:hover, .month-nav:hover, .day:hover { background: var(--hover); }
  .time-control { width: 120px; flex-shrink: 0; }
  input { width: 100%; padding: 0 9px; font-variant-numeric: tabular-nums; }
  input:disabled { opacity: .55; }
  .calendar { position: absolute; z-index: 5; top: calc(100% + 6px); left: 0; width: 276px; padding: 10px; border: 1px solid var(--border); border-radius: var(--radius); background: var(--panel); box-shadow: var(--shadow-md); }
  .calendar-head { display: grid; grid-template-columns: 32px 1fr 32px; align-items: center; margin-bottom: 8px; text-align: center; }
  .calendar-head strong { font-size: 13px; }
  .month-nav { display: grid; width: 30px; height: 30px; place-items: center; border: 0; border-radius: var(--radius-sm); background: transparent; color: var(--fg); cursor: pointer; }
  .weekdays, .days { display: grid; grid-template-columns: repeat(7, 1fr); gap: 2px; text-align: center; }
  .weekdays { margin-bottom: 3px; color: var(--muted-fg); font-size: 11px; font-weight: 600; }
  .weekdays span { padding: 5px 0; }
  .day { display: grid; aspect-ratio: 1; place-items: center; border: 0; border-radius: 50%; background: transparent; color: var(--fg); font: inherit; font-size: 12px; cursor: pointer; }
  .day.outside { color: var(--muted-fg); opacity: .58; }
  .day.today { outline: 1px solid var(--accent); outline-offset: -2px; }
  .day.selected { background: var(--accent); color: var(--accent-fg); outline: 0; }
  .calendar-foot { display: flex; justify-content: space-between; margin-top: 7px; padding-top: 7px; border-top: 1px solid var(--border); }
  .clear, .today-button { padding: 5px 7px; border: 0; border-radius: var(--radius-sm); background: transparent; color: var(--muted-fg); font: inherit; font-size: 11px; cursor: pointer; }
  .today-button { color: var(--accent); }
  .clear:hover, .today-button:hover { background: var(--hover); }
  @media (max-width: 520px) { .time-control { width: 104px; } .calendar { width: min(276px, calc(100vw - 80px)); } }
</style>
