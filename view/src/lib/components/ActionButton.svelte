<script lang="ts">
	import { MousePointerClick } from 'lucide-svelte';
	import { increaseType } from '$lib';
	import type { Counter } from '../../routes/+page.svelte';
	import { Tooltip } from '@skeletonlabs/skeleton-svelte';

	interface Props {
		counter: Counter;
		disabled: boolean;
		background: string;
		onclick?: () => void;
	}

	let { counter, disabled, background, onclick = () => {} }: Props = $props();

	const duration = 500;
	let things: any[] = $state([]);
	let timeout: ReturnType<typeof setTimeout>;
	let counter_div: HTMLDivElement | undefined = $state();

	let base = $derived((counter.upgrade.level + 1) * counter.upgrade.base);
	let exp = $derived(Math.sqrt(counter.count.value.length) + counter.upgrade.exponent);
	let result = $derived(Math.pow(base, exp));
	let totalClientsResult = $derived(result * counter.count.accumulated_actions);
	let roundedResult = $derived(Math.round(result));
	let displayResult = $derived(
		roundedResult.toString().length > 4 ? roundedResult.toExponential(4) : roundedResult
	);
	let roundedTotal = $derived(Math.round(totalClientsResult));
	let displayTotal = $derived(
		roundedTotal.toString().length > 4 ? roundedTotal.toExponential(4) : roundedTotal
	);

	let open = $state(false);

	function handleClick(event: MouseEvent) {
		onclick();

		// numbers
		const target = event.currentTarget as HTMLElement;
		const { clientX, clientY } = event;
		const rect = target.getBoundingClientRect();
		const counter_rect = counter_div?.getBoundingClientRect() || new DOMRect();
		const x = clientX - rect.left - counter_rect.width / 2;
		const y = clientY - rect.top - counter_rect.height / 2;

		things = [...things, { x, y }];

		clearTimeout(timeout);
		timeout = setTimeout(() => {
			things = [];
		}, duration);
	}

	function actionLabel(): string {
		if ($increaseType == 'increment') {
			return 'Increase!';
		} else if ($increaseType == 'decrement') {
			return 'Decrease!';
		} else {
			return 'None Selected';
		}
	}

	function actionText(): string {
		if ($increaseType == 'increment') {
			return 'text-primary-950-50';
		} else if ($increaseType == 'decrement') {
			return 'text-tertiary-950-50';
		} else {
			return '';
		}
	}
</script>

<Tooltip
	{open}
	disabled={disabled || !$increaseType}
	onOpenChange={(e) => (open = e.open)}
	positioning={{ placement: 'bottom' }}
	triggerBase="relative {!disabled || 'pointer-events-none opacity-40'}"
	contentBase="card {background} border-[1px] p-3 max-w-[calc(100vw-30px)] text-center z-[999] shadow"
	openDelay={0}
	closeDelay={50}
	closeOnClick={false}
	closeOnPointerDown={false}
>
	{#snippet trigger()}
		<button
			type="button"
			onpointerdown={handleClick}
			disabled={!$increaseType}
			class="{background} btn btn-lg relative flex touch-manipulation items-center gap-2 border shadow transition-transform duration-150 enabled:hover:scale-105 enabled:active:scale-90 disabled:pointer-events-none"
			title={actionLabel()}
		>
			<MousePointerClick />
			<span class="text-base">{actionLabel()}</span>
		</button>

		<div
			bind:this={counter_div}
			class="pointer-events-none absolute w-fit text-lg whitespace-nowrap opacity-0 select-none {actionText()}"
		>
			{$increaseType === 'increment' ? '+' : '-'}{displayResult}
		</div>

		{#each things as thing (thing)}
			<div
				class="floating-text pointer-events-none w-fit text-lg whitespace-nowrap select-none {actionText()} absolute"
				style="left: {thing.x}px; top: {thing.y}px;"
			>
				{$increaseType === 'increment' ? '+' : '-'}{displayResult}
			</div>
		{/each}
	{/snippet}
	{#snippet content()}
		{displayTotal}
	{/snippet}
</Tooltip>

<style>
	@keyframes fadeOut {
		0% {
			opacity: 1;
			transform: translateY(0);
		}
		100% {
			opacity: 0;
			transform: translateY(-20px);
		}
	}

	.floating-text {
		animation: fadeOut 500ms ease-out forwards;
	}
</style>
