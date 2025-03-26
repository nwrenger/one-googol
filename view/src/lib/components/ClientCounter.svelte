<script lang="ts">
	import { Tooltip } from '@skeletonlabs/skeleton-svelte';
	import type { Counter } from '../../routes/+page.svelte';

	interface Props {
		background: string;
		increase: number;
		counter: Counter;
		disabled: boolean;
	}

	let { background, increase, counter, disabled }: Props = $props();
	let open = $state(false);

	let absIncrease = $derived(Math.abs(increase));
	let exponent = $derived(Math.sqrt(counter.count.value.length) + counter.upgrade.exponent);
	let base = $derived(absIncrease * counter.upgrade.base);
	let result = $derived(Math.pow(base, exponent) * Math.sign(increase));
	let betweenResult = $derived(Math.round(result));
	let displayResult = $derived(
		betweenResult.toString().length > 4 ? betweenResult.toExponential(4) : betweenResult
	);
</script>

<Tooltip
	{open}
	onOpenChange={(e) => (open = e.open)}
	positioning={{ placement: 'top' }}
	triggerBase="card {background} {!disabled ||
		'pointer-events-none opacity-40'} relative flex h-10 w-fit min-w-8 items-center justify-center overflow-hidden border-[1px] p-2 text-center shadow-xs"
	contentBase="card {background} border-[1px] shadow-xs p-3 max-w-[calc(100vw-30px)] text-center"
	openDelay={0}
	closeDelay={50}
	closeOnClick={false}
	closeOnPointerDown={false}
	onclick={() => (open = !open)}
>
	{#snippet trigger()}
		{absIncrease}
	{/snippet}
	{#snippet content()}
		{displayResult}
	{/snippet}
</Tooltip>
