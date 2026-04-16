<script lang="ts">
	import type { CheckStatus } from '$lib/types/health';

	let { status }: { status: CheckStatus } = $props();

	const labels: Record<CheckStatus, string> = {
		pass: 'PASS',
		warn: 'WARN',
		fail: 'FAIL'
	};
</script>

<span class="badge {status}">{labels[status]}</span>

<style>
	.badge {
		display: inline-block;
		padding: 2px 10px;
		border-radius: 4px;
		font-size: 0.7rem;
		font-weight: 700;
		letter-spacing: 0.05em;
		text-transform: uppercase;
	}
	.pass {
		background: #d1fae5;
		color: #065f46;
	}
	.warn {
		background: #fef3c7;
		color: #92400e;
	}
	.fail {
		background: #fee2e2;
		color: #991b1b;
	}
</style>
