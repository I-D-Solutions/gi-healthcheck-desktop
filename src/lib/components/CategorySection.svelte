<script lang="ts">
	import type { CategoryGroup } from '$lib/types/health';
	import StatusBadge from './StatusBadge.svelte';
	import CheckCard from './CheckCard.svelte';

	let { group }: { group: CategoryGroup } = $props();
</script>

<section>
	<div class="section-header">
		<h2>{group.category}</h2>
		<StatusBadge status={group.worstStatus} />
	</div>
	<div class="checks">
		{#each group.checks as check (check.name)}
			<CheckCard {check} />
		{/each}
	</div>
</section>

<style>
	section {
		margin-bottom: 24px;
	}
	.section-header {
		display: flex;
		align-items: center;
		gap: 10px;
		margin-bottom: 10px;
	}
	h2 {
		font-size: 1.05rem;
		font-weight: 700;
		margin: 0;
	}
	.checks {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}
</style>
