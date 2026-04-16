import { invoke } from '@tauri-apps/api/core';
import type { CheckResult, CategoryGroup } from '$lib/types/health';
import { groupByCategory } from '$lib/types/health';

export const healthStore = createHealthStore();

function createHealthStore() {
	let results = $state<CheckResult[]>([]);
	let running = $state(false);
	let lastRun = $state<string | null>(null);
	let error = $state<string | null>(null);
	let activeCategory = $state<string | null>(null);

	const groups = $derived(groupByCategory(results));
	const filteredGroups = $derived(
		activeCategory ? groups.filter((g) => g.category === activeCategory) : groups
	);
	const summary = $derived({
		total: results.length,
		pass: results.filter((r) => r.status === 'pass').length,
		warn: results.filter((r) => r.status === 'warn').length,
		fail: results.filter((r) => r.status === 'fail').length
	});

	async function runChecks() {
		running = true;
		error = null;
		try {
			results = await invoke<CheckResult[]>('run_health_checks');
			lastRun = new Date().toLocaleString();
		} catch (e) {
			error = String(e);
		} finally {
			running = false;
		}
	}

	function setCategory(cat: string | null) {
		activeCategory = cat;
	}

	return {
		get results() { return results; },
		get running() { return running; },
		get lastRun() { return lastRun; },
		get error() { return error; },
		get groups() { return groups; },
		get filteredGroups() { return filteredGroups; },
		get summary() { return summary; },
		get activeCategory() { return activeCategory; },
		runChecks,
		setCategory
	};
}
