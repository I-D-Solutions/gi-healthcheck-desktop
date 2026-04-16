<script lang="ts">
	import type { Snippet } from 'svelte';
	import { healthStore } from '$lib/stores/health.svelte';

	let { children }: { children: Snippet } = $props();

	const statusColors: Record<string, string> = {
		pass: '#10b981',
		warn: '#f59e0b',
		fail: '#ef4444'
	};
</script>

<div class="shell">
	<aside class="sidebar">
		<div class="brand">GI Health Check</div>
		<nav>
			<button
				class="nav-item"
				class:active={healthStore.activeCategory === null}
				onclick={() => healthStore.setCategory(null)}
			>
				All Checks
				{#if healthStore.results.length > 0}
					<span class="count">{healthStore.results.length}</span>
				{/if}
			</button>

			{#if healthStore.groups.length > 0}
				<div class="nav-divider"></div>
				{#each healthStore.groups as group (group.category)}
					<button
						class="nav-item"
						class:active={healthStore.activeCategory === group.category}
						onclick={() => healthStore.setCategory(group.category)}
					>
						<span
							class="status-dot"
							style="background: {statusColors[group.worstStatus]}"
						></span>
						{group.category}
						<span class="count">{group.checks.length}</span>
					</button>
				{/each}
			{/if}
		</nav>
		<div class="version">v0.1.0</div>
	</aside>
	<main>
		{@render children()}
	</main>
</div>

<style>
	:global(*) {
		box-sizing: border-box;
		margin: 0;
		padding: 0;
	}
	:global(body) {
		font-family:
			-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial,
			sans-serif;
		background: var(--bg);
		color: var(--fg);
		--bg: #f9fafb;
		--fg: #111827;
		--muted: #6b7280;
		--surface: #ffffff;
		--surface-alt: #f3f4f6;
		--border: #e5e7eb;
		--sidebar-bg: #1e293b;
		--sidebar-fg: #e2e8f0;
	}
	.shell {
		display: flex;
		min-height: 100vh;
	}
	.sidebar {
		width: 220px;
		background: var(--sidebar-bg);
		color: var(--sidebar-fg);
		display: flex;
		flex-direction: column;
		padding: 20px 0;
		flex-shrink: 0;
	}
	.brand {
		font-weight: 800;
		font-size: 1.05rem;
		padding: 0 20px 20px;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
	}
	nav {
		flex: 1;
		padding: 16px 0;
		overflow-y: auto;
	}
	.nav-item {
		display: flex;
		align-items: center;
		gap: 8px;
		width: 100%;
		padding: 8px 20px;
		color: var(--sidebar-fg);
		text-decoration: none;
		font-size: 0.85rem;
		font-family: inherit;
		border: none;
		border-left: 3px solid transparent;
		background: none;
		cursor: pointer;
		transition: all 0.15s;
		text-align: left;
	}
	.nav-item:hover,
	.nav-item.active {
		background: rgba(255, 255, 255, 0.08);
		border-left-color: #60a5fa;
	}
	.nav-divider {
		height: 1px;
		background: rgba(255, 255, 255, 0.08);
		margin: 8px 20px;
	}
	.status-dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		flex-shrink: 0;
	}
	.count {
		margin-left: auto;
		font-size: 0.7rem;
		background: rgba(255, 255, 255, 0.12);
		padding: 1px 6px;
		border-radius: 8px;
		color: rgba(255, 255, 255, 0.5);
	}
	.version {
		padding: 12px 20px 0;
		font-size: 0.75rem;
		color: rgba(255, 255, 255, 0.4);
	}
	main {
		flex: 1;
		overflow-y: auto;
	}
</style>
