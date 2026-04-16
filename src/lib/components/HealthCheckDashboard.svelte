<script lang="ts">
	import { healthStore } from '$lib/stores/health.svelte';
	import CategorySection from './CategorySection.svelte';
</script>

<div class="dashboard">
	<header>
		<div class="title-row">
			<h1>
				{#if healthStore.activeCategory}
					{healthStore.activeCategory}
				{:else}
					Health Check
				{/if}
			</h1>
			<button class="run-btn" onclick={healthStore.runChecks} disabled={healthStore.running}>
				{healthStore.running ? 'Running...' : 'Run Checks'}
			</button>
		</div>
		{#if healthStore.lastRun}
			<p class="last-run">Last run: {healthStore.lastRun}</p>
		{/if}
	</header>

	{#if healthStore.error}
		<div class="error-banner">{healthStore.error}</div>
	{/if}

	{#if healthStore.results.length > 0}
		{#if !healthStore.activeCategory}
			<div class="summary">
				<div class="stat total">{healthStore.summary.total} checks</div>
				<div class="stat pass">{healthStore.summary.pass} passed</div>
				{#if healthStore.summary.warn > 0}
					<div class="stat warn">{healthStore.summary.warn} warnings</div>
				{/if}
				{#if healthStore.summary.fail > 0}
					<div class="stat fail">{healthStore.summary.fail} failed</div>
				{/if}
			</div>
		{/if}

		{#each healthStore.filteredGroups as group (group.category)}
			<CategorySection {group} />
		{/each}
	{:else if !healthStore.running}
		<div class="empty">
			<p>Click <strong>Run Checks</strong> to start diagnostics.</p>
		</div>
	{/if}
</div>

<style>
	.dashboard {
		max-width: 720px;
		margin: 0 auto;
		padding: 32px 24px;
	}
	header {
		margin-bottom: 24px;
	}
	.title-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	h1 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 800;
	}
	.run-btn {
		padding: 8px 20px;
		border: none;
		border-radius: 6px;
		background: #2563eb;
		color: white;
		font-weight: 600;
		font-size: 0.9rem;
		cursor: pointer;
		transition: background 0.15s;
	}
	.run-btn:hover:not(:disabled) {
		background: #1d4ed8;
	}
	.run-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
	.last-run {
		margin: 6px 0 0;
		font-size: 0.8rem;
		color: var(--muted);
	}
	.error-banner {
		background: #fee2e2;
		color: #991b1b;
		padding: 12px 16px;
		border-radius: 8px;
		margin-bottom: 16px;
		font-size: 0.85rem;
	}
	.summary {
		display: flex;
		gap: 16px;
		margin-bottom: 24px;
		flex-wrap: wrap;
	}
	.stat {
		padding: 6px 14px;
		border-radius: 6px;
		font-size: 0.85rem;
		font-weight: 600;
	}
	.stat.total {
		background: #e0e7ff;
		color: #3730a3;
	}
	.stat.pass {
		background: #d1fae5;
		color: #065f46;
	}
	.stat.warn {
		background: #fef3c7;
		color: #92400e;
	}
	.stat.fail {
		background: #fee2e2;
		color: #991b1b;
	}
	.empty {
		text-align: center;
		padding: 64px 0;
		color: var(--muted);
	}
</style>
