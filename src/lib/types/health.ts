export type CheckStatus = 'pass' | 'warn' | 'fail';

export interface CheckResult {
	name: string;
	category: string;
	status: CheckStatus;
	message: string;
	details: string | null;
	timestamp: string;
}

export interface CategoryGroup {
	category: string;
	checks: CheckResult[];
	worstStatus: CheckStatus;
}

const STATUS_WEIGHT: Record<CheckStatus, number> = { pass: 0, warn: 1, fail: 2 };

export function groupByCategory(results: CheckResult[]): CategoryGroup[] {
	const map = new Map<string, CheckResult[]>();
	for (const r of results) {
		const arr = map.get(r.category) ?? [];
		arr.push(r);
		map.set(r.category, arr);
	}
	return Array.from(map.entries()).map(([category, checks]) => ({
		category,
		checks,
		worstStatus: checks.reduce<CheckStatus>(
			(worst, c) => (STATUS_WEIGHT[c.status] > STATUS_WEIGHT[worst] ? c.status : worst),
			'pass'
		)
	}));
}
