<script lang="ts">
	interface Props {
		digit: string;
		finished: boolean;
	}

	let { digit = '0', finished = false }: Props = $props();
	let digitIndex = $derived(parseInt(digit, 10) || 0);

	const digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

	const rowHeightRem = 2.5;
</script>

<div
	class="card {finished
		? 'opacity-40'
		: ''} border-surface-950 shadow-surface-950 dark:border-surface-50 dark:shadow-surface-50 relative flex h-10 w-8 items-center justify-center overflow-hidden border-[1px] p-2 text-center shadow-sm transition-all duration-300"
>
	<div
		class="absolute left-0 right-0 top-0 transition-transform duration-300"
		style="transform: translateY(-{digitIndex * rowHeightRem}rem);"
	>
		{#each digits as d}
			<div class="flex h-10 items-center justify-center">
				{d}
			</div>
		{/each}
	</div>
</div>
