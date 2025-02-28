<script lang="ts">
	interface Props {
		digit: string;
		disabled: boolean;
		highlighted: boolean;
	}

	let { digit = '0', disabled = false, highlighted = false }: Props = $props();
	let digitIndex = $derived(parseInt(digit, 10) || 0);

	const digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

	const rowHeightRem = 2.5;
</script>

<div
	class="card {disabled ? 'opacity-40' : ''} {highlighted
		? 'border-secondary-500 shadow-secondary-500 preset-tonal-secondary'
		: 'border-surface-950 shadow-surface-950 preset-tonal-surface dark:border-surface-50 dark:shadow-surface-50'} relative flex h-10 w-8 items-center justify-center overflow-hidden border-[1px] p-2 text-center shadow-xs transition-transform duration-300"
>
	<div
		class="absolute top-0 right-0 left-0 transition-transform duration-300"
		style="transform: translateY(-{digitIndex * rowHeightRem}rem);"
	>
		{#each digits as d}
			<div class="flex h-10 items-center justify-center">
				{d}
			</div>
		{/each}
	</div>
</div>
