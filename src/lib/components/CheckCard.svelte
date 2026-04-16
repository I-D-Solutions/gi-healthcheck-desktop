<script lang="ts">
	import type { CheckResult } from '$lib/types/health';
	import StatusBadge from './StatusBadge.svelte';

	let { check }: { check: CheckResult } = $props();
	let expanded = $state(false);
</script>

<button class="card {check.status}" onclick={() => (expanded = !expanded)}>
	<div class="card-header">
		<span class="name">{check.name}</span>
		<StatusBadge status={check.status} />
	</div>
	<p class="message">{check.message}</p>
	{#if expanded && check.details}
		<pre class="details">{check.details}</pre>
	{/if}
</button>

<style>
	.card {
		display: block;
		width: 100%;
		text-align: left;
		background: var(--surface);
		border: 1px solid var(--border);
		border-radius: 8px;
		padding: 12px 16px;
		cursor: pointer;
		transition: box-shadow 0.15s;
		font-family: inherit;
		font-size: inherit;
	}
	.card:hover {
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
	}
	.card.fail {
		border-left: 3px solid #ef4444;
	}
	.card.warn {
		border-left: 3px solid #f59e0b;
	}
	.card.pass {
		border-left: 3px solid #10b981;
	}
	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.name {
		font-weight: 600;
		font-size: 0.95rem;
	}
	.message {
		margin: 4px 0 0;
		font-size: 0.85rem;
		color: var(--muted);
	}
	.details {
		margin: 8px 0 0;
		padding: 8px 12px;
		background: var(--surface-alt);
		border-radius: 4px;
		font-size: 0.8rem;
		white-space: pre-wrap;
		word-break: break-word;
	}
</style>
