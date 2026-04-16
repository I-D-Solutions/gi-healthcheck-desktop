<script lang="ts">
	import type { Snippet } from 'svelte';
	let { children }: { children: Snippet } = $props();
</script>

<div class="shell">
	<aside class="sidebar">
		<div class="brand">GI Health Check</div>
		<nav>
			<a href="/" class="nav-item active">Dashboard</a>
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
	}
	.nav-item {
		display: block;
		padding: 8px 20px;
		color: var(--sidebar-fg);
		text-decoration: none;
		font-size: 0.9rem;
		border-left: 3px solid transparent;
		transition: all 0.15s;
	}
	.nav-item:hover,
	.nav-item.active {
		background: rgba(255, 255, 255, 0.08);
		border-left-color: #60a5fa;
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
